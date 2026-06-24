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

fn markdown_files_under(relative: &str) -> Vec<PathBuf> {
    fn walk(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
        for entry in
            fs::read_dir(dir).unwrap_or_else(|_| panic!("Failed to read {}", dir.display()))
        {
            let entry = entry.expect("Failed to read directory entry");
            let path = entry.path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                out.push(path);
            }
        }
    }

    let mut files = Vec::new();
    walk(&repo_root().join(relative), &mut files);
    files
}

fn local_markdown_links(content: &str) -> Vec<String> {
    let mut links = Vec::new();
    for line in content.lines() {
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            let after_start = &rest[start + 2..];
            let Some(end) = after_start.find(')') else {
                break;
            };
            let link = &after_start[..end];
            rest = &after_start[end + 1..];

            if link.starts_with("http://")
                || link.starts_with("https://")
                || link.starts_with("mailto:")
                || link.starts_with('#')
                || link.is_empty()
            {
                continue;
            }

            links.push(link.to_string());
        }
    }
    links
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
fn docs_directory_has_one_readme_only() {
    let docs_readmes: Vec<PathBuf> = markdown_files_under("docs")
        .into_iter()
        .filter(|path| path.file_name().and_then(|name| name.to_str()) == Some("README.md"))
        .map(|path| path.strip_prefix(repo_root()).unwrap().to_path_buf())
        .collect();

    assert_eq!(
        docs_readmes,
        vec![PathBuf::from("docs/README.md")],
        "docs/ must only contain docs/README.md; nested docs README files should be descriptive pages"
    );
}

#[test]
fn docs_do_not_link_to_nested_readmes() {
    for file in markdown_files_under("docs") {
        let relative = file
            .strip_prefix(repo_root())
            .unwrap()
            .display()
            .to_string();
        let content = fs::read_to_string(&file).unwrap();
        for forbidden in [
            "commands/README.md",
            "features/README.md",
            "drivers/README.md",
            "config/README.md",
            "api/README.md",
            "hardware/README.md",
            "integration/README.md",
            "../commands/README.md",
            "../features/README.md",
            "../drivers/README.md",
            "../config/README.md",
            "../api/README.md",
            "../hardware/README.md",
            "../integration/README.md",
        ] {
            assert!(
                !content.contains(forbidden),
                "stale nested README link {forbidden} in {relative}"
            );
        }
    }
}

#[test]
fn docs_local_markdown_links_resolve() {
    for file in markdown_files_under("docs") {
        let file_dir = file.parent().unwrap();
        let relative_file = file
            .strip_prefix(repo_root())
            .unwrap()
            .display()
            .to_string();
        let content = fs::read_to_string(&file).unwrap();

        for link in local_markdown_links(&content) {
            let path_part = link.split('#').next().unwrap_or("").trim();
            if path_part.is_empty() {
                continue;
            }
            let target = file_dir.join(path_part);
            assert!(
                target.exists(),
                "broken local markdown link in {relative_file}: {link}"
            );
        }
    }
}

#[test]
fn root_readme_tracks_current_docs_surface() {
    let readme = read_repo_file("README.md");
    for expected in [
        "nvctl cuda doctor",
        "nvctl cuda ollama",
        "nvctl ai workloads",
        "docs/commands/cuda.md",
        "docs/features/cuda-ai.md",
        "docs/internals/architecture.md",
        "Drivers, DLSS, CUDA/AI, Settings",
    ] {
        assert!(
            readme.contains(expected),
            "README.md is missing current docs/feature surface: {expected}"
        );
    }
}

#[test]
fn cuda_ai_docs_include_flow_diagrams() {
    for doc in [
        "docs/commands/cuda.md",
        "docs/features/cuda-ai.md",
        "docs/internals/architecture.md",
    ] {
        let content = read_repo_file(doc);
        assert!(
            content.matches("```mermaid").count() >= 1,
            "{doc} should include at least one Mermaid diagram"
        );
    }

    let architecture = read_repo_file("docs/internals/architecture.md");
    for expected in [
        "Command Architecture",
        "Read/Write Boundary",
        "CUDA/AI Diagnostics Path",
        "TUI Data Flow",
        "Support Bundle Flow",
    ] {
        assert!(architecture.contains(expected), "missing {expected}");
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
fn release_metadata_targets_0_8_9() {
    let cargo_toml = read_repo_file("Cargo.toml");
    let root_pkgbuild = read_repo_file("PKGBUILD");
    let arch_pkgbuild = read_repo_file("release/arch/PKGBUILD");
    let fedora_spec = read_repo_file("release/fedora/nvcontrol.spec");
    let deb_changelog = read_repo_file("release/deb/changelog");
    let appimage = read_repo_file("appimage/AppImageBuilder.yml");
    let flatpak = read_repo_file("flatpak/com.github.nvcontrol.yml");

    assert!(cargo_toml.contains("version = \"0.8.9\""));
    assert!(root_pkgbuild.contains("pkgver=0.8.9"));
    assert!(arch_pkgbuild.contains("pkgver=0.8.9"));
    assert!(fedora_spec.contains("Version:        0.8.9"));
    assert!(deb_changelog.starts_with("nvcontrol (0.8.9-1)"));
    assert!(appimage.contains("version: 0.8.9"));
    assert!(flatpak.contains("tag: v0.8.9"));
}

#[test]
fn package_metadata_uses_platform_appropriate_driver_baseline() {
    let root_pkgbuild = read_repo_file("PKGBUILD");
    let arch_pkgbuild = read_repo_file("release/arch/PKGBUILD");
    let fedora_spec = read_repo_file("release/fedora/nvcontrol.spec");
    let deb_control = read_repo_file("release/deb/control");
    let pop_control = read_repo_file("release/popos-cosmic/control");
    let appimage = read_repo_file("appimage/AppImageBuilder.yml");

    assert!(root_pkgbuild.contains("nvidia-utils>=610"));
    assert!(arch_pkgbuild.contains("nvidia-utils>=610"));
    assert!(fedora_spec.contains("nvidia-driver-libs >= 610"));
    assert!(deb_control.contains("nvidia-driver-libs (>= 535)"));
    assert!(deb_control.contains("libnvidia-ml1 (>= 535)"));
    assert!(pop_control.contains("nvidia-driver-libs (>= 535)"));
    assert!(appimage.contains("nvidia-utils-610"));
}

#[test]
fn current_docs_do_not_use_removed_driver_baseline_for_supported_path() {
    for doc in [
        "README.md",
        "docs/README.md",
        "docs/commands.md",
        "docs/tui-user-guide.md",
        "docs/release-checklist.md",
        "SECURITY.md",
    ] {
        let content = read_repo_file(doc);
        assert!(
            !content.contains("open drivers 580+"),
            "stale 580 open-driver baseline in {doc}"
        );
        assert!(
            !content.contains("Open Kernel Modules 580+"),
            "stale 580 open-driver baseline in {doc}"
        );
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
