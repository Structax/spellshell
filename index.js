#!/usr/bin/env node

import fs from "fs";
import { exec } from "child_process";
import chalk from "chalk";

const spellInput = process.argv.slice(2).join(" ").toLowerCase();
const spells = JSON.parse(fs.readFileSync("spells.json", "utf-8"));

if (!spellInput) {
  console.log(chalk.yellow("🧙‍♂️ Speak a spell, brave dev!"));
  process.exit(0);
}

const command = spells[spellInput];

if (!command) {
  console.log(chalk.red(`❌ Unknown incantation: "${spellInput}"`));
  process.exit(1);
}

console.log(chalk.cyan(`✨ Casting: ${spellInput}`));
console.log(chalk.green(`💥 Executing: ${command}`));

exec(command, (err, stdout, stderr) => {
  if (err) {
    console.error(chalk.red(`⚠️ Spell backfired:\n${stderr}`));
    return;
  }
  console.log(chalk.white(stdout));
});
