use std::process::Command;
use crate::ast::{Spell, Modifier};
use crate::commands::divine; // 👈 これを追加！

pub fn execute(spell: Spell) {
    match spell {
        Spell::Summon { target, modifiers } => {
            if target.is_empty() {
                eprintln!("💥 Summon failed: no target specified.");
                return;
            }

            let cmd = &target[0];
            let args: Vec<String> = modifiers.iter().flat_map(|m| match m {
                Modifier::Silently => vec![], // silentlyは内部処理で扱う（出力抑制など）
                Modifier::Forcefully => vec!["--force".into()],
                Modifier::For(vs) | Modifier::From(vs) | Modifier::To(vs) | Modifier::Custom(vs) => vs.clone(),
            }).collect();

            let mut command = Command::new(cmd);
            command.args(args);

            let status = command.status();

            match status {
                Ok(s) if s.success() => {
                    if !modifiers.contains(&Modifier::Silently) {
                        println!("✨ Successfully summoned `{}`", cmd);
                    }
                }
                Ok(s) => {
                    eprintln!("💥 The summoning failed (exit code {}).", s);
                }
                Err(e) => {
                    eprintln!("💥 The summoning failed: {}", e);
                }
            }
        }
        Spell::Divine { target, .. } => {
            divine::handle_divine(target); // 🧠 Divine詠唱を実行
        }
        _ => {
            eprintln!("🔮 This spell type is not yet implemented.");
        }
    }
}
