# 📜 SpellScript Grammar Specification (v0.1)

*SpellScript is a human-centric scripting language designed for spellcasting, automation, and narrative computing.*

---

## 🌟 Purpose

This document defines the formal grammar of **SpellScript**,  
a domain-specific language used in **SpellShell**, the magical command-line interface.

SpellScript aims to:
- Be readable and writable like natural language
- Map intuitive phrases to shell actions and plugins
- Serve as both scripting and spellcasting format

---

## 🧠 Basic Syntax

```spell
summon nginx silently
banish logs for 3 days
divine crash logs
```

Each spell is made up of:

```
<verb> <target> <modifier>?
```

### ✅ Examples:
- `summon nginx silently`
- `cloak api-key from output`
- `teleport repo to github`
- `divine crash logs`

---

## 📀 Grammar (PEG-like notation)

```peg
program         = _ spell+ _
spell           = command _ EOL
command         = verb _ target _ modifier*
verb            = "summon" / "banish" / "divine" / "cloak" / "teleport"
target          = WORD+
modifier        = WORD+

_               = (" " / "\t")*
EOL             = "\n" / "\r\n"
WORD            = [a-zA-Z0-9_-]+
```

🧙 Note: Future versions may support `while`, `if`, and block-based spells.

---

## 📂 Reserved Keywords

| Category    | Keywords                         |
|-------------|----------------------------------|
| Verbs       | summon, banish, cloak, divine, teleport |
| Modifiers   | silently, forcefully, for, to, from |
| Reserved    | spellbook, incantation, chant, ritual |

---

## 🧲 Comments

- SpellScript supports inline comments using `#`

```spell
# This is a deploy ritual
summon nginx
```

---

## 📜 File Format

- Spellbooks use the `.spell` extension
- One spell per line
- UTF-8 encoded

```bash
cast myproject.spell
```

---

## 🔮 Future Grammar Considerations

- `if`, `while`, `repeat` support
- Nested spells / block incantations
- JSON / TOML-based structured magic (for advanced use)