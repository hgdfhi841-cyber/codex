# codex-ar — نسخة عربية غير رسمية من Codex CLI

Unofficial Arabic localization of the open-source [OpenAI Codex CLI](https://github.com/openai/codex) (Apache-2.0).
Not produced, endorsed, or supported by OpenAI.

## What changed

- `codex-rs/core/src/config/mod.rs`: built-in instruction so the model always answers in Arabic.
- `codex-rs/tui/src/**`, `codex-rs/tui/assets/tooltips.txt`: user-interface strings translated to Arabic (in progress).
- `.github/workflows/codex-ar-windows.yml`: builds a Windows x64 zip and publishes it under Releases. Upstream CI workflows were removed from this fork.
- `codex-ar/`: launcher, installer script, and Arabic readme shipped in the zip.

Modified source files carry a one-line notice at the top.
