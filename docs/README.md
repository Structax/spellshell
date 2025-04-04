# 🪄 SpellShell — The Language of Ritual

*Turn your terminal into a tome. Cast commands as incantations. Write scripts like spells.*

---

## ✨ Why SpellShell Exists

Because the terminal never changed.  
Because `-g daemon off` is not poetry.  
Because automation deserves imagination.

**SpellShell is not a wrapper. It is a ritual layer.**

It lets you:
- `summon nginx silently`
- `banish .DS_Store files`
- `divine crash logs`
- `invoke deploy.spell`

These are not commands. They are **spells**.

---

## 🧠 What Is SpellShell?

SpellShell is a programmable, extensible CLI designed to:
- Replace traditional shell scripting with natural-language inspired incantations
- Extend CLI behavior through plugins and GPT-powered spells
- Encourage joy, creativity, and expressiveness in daily automation

Under the hood:
- A custom PEG parser interprets **SpellScript**
- Spells are compiled into actions, or dispatched to plugins
- `divine` spells call LLMs like GPT for analysis or generation

---

## 📜 SpellScript Basics

```spell
summon nginx silently
cloak api-key from output
divine crash logs
```

🪶 Each spell follows this pattern:
```
<verb> <target> <modifier>*
```

View the [SpellScript Grammar](docs/01-grammar.md) →

---

## 📁 Spellbooks

`.spell` files are magical scripts — spellbooks. Run them with:
```bash
cast deploy.spell
```

Example:
```spell
# Deploy web stack
summon docker
invoke backend.spell
divine status
```

View the [Spellbook Spec](docs/04-spellbook-spec.md) →

---

## 🔌 Plugin Architecture

Extend SpellShell with custom plugins:
- Trait-based in Rust (for performance)
- Subprocess-based in any language (via stdin/stdout JSON)

SpellShell auto-discovers plugins in `~/.spell/plugins/`

View the [Plugin API Spec](docs/03-plugin-api.md) →

---

## 🧙 Examples of Magic

```bash
cast "summon nginx silently"
cast "banish .DS_Store files"
cast "divine crash logs"
cast deploy.spell
```

Each line is not just a command — it’s a piece of a personal language.

---

## ⚙️ Configuration (.spellrc)

Define API keys, plugin paths, execution modes:
```toml
[ai]
openai_api_key = "sk-..."
model = "gpt-4"
```

View the [Config Spec](docs/05-config-spec.md) →

---

## 💥 When Spells Fail

SpellShell turns errors into poetic feedback:
```text
💥 The summoning failed. 'nginx' resisted your call.
🌫️ The divination clouded. The Oracle could not be reached.
```

See [Poetic Errors](docs/06-error-messages.md) →

---

## 🧙‍♀️ Join the Ritual

This is not just a CLI. It’s a cult of clarity. A fellowship of function. A spellbook that writes back.

→ Coming soon: [spellbook.dev](https://spellbook.dev)  
→ Join the Guild (Discord coming soon)

---

## 🛠️ Install (Coming Soon)

```bash
# Rust version (WIP)
cargo install spellshell
```

For now, explore the docs and contribute to the language.

---

## 📚 Documentation Index

- [Grammar Spec](docs/01-grammar.md)
- [AST Design](docs/02-ast-design.md)
- [Plugin API](docs/03-plugin-api.md)
- [Spellbook Format](docs/04-spellbook-spec.md)
- [Configuration File](docs/05-config-spec.md)
- [Poetic Errors](docs/06-error-messages.md)
- [Spell Taxonomy](docs/07-spell-taxonomy.md)
