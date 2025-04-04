# 🌿 SpellShell Taxonomy of Magic (v0.1)

This document defines the **categorization of spells** in SpellShell, providing structure and thematic consistency across the language.  
It helps developers and users understand the "schools of magic" and how to extend or interpret spell behavior.

---

## 🏛 Schools of Magic

### 1. **Conjuration**

> Create or activate something in the system

- `summon nginx`
- `summon docker silently`

### 2. **Banishing**

> Remove, clean, or destroy something

- `banish .DS_Store files`
- `banish temp logs`

### 3. **Divination**

> Ask for insight or query external intelligence (AI/API)

- `divine crash logs`
- `divine status of backend`

### 4. **Illusion**

> Output, display, or echo information

- `echo "Mana is low"`
- `channel logs to crystal`

### 5. **Transmutation**

> Transform something into another form

- `transmute png to webp`
- `convert env to json`

### 6. **Cloaking**

> Obfuscate, protect, or hide data

- `cloak api-key from output`
- `cloak logs with shadow`

### 7. **Teleportation**

> Move or sync something from one place to another

- `teleport repo to github`
- `teleport files from server`

### 8. **Invocation**

> Call another ritual or spellbook

- `invoke deploy.spell`
- `invoke frontend.spell`

---

## 💼 Use in Implementation

- Plugins should declare their spell's category for indexing and help output
- `cast --list-spells` may group by category
- Spellbooks can be organized by school (e.g., `spellbooks/transmutation/resize.spell`)

---

## 🌈 Future Domains

- **Enchantment**: Modify system state (env vars, configs)
- **Summoning (Networked)**: Call cloud services (e.g., AWS Lambda)
- **Chronomancy**: Schedule or delay spells
- **Necromancy**: Access deleted logs, backups

---

## ✨ Summary

The taxonomy gives SpellShell not just utility, but identity.  
By organizing its language around magical archetypes, we make spell creation, discovery, and documentation more intuitive and inspiring.
