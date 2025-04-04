# 🔌 SpellShell Plugin API Specification (v0.1)

This document defines the **Plugin API** for SpellShell, enabling developers to extend the system by adding new spells.

Plugins allow external tools, commands, or even other programming languages to integrate seamlessly into the spellcasting ecosystem.

---

## 🚀 Goals

- Allow users to add custom spells beyond built-in verbs
- Support both dynamic and subprocess-based plugins
- Provide a consistent API surface for extensions
- Enable safe spell execution in isolated environments

---

## 🔧 Plugin Types

### 1. **Trait-Based Plugins (Rust-native)**

Ideal for performance and type-safety. Plugins are compiled together or loaded as dynamic libraries (optional future feature).

```rust
pub trait SpellPlugin {
    fn incant(&self, spell: Spell) -> Result<(), SpellError>;
    fn can_handle(&self, verb: &str) -> bool;
    fn description(&self) -> &'static str;
}
```

Registered plugins are loaded into a registry:

```rust
let plugins: Vec<Box<dyn SpellPlugin>> = vec![
    Box::new(GitSpell {}),
    Box::new(AwsSpell {}),
];
```

### 2. **Subprocess-Based Plugins (language-agnostic)**

Plugins are invoked via CLI commands and communicate via JSON over stdin/stdout.

#### Example Spell:

```spell
cast "teleport repo to github"
```

#### Expected Subprocess Interface:

```bash
$ spell-git --handle "teleport repo to github"
```

**Input (stdin):**

```json
{
  "verb": "teleport",
  "target": "repo",
  "modifiers": ["to", "github"]
}
```

**Output (stdout):**

```json
{
  "status": "success",
  "message": "Pushed repo to GitHub."
}
```

---

## 🛋️ Plugin Discovery

- SpellShell will scan `~/.spell/plugins/` for binaries named `spell-*`
- Plugins must be executable and follow the JSON interface
- Trait-based plugins must be registered manually in code (for now)

---

## 🔮 Error Handling

Plugins must return well-formed error messages on failure:

```json
{
  "status": "error",
  "message": "Authentication failed."
}
```

SpellShell will display these as poetic incantation failures:

```text
💥 The spell backfired: Authentication failed.
```

---

## 🎓 Best Practices

- Use consistent spell phrasing: `verb target [modifiers...]`
- Avoid ambiguity in modifiers (e.g. `to`, `from`, `with`)
- Ensure JSON schema matches the expected AST shape
- Consider implementing both plugin types if flexibility is needed

---

## 🌟 Future Considerations

- Plugin metadata registry (`spell-plugin.toml`)
- Auto-discovery of capabilities
- Plugin sandboxing / permission controls
- WASM-based portable plugins

---

## 🔖 Summary

This API defines the foundation for SpellShell's modular spell system. With it, the community can:

- Add powerful integrations (git, docker, aws...)
- Define entire new schools of magic (spell domains)
- Share reusable spells through `spellbook.dev`

Next up: Create the first official plugin — `spell-git`!
