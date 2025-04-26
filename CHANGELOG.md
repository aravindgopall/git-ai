# 📋 CHANGELOG

## [0.1.0] - 2024-04-26

🔥 First Open-Source Public Release

### Added

- `git-ai status`: Clean enhanced status view with colors, hints
- `git-ai add`: Interactive file selection, safe ignoring, re-ask ignored
- `git-ai stage`: Interactive hunk staging (`y/n/Y/N/q`)
- `git-ai commit`: AI-generated commit messages, amend, reword
- `git-ai pull`: Auto-stash, rebase choice, incoming commits preview
- `git-ai stash`: Save, list, pop, drop stashes interactively
- `git-ai init`: Smart git init with project detection
- `git-ai init --magic`: LLM-powered magic project bootstrapper
- `git-ai clone`: Smart clone, SSH detect, open in editor
- Full Multi-LLM backend (OpenAI, Ollama, Azure, Claude, Gemini)
- `.git-ai` file support for project configuration
- Full safe async global backend loading

### Improved

- Professional user-friendly UX
- Fail-fast checks for missing API keys
- Beautiful colorized CLI output
- Helpful hints after every action

### Notes

- Built 100% in Rust 🦀
- Powered by Async, Tokio, OnceCell, and Clean Architecture Principles
- Future: Web UI, squash, cherry-pick, merge assistant coming 🚀
