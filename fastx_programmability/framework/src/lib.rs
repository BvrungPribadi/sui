// Copyright (c) Mysten Labs
// SPDX-License-Identifier: Apache-2.0

pub mod natives;

#[test]
fn check_that_move_code_can_be_built() {
    use move_package::BuildConfig;
    use std::path::PathBuf;

    let framework_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let build_config = BuildConfig {
        dev_mode: true,
        ..Default::default()
    };
    build_config
        .compile_package(&framework_dir, &mut Vec::new())
        .unwrap();
}