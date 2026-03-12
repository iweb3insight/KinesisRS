//! tests/cli_validation.rs

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_invalid_slippage_high() {
    let mut cmd = Command::cargo_bin("kinesis_rs").unwrap();
    cmd.arg("buy")
        .arg("0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c")
        .arg("0.1")
        .arg("--slippage")
        .arg("101");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid arguments: Slippage must be between 0 and 100"));
}

#[test]
fn test_cli_invalid_slippage_low() {
    let mut cmd = Command::cargo_bin("kinesis_rs").unwrap();
    cmd.arg("buy")
        .arg("0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c")
        .arg("0.1")
        .arg("--slippage=-1"); // Use = to pass negative value
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid arguments: Slippage must be between 0 and 100"));
}

#[test]
fn test_cli_invalid_tip_rate_high() {
    let mut cmd = Command::cargo_bin("kinesis_rs").unwrap();
    cmd.arg("buy")
        .arg("0xbb4CdB9CBd36B01bD1cBaEBF2De08d9173bc095c")
        .arg("0.1")
        .arg("--tip-rate")
        .arg("6");
    
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid arguments: Tip rate must be between 0 and 5"));
}
