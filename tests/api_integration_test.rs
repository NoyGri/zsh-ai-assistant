use std::process::Command;
use spectral::prelude::*;

#[test]
fn given_shell_when_running_ai_lorem_ipsum_then_print_hi() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("ai")
        .arg("lorem")
        .arg("ipsum")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert_that(&stdout.to_string()).contains("hi");
}
