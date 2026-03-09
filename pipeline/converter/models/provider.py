import os
import logging
from abc import ABC, abstractmethod

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


def create_provider(
    provider: str, model: str, api_key_env: str, max_tokens: int = 8192
) -> ModelProvider:
    providers = {
        "claude": ClaudeProvider,
        "gemini": GeminiProvider,
    }
    if provider not in providers:
        raise ValueError(f"Unknown provider '{provider}'. Choose from: {list(providers)}")
    logger.info(f"Using {provider} provider with model {model}")
    return providers[provider](model, api_key_env, max_tokens)
