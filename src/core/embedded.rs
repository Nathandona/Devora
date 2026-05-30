use crate::error::{DevoraError, Result};
use include_dir::{include_dir, Dir, DirEntry};

/// Every bundled plugin, baked into the binary at compile time.
///
/// This is what lets `devora` run after `cargo install` from any working
/// directory — there is no longer a filesystem `plugins/` lookup.
pub static PLUGINS: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/plugins");

/// Read a manifest (or any text file) from the embedded plugin tree.
/// `path` is relative to the plugins root, e.g. `"rust/manifest.toml"`.
pub fn read_text(path: &str) -> Option<&'static str> {
    PLUGINS.get_file(path).and_then(|f| f.contents_utf8())
}

/// List every file under `templates_dir` (relative to the plugins root),
/// returning each file's path *relative to that templates dir* and its bytes.
/// Paths are normalized to use `/` on every platform.
pub fn template_files(templates_dir: &str) -> Result<Vec<(String, Vec<u8>)>> {
    let templates_dir = templates_dir.replace('\\', "/");
    let dir = PLUGINS
        .get_dir(&templates_dir)
        .ok_or_else(|| DevoraError::FileSystemError {
            path: templates_dir.clone(),
            message: "Templates directory not found in embedded plugins".to_string(),
        })?;

    let prefix = format!("{}/", templates_dir);
    let mut out = Vec::new();
    collect(dir, &prefix, &mut out);
    Ok(out)
}

fn collect(dir: &Dir, prefix: &str, out: &mut Vec<(String, Vec<u8>)>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::File(f) => {
                let full = f.path().to_string_lossy().replace('\\', "/");
                if let Some(rel) = full.strip_prefix(prefix) {
                    out.push((rel.to_string(), f.contents().to_vec()));
                }
            }
            DirEntry::Dir(d) => collect(d, prefix, out),
        }
    }
}

/// Top-level language plugin ids present in the embedded tree (e.g. `rust`, `cpp`).
pub fn language_ids() -> Vec<String> {
    PLUGINS
        .dirs()
        .filter_map(|d| {
            d.path()
                .file_name()
                .and_then(|n| n.to_str())
                .map(String::from)
        })
        .collect()
}
