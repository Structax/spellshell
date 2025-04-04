# 🧙‍♀️ Contributing to SpellShell

Welcome, caster! ✨ If you're here, it means you're ready to contribute spells, improve the ritual engine, or expand the magical ecosystem.

This guide will walk you through how to contribute effectively and respectfully to the SpellShell project.

---

## 📦 Project Setup

### Requirements

- Rust (stable or nightly)
- Git
- Recommended: `cargo-watch`, `asciinema` for demos

### Clone the repository

```bash
git clone https://github.com/spellshell/spellshell.git
cd spellshell
cargo build
```

### Run the CLI

```bash
cargo run -- cast "summon nginx silently"
```

---

## ✨ Ways to Contribute

### 🪄 Add a New Spell

- Check `src/commands/` for existing verbs
- Add your own spell logic (verb handler)
- Update `docs/07-spell-taxonomy.md` if it belongs to a new school of magic

### 🧩 Create a Plugin

- See `docs/03-plugin-api.md`
- Create a `spell-*` plugin in `plugins/`
- Use stdin/stdout JSON interface or implement the Rust trait

### 📜 Improve the Language

- Propose changes to SpellScript syntax in `docs/01-grammar.md`
- Update the parser (`parser.rs`) and AST (`ast.rs`) accordingly

### 🐛 Report Bugs / Suggest Features

- Open issues using templates
- Include reproducible spells and expected outcome

### 🧪 Add Examples

- Add `.spell` files to `examples/`
- Help build a library of reusable incantations

---

## 🔮 Guidelines

- Keep spell names poetic but predictable
- Match the tone of error messages and comments to the existing lore
- Maintain formatting and code style (`rustfmt`, `clippy`)
- Write descriptive commit messages (e.g., `feat: add banish logs command`)
- Keep PRs small and focused

---

## 🧙 Code of Conduct

We follow the [Contributor Covenant](https://www.contributor-covenant.org/).  
All contributors are expected to be kind, inclusive, and respectful.

---

## 🧵 Communication

- Join our (coming soon) Discord: **The Guild of Casters**
- Discussions on GitHub are welcome

---

Thank you for being part of this magical project. May your spells never miscast! 🪄
