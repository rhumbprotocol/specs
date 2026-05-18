// SPDX-License-Identifier: Apache-2.0
//
// Shared directory walker for the per-category validators.
//
// Background: P-02 (schema), P-03 (template), and P-04 (workflow) each
// shipped a private recursive walker with the same shape: skip symlinks,
// skip hidden directories, recurse into non-hidden directories, hand
// each candidate path to a per-validator predicate. P-05 (adapter)
// would have been the fourth duplicate; per RULE-CORE-04 the third use
// is the documented factoring point. P-04 deliberately punted the
// factoring to P-05 so the workflow validator could land inside its
// token budget. This module satisfies that carry-forward debt.
//
// Behavior:
//   - Symlinks are not followed (anywhere — at the target, in subdirs,
//     or as targeted entries). Following symlinks risks loops and turns
//     deterministic walks non-deterministic.
//   - Hidden directories (basename starts with `.`) are skipped — they
//     are typically VCS metadata (`.git`), build artifacts (`.cache`,
//     `.target`), or runtime state. Hidden FILES are not skipped (the
//     validators don't currently target dotfiles, but the walker
//     leaves the policy to the caller's `on_file` predicate).
//   - Nonexistent or non-directory targets short-circuit cleanly.
//   - If `target` is itself a regular file, `on_file` is invoked once
//     with that path. (Schema/template validators relied on this so
//     `--target some/single.json` works.)
//
// API:
//   `on_file(&Path)` is invoked for every regular file the walker
//   reaches.
//   `on_dir(&Path) -> bool` is invoked for every directory the walker
//   enters (INCLUDING the initial `target` if it's a directory). If
//   it returns `true`, descent stops at that directory — its contents
//   are not enumerated. This implements the "workflow root" /
//   "adapter root" stop-on-find pattern: the caller has matched the
//   directory itself and should not also re-classify its children.
//   For walkers that only match files, pass `|_| false`.
//
// The two callbacks are separate (rather than a single
// `visit(&Path, FileType)` callback) because every existing caller
// has clearly-separable file-vs-dir logic. Splitting them avoids
// type-tagging or pattern matches inside the closure body.

use std::fs;
use std::path::Path;

/// Walk `target`, invoking `on_file` for each regular file and
/// `on_dir` for each directory entered. See module docs for the full
/// contract. Symlinks are skipped; hidden directories are skipped;
/// directories where `on_dir` returns `true` are claimed and not
/// descended into.
pub(crate) fn walk_dir<OF, OD>(
    target: &Path,
    on_file: &mut OF,
    on_dir: &mut OD,
) -> std::io::Result<()>
where
    OF: FnMut(&Path),
    OD: FnMut(&Path) -> bool,
{
    if !target.exists() {
        return Ok(());
    }
    let meta = fs::symlink_metadata(target)?;
    if meta.file_type().is_symlink() {
        return Ok(());
    }
    if meta.is_file() {
        on_file(target);
        return Ok(());
    }
    if !meta.is_dir() {
        return Ok(());
    }
    walk_dir_inner(target, on_file, on_dir)
}

