use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::HashSet, env, fs, path::Path};

fn component(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && value
            .bytes()
            .all(|b| b.is_ascii_alphabetic() || b.is_ascii_digit() || b == b'-' || b == b'.')
}

fn main() {
    println!("cargo:rerun-if-changed=math-playground/catalog.json");
    let root = env::current_dir().unwrap();
    let catalog: Value =
        serde_json::from_slice(&fs::read("math-playground/catalog.json").unwrap()).unwrap();
    assert_eq!(catalog["format"], 1);
    let mut keys = HashSet::new();
    let mut generated = String::from(
        "pub(super) fn file(id: &str, version: &str, name: &str) -> Option<&'static str> { match (id, version, name) {\n",
    );
    for release in catalog["releases"].as_array().unwrap() {
        let id = release["id"].as_str().unwrap();
        let version = release["version"].as_str().unwrap();
        assert!(component(id) && component(version));
        assert!(keys.insert((id, version)));
        assert_eq!(release["visibility"], "member");
        assert!(matches!(
            release["review"].as_str(),
            Some("sol_reviewed_public_synthetic" | "developer_reviewed_public")
        ));
        let folder = root.join("math-playground/exhibits").join(id).join(version);
        let manifest_path = folder.join("manifest.json");
        let manifest: Value = serde_json::from_slice(&fs::read(&manifest_path).unwrap()).unwrap();
        assert_eq!(manifest["id"], id);
        assert_eq!(manifest["version"], version);
        assert_eq!(manifest["review"], release["review"]);
        let files = manifest["files"].as_object().unwrap();
        for required in ["engine.mjs", "renderer.mjs", "viewer.html", "tests.mjs"] {
            assert!(files.contains_key(required));
        }
        for name in files
            .keys()
            .map(String::as_str)
            .chain(std::iter::once("manifest.json"))
        {
            assert!(component(name));
            let path = folder.join(name);
            assert!(fs::symlink_metadata(&path).unwrap().is_file());
            let bytes = fs::read(&path).unwrap();
            if name != "manifest.json" {
                assert_eq!(
                    files[name].as_str().unwrap(),
                    format!("{:x}", Sha256::digest(&bytes))
                );
            }
            assert!(std::str::from_utf8(&bytes).is_ok());
            println!("cargo:rerun-if-changed={}", path.display());
            generated.push_str(&format!(
                "({id:?}, {version:?}, {name:?}) => Some(include_str!({:?})),\n",
                path.to_str().unwrap()
            ));
        }
        assert_eq!(manifest["engine_sha256"], files["engine.mjs"]);
        assert_eq!(manifest["renderer_sha256"], files["renderer.mjs"]);
    }
    generated.push_str("_ => None, } }\n");
    fs::write(
        Path::new(&env::var("OUT_DIR").unwrap()).join("curated_files.rs"),
        generated,
    )
    .unwrap();
}
