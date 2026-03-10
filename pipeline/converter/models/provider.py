import os
import json
import time
import logging
from abc import ABC, abstractmethod
from collections import deque
from dataclasses import dataclass, field
from urllib.request import Request, urlopen
from urllib.error import HTTPError

logger = logging.getLogger(__name__)


class ModelProvider(ABC):
    @abstractmethod
    def generate(self, prompt: str, system: str = "") -> str:
        pass


class ClaudeProvider(ModelProvider):
    def __init__(self, model: str, api_key_env: str, max_tokens: int = 8192):
        import anthropic

        self.client = anthropic.Anthropic(api_key=os.environ[api_key_env])
        self.model = model
        self.max_tokens = max_tokens

    def generate(self, prompt: str, system: str = "") -> str:
        kwargs = {
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": [{"role": "user", "content": prompt}],
        }
        if system:
            kwargs["system"] = system

        response = self.client.messages.create(**kwargs)
        return response.content[0].text


class GeminiProvider(ModelProvider):
    def __init__(self, model: str, api_key_env: str, max_tokens: int = 8192):
        import google.generativeai as genai

        genai.configure(api_key=os.environ[api_key_env])
        self.model = genai.GenerativeModel(model)
        self.max_tokens = max_tokens

    def generate(self, prompt: str, system: str = "") -> str:
        full_prompt = f"{system}\n\n{prompt}" if system else prompt
        response = self.model.generate_content(
            full_prompt,
            generation_config={"max_output_tokens": self.max_tokens},
        )
        return response.text


# ── Gemini Free Tier Rate Limits (as of March 2026) ────────────────────
#
# Model                  RPM   RPD     TPM
# gemini-2.5-flash-lite  15    1,000   250,000
# gemini-2.5-flash       10    250     250,000
# gemini-2.5-pro         5     100     250,000
# gemini-2.0-flash       5     ~100    250,000
#
# Strategy: rotate across models to get ~1,350 free requests/day.


def estimate_tokens(text: str) -> int:
    """Estimate token count from text. Roughly 1 token per 4 chars for code."""
    return len(text) // 4


@dataclass
class _ModelSlot:
    """Tracks rate-limit state for a single Gemini model."""
    name: str
    rpm: int
    rpd: int
    tpm: int = 250_000
    # Rolling window: (timestamp, token_count) for RPM + TPM enforcement
    recent_requests: deque = field(default_factory=deque)  # (time, tokens)
    daily_count: int = 0
    daily_reset: float = 0.0

    def _evict_old(self):
        """Remove entries older than 60 seconds from the rolling window."""
        cutoff = time.time() - 60
        while self.recent_requests and self.recent_requests[0][0] < cutoff:
            self.recent_requests.popleft()

    @property
    def daily_exhausted(self) -> bool:
        now = time.time()
        if now >= self.daily_reset:
            self.daily_count = 0
            self.daily_reset = now + 86400
        return self.daily_count >= self.rpd

    @property
    def tokens_used_this_minute(self) -> int:
        self._evict_old()
        return sum(tokens for _, tokens in self.recent_requests)

    @property
    def requests_this_minute(self) -> int:
        self._evict_old()
        return len(self.recent_requests)

    def tokens_available(self) -> int:
        """How many tokens can still be sent this minute."""
        return max(0, self.tpm - self.tokens_used_this_minute)

    def can_fit(self, estimated_tokens: int) -> bool:
        """Check if a request of this size fits within current RPM + TPM."""
        self._evict_old()
        if self.requests_this_minute >= self.rpm:
            return False
        if self.tokens_used_this_minute + estimated_tokens > self.tpm:
            return False
        return True

    def seconds_until_slot(self, estimated_tokens: int = 0) -> float:
        """Seconds to wait before a request of this token size can be sent."""
        self._evict_old()
        now = time.time()

        # Check RPM
        rpm_wait = 0.0
        if self.requests_this_minute >= self.rpm:
            rpm_wait = self.recent_requests[0][0] + 60 - now + 0.1

        # Check TPM — wait until enough tokens drain from the window
        tpm_wait = 0.0
        if estimated_tokens > 0:
            needed = self.tokens_used_this_minute + estimated_tokens - self.tpm
            if needed > 0:
                # Walk through the window to find when enough tokens expire
                cumulative = 0
                for ts, tok in self.recent_requests:
                    cumulative += tok
                    if cumulative >= needed:
                        tpm_wait = ts + 60 - now + 0.1
                        break
                else:
                    tpm_wait = 60.0  # worst case, wait a full minute

        return max(0.0, rpm_wait, tpm_wait)

    def record_request(self, token_count: int):
        self.recent_requests.append((time.time(), token_count))
        self.daily_count += 1


