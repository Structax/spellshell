# ⚙️ SpellShell Configuration Spec (.spellrc)

This document outlines the configuration system for **SpellShell**, defined via the optional `.spellrc` file.

The `.spellrc` file allows users to customize the runtime environment for spellcasting, including API keys, plugin paths, and execution settings.

---

## 🎯 Purpose

- Provide user-level customization of SpellShell behavior
- Centralize external API keys and plugin configurations
- Allow global defaults without modifying spells

---

## 🗂️ File Format

- Filename: `.spellrc`
- Location: user home directory (`~/.spellrc`) or project root
- Format: **TOML** (human-friendly and typed)
- Encoding: UTF-8

---

## 🔧 Configuration Keys

### Required Keys (if using AI):
```toml
[ai]
openai_api_key = "sk-..."
model = "gpt-4"
```

### Optional Execution Flags:
```toml
[execution]
continue_on_fail = true
verbosity = "verbose" # or "silent"
```

### Plugin Path:
```toml
[plugins]
path = "~/.spell/plugins"
```

### Environment Defaults:
```toml
[env]
default_region = "us-west-2"
timezone = "UTC"
```

---

## 🧙 Example .spellrc File
```toml
[ai]
openai_api_key = "sk-abc123"
model = "gpt-4"

[execution]
continue_on_fail = false
verbosity = "silent"

[plugins]
path = "~/.spell/plugins"

[env]
default_region = "us-east-1"
timezone = "Asia/Tokyo"
```

---

## 🧠 Behavior

- SpellShell will auto-load `.spellrc` on startup (project overrides global)
- All keys are optional, fallback to defaults if not set
- Missing or malformed `.spellrc` will **not crash** the shell

---

## 💡 Future Expansion Ideas

- Multiple profiles (e.g., `[profile.prod]`, `[profile.dev]`)
- Plugin-specific configs: `[plugin.git]` or `[plugin.aws]`
- Secrets manager integration (vault, 1Password, etc.)

---

## 🔚 Summary

The `.spellrc` file is the invisible hand guiding all spellcasting in SpellShell.  
By externalizing sensitive keys and preferences, it enables a clean separation between ritual and runtime.

Next: Poetic error UX for miscast spells!