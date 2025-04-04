# 🌳 SpellScript AST Design (v0.1)

This document defines the internal Abstract Syntax Tree (AST) structure of **SpellScript**, the magical DSL used in **SpellShell**.

The AST serves as the intermediary between the raw text of a spell and its execution engine, enabling validation, transformation, execution, and potential AI augmentation.

---

## 📊 Goals of the AST Design

- Provide a structured representation of parsed spells
- Allow safe and testable interpretation and execution
- Support extension (e.g., conditional logic, repetition, AI hints)
- Enable serialization (for logging, AI, or external tools)

---

## 🔄 High-Level Structure

```rust
pub enum Spell {
    Summon {
        target: String,
        modifiers: Vec<Modifier>,
    },
    Banish {
        target: String,
        duration: Option<Duration>,
    },
    Cloak {
        subject: String,
        scope: Option<String>,
    },
    Divine {
        subject: String,
    },
    Teleport {
        source: String,
        destination: String,
    },
    // For script-based execution
    SpellSequence(Vec<Spell>),
}

pub enum Modifier {
    Silently,
    Forcefully,
    Verbose,
    Custom(String),
}

pub struct Duration {
    amount: u32,
    unit: TimeUnit,
}

pub enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
}
```

---

## 🌟 Notes

- **Extensibility:** Verbs are defined as enum variants to keep matching safe & explicit.
- **Custom modifiers** allow user-defined semantic extensions.
- **Duration types** are normalized for parsing `"for 3 days"` or `"for 5 mins"` into the same structure.
- **SpellSequence** allows batch spells from `.spell` files to be executed in order.

---

## 🧬 Optional AST Features (Future Support)

```rust
pub enum ConditionalSpell {
    If {
        condition: Condition,
        then_branch: Box<Spell>,
        else_branch: Option<Box<Spell>>,
    },
    While {
        condition: Condition,
        body: Box<Spell>,
    },
}

pub enum Condition {
    FileExists(String),
    EnvVarSet(String),
    Custom(String),
}
```

These features will allow flow control constructs to enter the spell world.

---

## 📚 Serialization Plan (Optional)

- Use `serde` with `#[derive(Serialize, Deserialize)]`
- Support JSON and TOML formats
- Useful for:
  - Debug logging
  - AI spell suggestion
  - Spellbook backups

---

## 🔮 Summary

This AST is designed to balance **clarity**, **expressiveness**, and **future growth**.
It is meant to be the canonical representation of spells within the SpellShell engine.

Next step: Build `parser.rs` to transform `grammar.md` syntax into this structure.



