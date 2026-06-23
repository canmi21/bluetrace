# Repository Instructions

These instructions apply to the entire repository unless a more specific `AGENTS.md` file overrides them.

## Communication

- Communicate with the user in Simplified Chinese, naturally mixing in English technical terms where appropriate.

## Written Content

- Use English (US) for all content written to files, including code, comments, documentation, configuration, and app UI copy, unless the user explicitly requests otherwise.
- Use lowercase English source filenames. Keep required toolchain, manifest, and generated platform filenames when the ecosystem requires them.
- Do not create `readme.md` or `README.md` unless the user explicitly requests it.
- Use TypeScript for the frontend and Rust for the Tauri backend unless the user explicitly requests a different stack.
- Use `pnpm` for JavaScript package management unless the user explicitly requests another package manager.
- Pin the bluetrace Vite development server to `127.0.0.1:47631` with strict port handling; do not allow automatic fallback to another port.
- Keep app icon assets limited to variants generated from the user-provided source icon. Do not keep default Vite, Svelte, Tauri, or favicon image assets.
- For flattened 1024x1024 Icon Composer source PNGs used with Tauri icon generation, first scale the artwork to 824x824 and center it in a transparent 1024x1024 canvas, leaving 100px transparent padding on each side before running `pnpm tauri icon`.
- Keep code and file names lowercase, but use `Bluetrace` only for user-facing product names and window titles when the user wants a capitalized app name.

## Commit Rules

- Use Conventional Commit messages.
- Do not include a scope in commit messages. Use `docs: add agent instructions`, not `docs(agent): add instructions`.
- Do not add AI co-authorship or similar metadata when the assistant did not participate in writing code, when the user only asked for a simple commit, or when code changes were direct user instructions with no assistant-authored solution work.
- Add AI co-authorship metadata only when the assistant materially contributed original code or solution design beyond direct user instructions.
- When AI co-authorship metadata is appropriate, use exactly: `Co-Authored-By: Codex GPT-{VERSION} <codex@openai.com>`.

## Instruction Maintenance

- When the user explicitly states new standing rules, update this `AGENTS.md` promptly so it remains current.
