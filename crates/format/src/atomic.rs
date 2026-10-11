//! Write a complete replacement beside the destination before replacing it. Adapted from
//! VectorCraft's MIT/Apache-2.0 atomic writer; private staging prevents exposing draft contents.

use std::fs::{File, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub fn write_atomic(path: impl AsRef<Path>, bytes: &[u8]) -> io::Result<()> {
    write_with(path.as_ref(), |file| file.write_all(bytes))
}

fn write_with(path: &Path, fill: impl FnOnce(&mut File) -> io::Result<()>) -> io::Result<()> {
    // Keep existing symlinks pointing at the same document.
    let target = match std::fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_symlink() => std::fs::canonicalize(path)?,
        Ok(_) => path.to_path_buf(),
        Err(e) if e.kind() == io::ErrorKind::NotFound => path.to_path_buf(),
        Err(e) => return Err(e),
    };
    let tmp = temp_path(&target)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.mode(0o600);
    }
    let mut file = options.open(&tmp)?;
    let filled = fill(&mut file).and_then(|()| file.sync_all());
    drop(file);
    filled.and_then(|()| keep_permissions(&target, &tmp)).and_then(|()| std::fs::rename(&tmp, &target)).inspect_err(|_| {
        // Best effort cleanup; the original destination has not been modified.
        let _ = std::fs::remove_file(&tmp);
    })
}

fn temp_path(target: &Path) -> io::Result<PathBuf> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let name = target.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "not a file path"))?;
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    Ok(target.with_file_name(format!(".{}.{}-{n}.tmp", name.to_string_lossy(), std::process::id())))
}

fn keep_permissions(target: &Path, tmp: &Path) -> io::Result<()> {
    match std::fs::metadata(target) {
        Ok(m) if m.is_file() => {
            #[cfg(unix)]
            std::fs::set_permissions(tmp, m.permissions())?;
            #[cfg(not(unix))]
            let _ = tmp;
            Ok(())
        }
        Ok(_) => Err(io::Error::new(io::ErrorKind::InvalidInput, "destination is not a regular file")),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("designcraft-atomic-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn failed_writes_preserve_the_original_and_remove_staging() {
        let dir = dir("failure");
        let path = dir.join("document.designcraft");
        std::fs::write(&path, b"original").unwrap();
        let result = write_with(&path, |file| {
            file.write_all(b"partial document")?;
            Err(io::Error::other("simulated disk full"))
        });
        assert!(result.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"original");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        write_atomic(&path, b"replacement").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"replacement");
        assert!(write_atomic(&dir, b"invalid destination").is_err());
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    #[cfg(unix)]
    fn private_staging_preserves_file_permissions_and_symlinks() {
        use std::os::unix::fs::{PermissionsExt as _, symlink};
        let dir = dir("private");
        let path = dir.join("document.designcraft");
        write_atomic(&path, b"new").unwrap();
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o600);
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640)).unwrap();
        let link = dir.join("link.designcraft");
        symlink(&path, &link).unwrap();
        write_with(&link, |file| {
            assert_eq!(file.metadata()?.permissions().mode() & 0o777, 0o600);
            assert_eq!(std::fs::read(&path)?, b"new");
            file.write_all(b"updated")
        })
        .unwrap();
        assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink());
        assert_eq!(std::fs::metadata(&path).unwrap().permissions().mode() & 0o777, 0o640);
        assert_eq!(std::fs::read(&path).unwrap(), b"updated");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
