//! Runs the built binary against a throwaway `HOME` to check that `main`
//! wires `TotalTokensConfig::show_io` into the `total_tokens` Segment. The
//! unit tests in `src/segments/total_tokens.rs` cover the formatting rules.

mod common;

use common::Sandbox;

const PAYLOAD: &str = r#"{"context_window":{"total_input_tokens":47427,"total_output_tokens":4713,"context_window_size":1048576}}"#;

const PLAIN_ARGS: [&str; 6] = [
    "--style",
    "plain",
    "--icon-set",
    "plain",
    "--theme",
    "plain",
];

fn total_tokens_segment(output: &str) -> &str {
    output
        .split("  |  ")
        .find(|segment| segment.starts_with("Total:"))
        .unwrap_or("")
}

#[test]
fn test_total_tokens_shows_io_by_default() {
    let sandbox = Sandbox::new();
    let output = sandbox.refresh(PAYLOAD, &PLAIN_ARGS);

    assert_eq!(total_tokens_segment(&output), "Total: 52k (↑47k ↓4.7k)");
}

#[test]
fn test_total_tokens_hides_io_when_disabled_in_config() {
    let sandbox = Sandbox::new();
    sandbox.write_config("[total_tokens]\nshow_io = false\n");
    let output = sandbox.refresh(PAYLOAD, &PLAIN_ARGS);

    assert_eq!(total_tokens_segment(&output), "Total: 52k");
}
