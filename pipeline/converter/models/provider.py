import os
import json
import logging
from abc import ABC, abstractmethod
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


class OpenRouterProvider(ModelProvider):
    """Provider for OpenRouter (https://openrouter.ai).

    Supports hundreds of models including free ones like:
      - qwen/qwen3-235b-a22b:free
      - deepseek/deepseek-chat-v3-0324:free
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
        "openrouter": OpenRouterProvider,
    }
    if provider not in providers:
        raise ValueError(f"Unknown provider '{provider}'. Choose from: {list(providers)}")
    logger.info(f"Using {provider} provider with model {model}")
    return providers[provider](model, api_key_env, max_tokens)