# Default free-tier model pool, ordered by daily quota (most generous first)
GEMINI_FREE_MODELS = [
    {"name": "gemini-2.5-flash-lite", "rpm": 15, "rpd": 1000},
    {"name": "gemini-2.5-flash",      "rpm": 10, "rpd": 250},
    {"name": "gemini-2.5-pro",        "rpm": 5,  "rpd": 100},
    {"name": "gemini-2.0-flash",      "rpm": 5,  "rpd": 100},
]


class GeminiRotatingProvider(ModelProvider):
    """Rotating Gemini provider that cycles across free-tier models.

    Maximizes throughput by:
    1. Using the model with the most remaining daily quota
    2. Respecting per-model RPM limits with automatic throttling
    3. Falling back to the next model when one is exhausted
    4. Sleeping only when ALL models are RPM-blocked (never wastes time)
    """

    def __init__(
        self,
        model: str,  # ignored — uses the full pool
        api_key_env: str,
        max_tokens: int = 8192,
        models: list[dict] | None = None,
    ):
        import google.generativeai as genai

        self.api_key = os.environ[api_key_env]
        genai.configure(api_key=self.api_key)
        self.genai = genai
        self.max_tokens = max_tokens

        pool = models or GEMINI_FREE_MODELS
        self.slots = [
            _ModelSlot(name=m["name"], rpm=m["rpm"], rpd=m["rpd"])
            for m in pool
        ]
        self._total_requests = 0
        logger.info(
            f"Gemini rotating provider initialized with {len(self.slots)} models: "
            + ", ".join(f"{s.name} ({s.rpd} RPD)" for s in self.slots)
        )

    def _pick_slot(self, estimated_tokens: int = 0) -> _ModelSlot | None:
        """Pick the best available model slot, or None if all exhausted."""
        available = [s for s in self.slots if not s.daily_exhausted]
        if not available:
            return None
        # Prefer the one with the shortest wait for this token budget
        available.sort(key=lambda s: s.seconds_until_slot(estimated_tokens))
        return available[0]

    def generate(self, prompt: str, system: str = "") -> str:
        full_prompt = f"{system}\n\n{prompt}" if system else prompt
        prompt_tokens = estimate_tokens(full_prompt)
        # Account for expected output tokens too (input + output = total TPM)
        estimated_total = prompt_tokens + self.max_tokens

        slot = self._pick_slot(estimated_total)
        if slot is None:
            exhausted = ", ".join(
                f"{s.name}: {s.daily_count}/{s.rpd}" for s in self.slots
            )
            raise RuntimeError(
                f"All Gemini free-tier models exhausted for today. "
                f"Usage: [{exhausted}]. Retry tomorrow or use a paid key."
            )

        # Throttle to respect both RPM and TPM
        wait = slot.seconds_until_slot(estimated_total)
        if wait > 0:
            logger.info(
                f"    Throttling {wait:.1f}s for {slot.name} "
                f"(~{prompt_tokens:,} input tokens, "
                f"{slot.tokens_used_this_minute:,}/{slot.tpm:,} TPM used)"
            )
            time.sleep(wait)

        logger.debug(
            f"    Sending ~{prompt_tokens:,} tokens to {slot.name} "
            f"(RPM: {slot.requests_this_minute}/{slot.rpm}, "
            f"TPM: {slot.tokens_used_this_minute:,}/{slot.tpm:,})"
        )

        model = self.genai.GenerativeModel(slot.name)

        try:
            response = model.generate_content(
                full_prompt,
                generation_config={"max_output_tokens": self.max_tokens},
            )
            # Record actual usage (estimate output tokens from response)
            output_tokens = estimate_tokens(response.text) if response.text else 0
            actual_tokens = prompt_tokens + output_tokens
            slot.record_request(actual_tokens)
            self._total_requests += 1

            if self._total_requests % 10 == 0:
                usage = ", ".join(
                    f"{s.name}: {s.daily_count}/{s.rpd}" for s in self.slots
                )
                logger.info(f"  [quota] {usage}")

            return response.text

        except Exception as e:
            err_str = str(e)
            if "429" in err_str or "quota" in err_str.lower():
                logger.warning(
                    f"    {slot.name} rate-limited ({slot.daily_count} used), "
                    f"trying next model..."
                )
                slot.daily_count = slot.rpd  # mark as exhausted
                return self.generate(prompt, system)
            raise

    def usage_summary(self) -> str:
        lines = ["Gemini free-tier usage:"]
        total_used = 0
        total_avail = 0
        for s in self.slots:
            remaining = max(0, s.rpd - s.daily_count)
            lines.append(f"  {s.name}: {s.daily_count}/{s.rpd} used, {remaining} remaining")
            total_used += s.daily_count
            total_avail += s.rpd
        lines.append(f"  Total: {total_used}/{total_avail} requests used")
        return "\n".join(lines)


