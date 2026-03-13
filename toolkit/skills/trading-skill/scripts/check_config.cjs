#!/usr/bin/env node
const { execSync } = require('child_process');

function check() {
  console.log("Checking KinesisRS Environment...");
  const vars = ['BSC_RPC_URL', 'SOL_RPC_URL', 'BSC_PRIVATE_KEY_1', 'SOL_PRIVATE_KEY_1'];
  vars.forEach(v => {
    if (process.env[v]) {
      console.log(`✅ ${v} is set.`);
    } else {
      console.log(`❌ ${v} is MISSING.`);
    }
  });

  try {
    const version = execSync('./target/debug/solana_claw_coin_cli --version').toString().trim();
    console.log(`✅ Binary found: ${version}`);
  } catch (e) {
    console.log(`❌ Binary not found at ./target/debug/solana_claw_coin_cli. Please run 'cargo build'.`);
  }
}

check();
