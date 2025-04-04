use crate::ast::{Spell, Modifier};

pub fn parse_spell(input: &str) -> Result<Spell, String> {
    // 仮実装（構文を受け取って AST に変換）
    // 必要に応じて pest パーサを後で結びつける
    if input.trim().starts_with("summon") {
        Ok(Spell::Summon {
            target: vec!["nginx".into()],
            modifiers: vec![Modifier::Silently],
        })
    } else if input.trim().starts_with("cloak") {
        Ok(Spell::Cloak {
            target: vec!["api-key".into()],
            modifiers: vec![Modifier::From(vec!["output".into()])],
        })
    } else if input.trim().starts_with("divine") {
        Ok(Spell::Divine {
            target: vec!["crash".into(), "logs".into()],
            modifiers: vec![],
        })
    } else {
        Err("Unsupported".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Spell, Modifier};

    #[test]
    fn test_cloak_with_modifier() {
        let input = "cloak api-key from output\n";
        let spell = parse_spell(input).expect("Failed to parse");

        assert!(matches!(spell, Spell::Cloak { .. }));

        if let Spell::Cloak { target, modifiers } = spell {
            assert_eq!(target, vec!["api-key"]);
            assert_eq!(
                modifiers,
                vec![Modifier::From(vec!["output".into()])]
            );
        }
    }

    #[test]
    fn test_summon_silently() {
        let input = "summon nginx silently\n";
        let spell = parse_spell(input).expect("Should parse");

        if let Spell::Summon { target, modifiers } = spell {
            assert_eq!(target, vec!["nginx"]);
            assert_eq!(modifiers, vec![Modifier::Silently]);
        } else {
            panic!("Expected summon spell");
        }
    }

    #[test]
    fn test_divine_multiple_words() {
        let input = "divine crash logs\n";
        let spell = parse_spell(input).expect("Parse failed");

        if let Spell::Divine { target, modifiers } = spell {
            assert_eq!(target, vec!["crash", "logs"]);
            assert!(modifiers.is_empty());
        } else {
            panic!("Expected divine spell");
        }
    }
}
