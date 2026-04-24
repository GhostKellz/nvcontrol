use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_repo_file(relative: &str) -> String {
    fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|_| panic!("Failed to read {}", relative))
}

fn assert_exists(relative: &str) {
    assert!(
        repo_root().join(relative).exists(),
        "Missing expected path: {relative}"
    );
}

#[test]
fn release_and_packaging_assets_exist() {
    for path in [
        "assets/nvcontrol.desktop",
        "assets/icons/icon-256x256.png",
        "release/arch/nvcontrol-monitor.service",
        "release/arch/nvcontrol-game-profile-auto.service",
        "release/install-system.sh",
        "packaging/nvcontrol.desktop",
        "completions/nvctl.bash",
        "completions/_nvctl",
        "completions/nvctl.fish",
    ] {
        assert_exists(path);
    }
}

#[test]
fn rust_version_is_consistent_across_core_release_metadata() {
    let cargo_toml = read_repo_file("Cargo.toml");
    let toolchain = read_repo_file("rust-toolchain.toml");
    let ci_doc = read_repo_file("CI.md");
    let building_doc = read_repo_file("docs/building.md");
    let fedora_spec = read_repo_file("release/fedora/nvcontrol.spec");
    let deb_control = read_repo_file("release/deb/control");
    let pop_control = read_repo_file("release/popos-cosmic/control");

    assert!(cargo_toml.contains("rust-version = \"1.95\""));
    assert!(toolchain.contains("channel = \"1.95.0\""));
    assert!(ci_doc.contains("Rust 1.95 stable toolchain"));
    assert!(building_doc.contains("Rust 1.95+"));
    assert!(fedora_spec.contains("BuildRequires:  rust >= 1.95"));
    assert!(deb_control.contains("rustc (>= 1.95)"));
    assert!(pop_control.contains("rustc (>= 1.95)"));
}

#[test]
fn packaging_references_current_service_name() {
    let root_pkgbuild = read_repo_file("PKGBUILD");
    let arch_pkgbuild = read_repo_file("release/arch/PKGBUILD");
    let deb_rules = read_repo_file("release/deb/rules");
    let fedora_spec = read_repo_file("release/fedora/nvcontrol.spec");

    for content in [&root_pkgbuild, &arch_pkgbuild, &deb_rules, &fedora_spec] {
        assert!(content.contains("nvcontrol-game-profile-auto"));
        assert!(!content.contains("nvcontrol-gamedetect"));
    }
}

#[test]
fn installer_prefers_full_nvcontrol_release_asset() {
    let installer = read_repo_file("release/install-system.sh");
    let nvcontrol_pos = installer
        .find("name.startswith('nvcontrol-')")
        .expect("missing nvcontrol asset preference");
    let nvctl_pos = installer
        .find("name.startswith('nvctl-')")
        .expect("missing nvctl fallback preference");
    assert!(
        nvcontrol_pos < nvctl_pos,
        "installer should prefer nvcontrol asset before nvctl fallback"
    );
}

#[test]
fn workflow_files_no_longer_assume_old_runner_home() {
    for workflow in [
        ".github/workflows/ci.yml",
        ".github/workflows/nightly.yml",
        ".github/workflows/release.yml",
    ] {
        let content = read_repo_file(workflow);
        assert!(
            !content.contains("/home/runner"),
            "stale runner path in {workflow}"
        );
        assert!(
            !content.contains("nv-palladium"),
            "stale runner label in {workflow}"
        );
    }
}

#[test]
fn docs_do_not_reference_removed_service_or_completion_path() {
    for doc in [
        "README.md",
        "docs/README.md",
        "docs/commands.md",
        "CHANGELOG.md",
        "CI.md",
        "SECURITY.md",
    ] {
        let content = read_repo_file(doc);
        assert!(
            !content.contains("nvcontrol-gamedetect"),
            "stale service name in {doc}"
        );
        assert!(
            !content.contains("drivers generate-completions"),
            "stale completion path in {doc}"
        );
    }
}