class OpenRouterProvider(ModelProvider):
    """Provider for OpenRouter (https://openrouter.ai).

    Supports hundreds of models including free ones like:
      - qwen/qwen3-235b-a22b:free
      - qwen/qwen3-coder:free
      - google/gemma-3-27b-it:free
      - meta-llama/llama-4-maverick:free

    Uses the OpenAI-compatible chat completions API — no extra dependencies.
    """

    BASE_URL = "https://openrouter.ai/api/v1/chat/completions"

    def __init__(self, model: str, api_key_env: str, max_tokens: int = 8192):
        self.api_key = os.environ[api_key_env]
        self.model = model
        self.max_tokens = max_tokens

    def generate(self, prompt: str, system: str = "") -> str:
        messages = []
        if system:
            messages.append({"role": "system", "content": system})
        messages.append({"role": "user", "content": prompt})

        payload = json.dumps({
            "model": self.model,
            "max_tokens": self.max_tokens,
            "messages": messages,
        }).encode()

        req = Request(
            self.BASE_URL,
            data=payload,
            headers={
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json",
                "HTTP-Referer": "https://github.com/kathir-ks/rusty-v8",
                "X-Title": "rusty-v8 converter",
            },
            method="POST",
        )

        try:
            with urlopen(req, timeout=300) as resp:
                data = json.loads(resp.read().decode())
        except HTTPError as e:
            body = e.read().decode() if e.fp else ""
            raise RuntimeError(
                f"OpenRouter API error {e.code}: {body}"
            ) from e

        # Handle error responses
        if "error" in data:
            raise RuntimeError(f"OpenRouter error: {data['error']}")

        return data["choices"][0]["message"]["content"]


def create_provider(
    provider: str, model: str, api_key_env: str, max_tokens: int = 8192
) -> ModelProvider:
    providers = {
        "claude": ClaudeProvider,
        "gemini": GeminiProvider,
        "gemini-rotating": GeminiRotatingProvider,
        "openrouter": OpenRouterProvider,
    }
    if provider not in providers:
        raise ValueError(f"Unknown provider '{provider}'. Choose from: {list(providers)}")
    logger.info(f"Using {provider} provider with model {model}")
    return providers[provider](model, api_key_env, max_tokens)
