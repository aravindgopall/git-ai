# 🚀 git-ai

Next-gen Git Assistant: Interactive staging. Smart diff cleaning. Terminal magic.

![GitHub](https://img.shields.io/github/license/aravindgopall/git-ai)
![GitHub last commit](https://img.shields.io/github/last-commit/aravindgopall/git-ai)
![Crates.io](https://img.shields.io/crates/v/git-ai)

---

## ✨ Core Features

- 📝 `git-ai status` → Clean, color-coded git status with hints
- 🛡️ `git-ai add` → Smarter `git add` with interactive picker
- 🔥 `git-ai stage` → Stage hunks interactively (`y/n/Y/N/q`)
- ✍️ `git-ai commit` → AI-suggested commit messages + amend/reword options
- 📥 `git-ai pull` → Smart pull with auto-stash, conflict detection, and rebase choice
- 🎒 `git-ai stash` → Interactive stash (save, list, pop, drop)
- 📦 `git-ai init` → Smart git init with project detection + optional AI magic
- 🔮 `git-ai init --magic` → Let AI generate `.gitignore`, `.git-ai-ignore`, and README
- 🌐 Multi-LLM support (OpenAI, Azure, Ollama, Claude, Gemini)

---

## 📦 Install

```bash
git clone https://github.com/yourname/git-ai.git
cd git-ai
cargo install --path .
```

## 🚀 CLI Commands

```
Command                         | Purpose
-------------------------------------------------------------------------
git-ai status                   | Enhanced git status with conflict hints
git-ai add                      | Add files interactively
git-ai stage                    | Interactive hunk staging
git-ai commit                   | AI commit messages, amend, reword
git-ai pull                     | Auto-stash, incoming commits summary
git-ai stash save/list/pop/drop | Full stash manager
git-ai init                     | Smart repo initialization
git-ai init --magic             | Full LLM magic project setup
```

🤖 AI-Powered Features

```
Feature                 | Description
-------------------------------------------------------------------------
Commit Message AI       | Generates smart, clean commit messages
Magic Init AI           | Suggests .gitignore, .git-ai-ignore, README.md
Supported LLMs          | OpenAI, Azure OpenAI, Claude, Gemini, Ollama
```

set your backend easily

```bash
export GIT_AI_LLM=openai  # or azure, ollama, claude, gemini

# further set the respective env's
```

## Example .git-ai-ignore

```bash
# Node project junk
node_modules/
dist/

# Rust project junk
target/
Cargo.lock

# Python junk
__pycache__/
*.pyc

# Local environment files
.env
*.local
```

## 🎯 Why git-ai?

- Safer staging: No accidental node_modules/ commits
- Hunk-by-hunk control: Like git add -p but smarter
- Custom ignores: Like .gitignore, but for staging
- Faster UX: Super lightweight, zero wait
- Beautiful output: Colors, paging, smooth interactions

## 🚀 Coming Soon

- 🌐 Local Web UI (git-ai web)
- 🤖 Natural language prompts (git-ai diff --prompt "ignore paths")

- ✍️ AI-generated commit messages
- 📦 GitHub Action integration
