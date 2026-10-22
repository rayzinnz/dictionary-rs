use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=assets/thesaurus.db");

    let profile = std::env::var("PROFILE").unwrap();

    let target_dir = Path::new("target").join(profile);

    fs::copy(
        "assets/thesaurus.db",
        target_dir.join("thesaurus.db"),
    ).unwrap();
}