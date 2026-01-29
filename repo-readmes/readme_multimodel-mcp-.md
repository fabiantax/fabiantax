# Multi-Model MCP Server (Gemini + OpenRouter + Mistral)

MCP (Model Context Protocol) server providing access to 200+ AI models, built with Deno and Hono.

## Features

- ✅ **Google Gemini** - 11 Gemini models (latest Gemini 3)
- ✅ **OpenRouter** - 200+ models (Claude, GPT-4, Llama, etc.)
- ✅ **Mistral AI** - 13 models (Devstral 2 for coding, Pixtral for vision)
- ✅ **Free Tiers** - Many models available for free
- ✅ **MCP Protocol** - Full MCP compatibility for Claude Code
- ✅ **Dual Mode** - Stdio (local) and HTTP (deployable)
- ✅ **Deno + Hono** - Modern, secure, deployable
- ✅ **TypeScript** - Full type safety

## Quick Start

> **Note:** First time? See [INSTALL.md](./INSTALL.md) for detailed setup instructions.

### 1. Install Deno

```bash
curl -fsSL https://deno.land/install.sh | sh
```

### 2. Get API Keys

**Gemini (Optional):**
- Get free API key from [Google AI Studio](https://aistudio.google.com/app/apikey)

**OpenRouter (Optional):**
- Get free API key from [OpenRouter](https://openrouter.ai/keys)
- Provides access to Claude, GPT-4, Llama, and 200+ models

**Mistral (Optional):**
- Get free API key from [Mistral Console](https://console.mistral.ai/api-keys/)
- Devstral 2 (123B coding model) is free during preview

### 3. Set Environment Variables

```bash
# Add to your .env file (at least one required)
echo "GEMINI_API_KEY=your_gemini_key_here" >> ../.env
echo "OPENROUTER_API_KEY=your_openrouter_key_here" >> ../.env
echo "MISTRAL_API_KEY=your_mistral_key_here" >> ../.env
```

### 4. Run Locally (MCP Mode)

```bash
deno task dev
```

### 5. Or Run as HTTP Server

```bash
deno task serve
```

Visit http://localhost:3000/health to verify it's running.

## MCP Tools

### Gemini Tools

#### `gemini_generate`

Generate text using Gemini models.

**Parameters:**
- `prompt` (required): The text prompt
- `model` (optional): Model name (default: `gemini-2.0-flash-exp`)
- `temperature` (optional): 0.0-2.0 (default: 1.0)
- `maxTokens` (optional): Max output tokens (default: 8192)

**Example:**
```json
{
  "prompt": "Explain quantum computing in simple terms",
  "model": "gemini-2.0-flash-exp",
  "temperature": 0.7
}
```

### `gemini_chat`

Multi-turn conversation with context.

**Parameters:**
- `messages` (required): Array of message objects
- `model` (optional): Model name

**Example:**
```json
{
  "messages": [
    {
      "role": "user",
      "parts": [{"text": "What is MCP?"}]
    },
    {
      "role": "model",
      "parts": [{"text": "MCP stands for Model Context Protocol..."}]
    },
    {
      "role": "user",
      "parts": [{"text": "How does it work?"}]
    }
  ]
}
```

#### `gemini_models`

List available Gemini models.

---

### OpenRouter Tools

#### `openrouter_generate`

Generate text using any OpenRouter model (200+ models).

**Parameters:**
- `prompt` (required): The text prompt
- `model` (optional): Model ID (default: `anthropic/claude-3.5-sonnet:free`)
- `system` (optional): System message
- `temperature` (optional): 0.0-2.0 (default: 1.0)
- `maxTokens` (optional): Max tokens (default: 4096)

**Example:**
```json
{
  "prompt": "Explain async/await in Rust",
  "model": "anthropic/claude-3.5-sonnet:free",
  "temperature": 0.7
}
```

#### `openrouter_chat`

Multi-turn conversation with any model.

**Parameters:**
- `messages` (required): Array of message objects
- `model` (optional): Model ID
- `temperature` (optional): 0.0-2.0
- `maxTokens` (optional): Max tokens

**Example:**
```json
{
  "messages": [
    {"role": "user", "content": "What is MCP?"},
    {"role": "assistant", "content": "MCP stands for..."},
    {"role": "user", "content": "How does it work?"}
  ],
  "model": "openai/gpt-4o:free"
}
```

#### `openrouter_models`

List all available OpenRouter models.

**Parameters:**
- `filter` (optional): Filter by name (e.g., "free", "claude", "gpt")

#### `openrouter_popular`

Get curated list of popular models (free, paid, specialized).

---

### Mistral Tools

#### `mistral_generate`

Generate text using Mistral models (optimized for coding).

**Parameters:**
- `prompt` (required): The text prompt
- `model` (optional): Model name (default: `devstral-2512`)
- `system` (optional): System message
- `temperature` (optional): 0.0-2.0 (default: 0.7)
- `maxTokens` (optional): Max tokens (default: 8192)

**Example:**
```json
{
  "prompt": "Write a Rust function to parse JSON with error handling",
  "model": "devstral-2512",
  "temperature": 0.3
}
```

#### `mistral_chat`

Multi-turn conversation with Mistral models.

**Parameters:**
- `messages` (required): Array of message objects
- `model` (optional): Model name (default: `devstral-2512`)
- `temperature` (optional): 0.0-2.0 (default: 0.7)
- `maxTokens` (optional): Max tokens (default: 8192)

**Example:**
```json
{
  "messages": [
    {"role": "system", "content": "You are a helpful coding assistant."},
    {"role": "user", "content": "How do I implement a binary search tree?"},
    {"role": "assistant", "content": "Here's how..."},
    {"role": "user", "content": "Add a delete method"}
  ],
  "model": "devstral-2512"
}
```

#### `mistral_models`

List all available Mistral models organized by category.

## Available Models

### Gemini Models

> **Note:** Google's latest generation is **Gemini 3** (released Dec 2025). See [MODELS.md](./MODELS.md) for detailed model guide.

**Gemini 3 (Latest Generation - Dec 2025):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `gemini-3-flash-preview` | ⭐ Latest! Pro-level intelligence at Flash speed (recommended) | ✅ Yes |
| `gemini-3-pro-preview` | Most capable Gemini 3, frontier performance | 💰 Paid |

**Gemini 2.0 (Previous Generation - Dec 2024):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `gemini-2.0-flash-exp` | Gemini 2.0 Flash experimental | ✅ Yes |
| `gemini-2.0-flash-thinking-exp-1219` | Advanced reasoning & thinking | ✅ Yes |
| `gemini-exp-1206` | Experimental cutting edge | ✅ Yes |
| `gemini-exp-1121` | Experimental enhanced | ✅ Yes |

**Gemini 1.5 (Stable - May 2024):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `gemini-1.5-flash` | Fast and efficient, stable | ✅ Yes |
| `gemini-1.5-flash-8b` | Smaller, faster, cheaper | ✅ Yes |
| `gemini-1.5-pro` | Most capable 1.5 (2M context) | ✅ Yes (rate limited) |

**Gemini 1.0 (Legacy - Dec 2023):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `gemini-pro` | Original Gemini (legacy) | ✅ Yes |

**Total:** 11 Gemini models available

### OpenRouter Models (Popular)

| Model | Provider | Free Tier |
|-------|----------|-----------|
| `anthropic/claude-3.5-sonnet:free` | Anthropic | ✅ Yes |
| `openai/gpt-4o:free` | OpenAI | ✅ Yes |
| `google/gemini-pro-1.5:free` | Google | ✅ Yes |
| `meta-llama/llama-3.1-8b-instruct:free` | Meta | ✅ Yes |
| `meta-llama/llama-3.1-70b-instruct` | Meta | 💰 Cheap |
| `anthropic/claude-opus-4` | Anthropic | 💰 Paid |
| `openai/gpt-4o-mini` | OpenAI | 💰 Very cheap |
| `deepseek/deepseek-chat` | DeepSeek | 💰 Very cheap |

**Total:** 200+ models available through OpenRouter

See [OPENROUTER.md](./OPENROUTER.md) for complete model list and details.

### Mistral Models

**Coding Models:**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `devstral-2512` | ⭐ Devstral 2 - 123B coding model, 256K context | ✅ Yes (preview) |
| `devstral-small-2505` | Devstral Small - 24B coding model, 256K context | 💰 Paid |
| `codestral-2508` | Codestral - Code generation and completion | 💰 Paid |

**General Purpose:**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `mistral-large-2512` | Mistral Large 3 - Most capable model | 💰 Paid |
| `mistral-medium-2412` | Mistral Medium 3.1 - Balanced performance | 💰 Paid |
| `mistral-small-2501` | Mistral Small 3.2 - Fast and efficient | 💰 Paid |

**Multimodal (Vision):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `pixtral-large-2411` | Pixtral Large - Advanced vision + text | 💰 Paid |
| `pixtral-12b-2409` | Pixtral 12B - Vision model | 💰 Paid |

**Small Models (Edge):**

| Model | Description | Free Tier |
|-------|-------------|-----------|
| `ministral-3b-2512` | Ministral 3B - Edge/local deployment | 💰 Paid |
| `ministral-8b-2512` | Ministral 8B - Efficient inference | 💰 Paid |

**Open Source:**

| Model | Description | License |
|-------|-------------|---------|
| `open-mistral-nemo-2407` | Mistral Nemo 12B | Apache 2.0 |
| `open-mixtral-8x7b` | Mixtral 8x7B - MoE model | Apache 2.0 |
| `open-mixtral-8x22b` | Mixtral 8x22B - Large MoE | Apache 2.0 |

**Total:** 13 Mistral models available

See [MISTRAL.md](./MISTRAL.md) for complete model guide and best practices.

## HTTP API

When running in HTTP mode (`deno task serve`), you can use these endpoints:

### `GET /health`
Health check

### `GET /models`
List available models

### `POST /generate`
Generate text

**Body:**
```json
{
  "prompt": "Your prompt here",
  "model": "gemini-2.0-flash-exp",
  "temperature": 1.0,
  "maxTokens": 8192
}
```

### `POST /chat`
Multi-turn chat

**Body:**
```json
{
  "messages": [...],
  "model": "gemini-2.0-flash-exp"
}
```

## Deployment

### Deploy to Deno Deploy

1. Install Deno Deploy CLI:
```bash
deno install -A --global jsr:@deno/deployctl
```

2. Deploy:
```bash
deployctl deploy --project=gemini-mcp --env=GEMINI_API_KEY=your_key src/server.ts
```

### Deploy to Any Platform

The HTTP server runs on any platform that supports Deno:
- Deno Deploy
- Cloudflare Workers (with Deno compatibility)
- Railway
- Fly.io
- Any VPS with Deno installed

## Usage with Claude Code

Once configured in `.mcp.json`, Claude Code can use Gemini, OpenRouter, and Mistral:

```
User: "Ask Gemini to explain this code: [paste code]"
Claude: [Uses gemini_generate tool]

User: "Use OpenRouter's free Claude to review this"
Claude: [Uses openrouter_generate with anthropic/claude-3.5-sonnet:free]

User: "Use Mistral's Devstral 2 to refactor this function"
Claude: [Uses mistral_generate with devstral-2512]

User: "Compare responses from GPT-4 and Llama"
Claude: [Uses openrouter_generate with multiple models]
```

## Development

```bash
# Install dependencies (auto-handled by Deno)
deno cache src/index.ts

# Run in stdio mode (MCP)
deno task dev

# Run as HTTP server
deno task serve

# Format code
deno fmt

# Lint
deno lint
```

## Documentation

- **[README.md](./README.md)** - This file (overview & API reference)
- **[MODELS.md](./MODELS.md)** - Complete Gemini models guide
- **[OPENROUTER.md](./OPENROUTER.md)** - OpenRouter integration guide (200+ models)
- **[MISTRAL.md](./MISTRAL.md)** - Mistral AI integration guide (coding, vision, open source)
- **[INSTALL.md](./INSTALL.md)** - Installation instructions
- **[USAGE.md](./USAGE.md)** - Usage examples & deployment
- **[SUMMARY.md](./SUMMARY.md)** - Implementation summary

## License

MIT
