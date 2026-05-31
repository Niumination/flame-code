---
description: AI integration specialist — opencode.ai, gemini-cli, openrouter for Flame ADE V2
mode: subagent
model: opencode/big-pickle
temperature: 0.3
permission:
  edit: ask
  bash: ask
---

AI integration specialist for Flame ADE V2.

## Providers (3 default)
- opencode.ai
- gemini-cli
- openrouter

## Architecture
Agent with tools: read/write file, search, shell execution, directory operations.

## Security
- Approval flow for write/delete/execute operations
- Deny-list for secret paths (.env, .ssh, credentials)
- Path sanitization
