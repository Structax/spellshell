# 💥 SpellShell Poetic Error Messages Spec

This document defines the UX specification for **SpellShell's spell failure messages**.  
Rather than showing raw errors, SpellShell delivers **poetic, magical, and user-friendly messages** that align with its narrative fantasy design.

---

## 🫠 Philosophy

- CLI should not be hostile — even in failure, it should inspire.
- Errors are part of the spellcasting narrative.
- Maintain immersion while providing actionable feedback.

---

## ❌ Error Categories & Example Messages

### 1. **Command Execution Failure**
> Shell command could not be executed
```text
⚡ The summoning failed. 'nginx' resisted your call.
```

### 2. **Unknown Verb / Parsing Error**
> Invalid syntax or unrecognized structure
```text
🔥 Miscast! I know not the words you uttered: 'evoke'.
```

### 3. **Missing Target**
> Verb requires a target, but none was given
```text
💀 The spell fizzled. No entity to summon.
```

### 4. **Plugin Not Found**
> Plugin binary not found
```text
🌀 You called upon a forgotten magic: 'spell-git' is not installed.
```

### 5. **API / GPT Failure**
> GPT API error, missing key, network failure
```text
💫 The divination clouded. The Oracle could not be reached.
```

### 6. **File Not Found / IO Error**
```text
🌪 No scroll found: 'myproject.spell' is missing from this realm.
```

### 7. **Invalid Modifier**
```text
🦄 The spell twisted unnaturally. 'furiously' is not a known modifier.
```

---

## 🔧 Implementation Suggestions

- Match errors to categories based on parser/executor/IO layer
- Include original system error in verbose mode (optional)
- Use emoji + literary tone to enhance clarity & joy

---

## 💭 Customization (future)

- Allow overriding message templates in `.spellrc`
- Support localization (e.g., `messages.ja.toml`)
- Plugin developers can define their own failure poetry

---

## 🌖 Summary

Errors are not just messages — they're part of the magic.  
SpellShell turns failure into feedback, and bugs into bardic tales.

May every miscast lead to greater mastery. ✨