fn walk_dir_inner<OF, OD>(
    dir: &Path,
    on_file: &mut OF,
    on_dir: &mut OD,
) -> std::io::Result<()>
where
    OF: FnMut(&Path),
    OD: FnMut(&Path) -> bool,
{
    if on_dir(dir) {
        return Ok(());
    }
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with('.'))
            {
                continue;
            }
            walk_dir_inner(&path, on_file, on_dir)?;
        } else if file_type.is_file() {
            on_file(&path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    /// Allocate a fresh per-test directory under the OS temp dir. Avoids
    /// pulling in `tempfile` for one helper. Cleaned up by Drop.
    struct TempTree(PathBuf);

    impl TempTree {
        fn new(tag: &str) -> std::io::Result<Self> {
            let mut p = std::env::temp_dir();
            let unique = format!(
                "rhumb-validate-walk-{}-{}-{}",
                tag,
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_nanos())
                    .unwrap_or(0)
            );
            p.push(unique);
            fs::create_dir_all(&p)?;
            Ok(Self(p))
        }
    }

    impl Drop for TempTree {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn collect_files(target: &Path) -> std::io::Result<Vec<PathBuf>> {
        let mut out = Vec::new();
        walk_dir(target, &mut |p| out.push(p.to_path_buf()), &mut |_| false)?;
        out.sort();
        Ok(out)
    }

    #[test]
    fn walk_returns_ok_for_nonexistent_target() -> TestResult {
        let target = Path::new("/this/path/should/not/exist/rhumb-walk-empty");
        let files = collect_files(target)?;
        assert!(files.is_empty());
        Ok(())
    }

    #[test]
    fn walk_visits_single_file_target() -> TestResult {
        let tree = TempTree::new("single-file")?;
        let f = tree.0.join("only.txt");
        fs::write(&f, b"x")?;
        let files = collect_files(&f)?;
        assert_eq!(files, vec![f]);
        Ok(())
    }

    #[test]
    fn walk_recurses_into_subdirectories() -> TestResult {
        let tree = TempTree::new("recurse")?;
        let sub = tree.0.join("a/b");
        fs::create_dir_all(&sub)?;
        let f1 = tree.0.join("top.txt");
        let f2 = sub.join("nested.txt");
        fs::write(&f1, b"x")?;
        fs::write(&f2, b"y")?;

        let files = collect_files(&tree.0)?;
        assert_eq!(files, vec![f2, f1]); // sorted: a/b/nested.txt < top.txt
        Ok(())
    }

    #[test]
    fn walk_skips_hidden_directories() -> TestResult {
        let tree = TempTree::new("hidden")?;
        fs::create_dir_all(tree.0.join(".cache"))?;
        fs::write(tree.0.join(".cache/should-not-appear.txt"), b"x")?;
        fs::write(tree.0.join("visible.txt"), b"y")?;

        let files = collect_files(&tree.0)?;
        assert_eq!(files, vec![tree.0.join("visible.txt")]);
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn walk_skips_symlinked_files_and_dirs() -> TestResult {
        use std::os::unix::fs::symlink;

        let tree = TempTree::new("symlink")?;
        let real_dir = tree.0.join("real");
        fs::create_dir_all(&real_dir)?;
        let real_file = tree.0.join("real.txt");
        fs::write(&real_file, b"x")?;
        let inside = real_dir.join("inner.txt");
        fs::write(&inside, b"y")?;

        let link_to_file = tree.0.join("link-file.txt");
        let link_to_dir = tree.0.join("link-dir");
        symlink(&real_file, &link_to_file)?;
        symlink(&real_dir, &link_to_dir)?;

        let files = collect_files(&tree.0)?;
        // Should see real.txt and real/inner.txt, NOT the symlinks.
        assert!(files.contains(&real_file));
        assert!(files.contains(&inside));
        assert!(!files.contains(&link_to_file));
        for f in &files {
            assert!(!f.starts_with(&link_to_dir));
        }
        Ok(())
    }

    #[test]
    fn walk_stops_descending_when_on_dir_returns_true() -> TestResult {
        let tree = TempTree::new("stop")?;
        let claim = tree.0.join("root");
        let inner = claim.join("inner");
        fs::create_dir_all(&inner)?;
        fs::write(claim.join("MARKER"), b"x")?;
        fs::write(claim.join("contents.txt"), b"y")?;
        fs::write(inner.join("nested.txt"), b"z")?;

        let mut files = Vec::new();
        let mut claimed = Vec::new();
        walk_dir(
            &tree.0,
            &mut |p| files.push(p.to_path_buf()),
            &mut |d| {
                if d.join("MARKER").is_file() {
                    claimed.push(d.to_path_buf());
                    true
                } else {
                    false
                }
            },
        )?;

        // The walker entered tree.0 (no MARKER → recurse), then `claim`
        // (has MARKER → stop). Neither claim's contents nor its inner
        // subdir should be visited.
        assert_eq!(claimed, vec![claim.clone()]);
        assert!(files.is_empty(), "expected no files; got {files:?}");
        Ok(())
    }

    #[test]
    fn walk_invokes_on_dir_for_every_directory_entered() -> TestResult {
        let tree = TempTree::new("on-dir")?;
        let a = tree.0.join("a");
        let b = a.join("b");
        fs::create_dir_all(&b)?;

        let mut visited = Vec::new();
        walk_dir(
            &tree.0,
            &mut |_| {},
            &mut |d| {
                visited.push(d.to_path_buf());
                false
            },
        )?;
        visited.sort();
        let expected: Vec<PathBuf> = {
            let mut v = vec![tree.0.clone(), a, b];
            v.sort();
            v
        };
        assert_eq!(visited, expected);
        Ok(())
    }
}
