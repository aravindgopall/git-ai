# 🚀 git-ai

Next-gen Git Assistant: Interactive staging. Smart diff cleaning. Terminal magic.

![GitHub](https://img.shields.io/github/license/aravindgopall/git-ai)
![GitHub last commit](https://img.shields.io/github/last-commit/aravindgopall/git-ai)
![Crates.io](https://img.shields.io/crates/v/git-ai)

---

## ✨ Features

- 🔥 Interactive hunk-by-hunk staging (y/n/Y/N/q)
- 📦 Auto-detect project type (Rust, Node.js, Python, Haskell, etc)
- 🛡️ Auto-ignore junk files like `node_modules/`, `target/`, `dist/`
- 🧹 Custom ignore patterns with `.git-ai-ignore` (also backward compatible with .gitignore)
- 📜 Smart internal pager (like `git add -p`)
- 🎨 Beautiful colored output
- ⚡ Extremely lightweight and blazing fast

---

## 📦 Install

```bash
git clone https://github.com/yourname/git-ai.git
cd git-ai
cargo install --path .
```

## Usage

```bash
# Fast staging (auto-stage clean files)

git-ai stage

# Interactive staging (review hunks one-by-one)

git-ai stage --interactive

# Smart diff review (natural language coming soon 🚀)

git-ai diff

# Web mode (coming soon 🚀)

git-ai web

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
