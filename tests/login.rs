#![cfg(feature = "__test_with_credentials")]

pub mod common;

use ignore::overrides::Override;
use insta::{assert_json_snapshot, assert_snapshot};

#[test]
fn atcoder() -> eyre::Result<()> {
    let (output, tree) = run("atcoder")?;
    assert_snapshot!("atcoder_output", output);
    assert_json_snapshot!("atcoder_file_tree", tree);
    Ok(())
}

fn run(platform: &str) -> eyre::Result<(String, serde_json::Value)> {
    common::run(
        |_| Ok(()),
        common::atcoder_credentials()?,
        &["", "compete", "l", platform],
        |_, output| output,
        |_| Ok(Override::empty()),
    )
}
