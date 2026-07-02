use std::path::{Path, PathBuf};

#[test]
fn signkey_not_referenced_outside_gen_fixture_binary() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let tests_dir = manifest_dir.join("tests");

    let gen_fixture_path = src_dir.join("bin").join("gen_fixture.rs");
    let this_guard_path = tests_dir.join("signkey_isolation.rs");

    let mut violations: Vec<PathBuf> = Vec::new();
    collect_signing_key_file_violations(&src_dir, &gen_fixture_path, &mut violations);
    collect_signing_key_file_violations(&tests_dir, &this_guard_path, &mut violations);

    assert!(
        violations.is_empty(),
        "signing key file referenced in code outside the gen_fixture binary;          runtime and test code must not load the signing key:
  {}",
        violations
            .iter()
            .map(|p| p.display().to_string())
            .collect::<Vec<_>>()
            .join("
  ")
    );
}

fn collect_signing_key_file_violations(dir: &Path, exempt: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_signing_key_file_violations(&path, exempt, out);
        } else if is_rust_source(&path) && path != exempt && references_signing_key_file(&path) {
            out.push(path);
        }
    }
}

fn is_rust_source(path: &Path) -> bool {
    path.extension().and_then(|e| e.to_str()) == Some("rs")
}

fn references_signing_key_file(path: &Path) -> bool {
    std::fs::read_to_string(path)
        .map(|s| s.contains("signkey.bin"))
        .unwrap_or(false)
}
