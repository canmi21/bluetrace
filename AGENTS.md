# Repository Instructions

These instructions apply to the entire repository unless a more specific `AGENTS.md` file overrides them.

## Communication

- Communicate with the user in Simplified Chinese, naturally mixing in English technical terms where appropriate.

## Written Content

- Use English (US) for all content written to files, including code, comments, documentation, configuration, and app UI copy, unless the user explicitly requests otherwise.

## Commit Rules

- Use Conventional Commit messages.
- Do not include a scope in commit messages. Use `docs: add agent instructions`, not `docs(agent): add instructions`.
- Do not add AI co-authorship or similar metadata when the assistant did not participate in writing code, when the user only asked for a simple commit, or when code changes were direct user instructions with no assistant-authored solution work.
- Add AI co-authorship metadata only when the assistant materially contributed original code or solution design beyond direct user instructions.
- When AI co-authorship metadata is appropriate, use exactly: `Co-Authored-By: Codex GPT-{VERSION} <codex@openai.com>`.

## Instruction Maintenance

- When the user explicitly states new standing rules, update this `AGENTS.md` promptly so it remains current.
