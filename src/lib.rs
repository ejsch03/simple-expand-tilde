use std::path::*;

/// Expand the tilde (`~`) from within the provided path.
pub fn expand_tilde(path: impl AsRef<Path>) -> Option<PathBuf> {
    let p = path.as_ref();

    if p.starts_with("~") {
        let mut base = std::env::home_dir()?;
        base.extend(p.components().skip(1));
        Some(base)
    } else {
        Some(p.to_path_buf())
    }
}
