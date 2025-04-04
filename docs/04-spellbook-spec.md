# 📜 Spellbook File Specification (.spell)

This document defines the specification for **Spellbooks**, the `.spell` files used by **SpellShell** to execute multiple spells as a ritual or script.

Spellbooks are the magical equivalents of shell scripts, capturing multi-step operations in a readable and composable format.

---

## 🌟 Purpose

- Provide a way to define sequences of spells in a text file
- Enable reusability and repeatability of rituals (automation)
- Support human readability and Git-friendly versioning

---

## 🔍 Basic Syntax Rules

- One spell per line
- Lines are parsed using the same grammar as CLI input
- Comments begin with `#`
- File extension must be `.spell`
- UTF-8 encoded

```spell
# Spellbook for deploying web server
summon nginx silently
cloak api-key from output
divine crash logs
banish .DS_Store files
```

---

## 📖 File Format

- **Extension**: `.spell`
- **Encoding**: UTF-8
- **Structure**:
  - Header comments are allowed
  - Whitespace lines are ignored
  - Line order = execution order

---

## ⚖️ Execution Model

- Executed top-to-bottom
- Failures may stop execution (configurable via CLI flag: `--continue-on-fail`)
- Each spell is parsed and executed as if typed into `cast "..."`
- Nested spellbook calls (`invoke other.spell`) are allowed

---

## 💡 Advanced Ideas (optional/future)

- **Ritual Blocks**: grouping of related spells
- **Named Macros**: reusable spell sequences
- **Parameterization**: define input variables at runtime
- **Conditional Execution**: `if`, `when`, `repeat`

---

## 🧲 Example Spellbook

```spell
# Deploy stack
summon docker silently
invoke backend.spell
invoke frontend.spell
divine status
```

---

## 🎓 Best Practices

- Group spells logically by concern (network, env, infra...)
- Add comments for human context
- Store in `spellbooks/` directory per project
- Use consistent verbs and targets for readability

---

## 🔖 Summary

The `.spell` file is the fundamental unit of automation and ritual in SpellShell.  
It allows multi-line spell execution, sharing of common incantations, and the building of modular magical workflows.

Next up: `.spellrc` configuration spec!