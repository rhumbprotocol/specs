// SPDX-License-Identifier: Apache-2.0
//
// build.rs — emit the embedded-fixture blob consumed by
// `rhumb_validate::embedded_fixtures()`.
//
// MP-0276 P-08. The lib's public contract returns `&'static [u8]` — opaque
// from the caller's perspective — so we embed the canonical fixture 
// in a small, self-describing record format and document the format in the
// lib.rs docstring. Library consumers parse the blob to recover individual
// fixtures without depending on the on-disk `fixtures/` tree.
//
// Format (little-endian, all integers unsigned):
//
//   ┌──────────────────────────────────┐
//   │  4 bytes  magic   = "RVF\0"      │
//   │  4 bytes  version = 1            │
//   │  4 bytes  entry_count            │
//   ├──────────────────────────────────┤
//   │  per entry (entry_count times):  │
//   │    4 bytes  path_len             │
//   │    N bytes  path (UTF-8, /-sep)  │
//   │    8 bytes  content_len          │
//   │    M bytes  content              │
//   └──────────────────────────────────┘
//
// Determinism: entries are sorted lexicographically by path. The blob is
// reproducible byte-for-byte from a fixed `fixtures/` tree.

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const MAGIC: &[u8; 4] = b"RVF\0";
const FORMAT_VERSION: u32 = 1;

fn main() {
    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR unset");
    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR unset");

    let fixtures_root = PathBuf::from(&manifest_dir).join("fixtures");
    if !fixtures_root.is_dir() {
        panic!(
            "fixtures/ not found at {} — embedded_fixtures requires the \
             canonical fixture set to be present at build time",
            fixtures_root.display()
        );
    }

    let mut entries: Vec<(String, Vec<u8>)> = Vec::new();
    walk(&fixtures_root, &fixtures_root, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    if entries.is_empty() {
        panic!(
            "fixtures/ at {} is empty — refusing to ship an empty embedded \
             (MP-0276 P-08 binding: embedded_fixtures must round-trip \
             the canonical fixture set)",
            fixtures_root.display()
        );
    }

    let out_path = PathBuf::from(&out_dir).join("embedded_fixtures.bin");
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(MAGIC);
    buf.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    let entry_count = u32::try_from(entries.len()).expect("entry_count overflows u32");
    buf.extend_from_slice(&entry_count.to_le_bytes());
    for (path, content) in &entries {
        let path_bytes = path.as_bytes();
        let path_len = u32::try_from(path_bytes.len()).expect("path length overflows u32");
        buf.extend_from_slice(&path_len.to_le_bytes());
        buf.extend_from_slice(path_bytes);
        let content_len = u64::try_from(content.len()).expect("content length overflows u64");
        buf.extend_from_slice(&content_len.to_le_bytes());
        buf.extend_from_slice(content);
    }

    let mut f = fs::File::create(&out_path).expect("create OUT_DIR/embedded_fixtures.bin");
    f.write_all(&buf).expect("write embedded_fixtures.bin");

    println!("cargo:rerun-if-changed=fixtures");
    println!("cargo:rerun-if-changed=build.rs");
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    let read = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(e) => panic!("read_dir({}) failed: {e}", dir.display()),
    };
    for entry in read {
        let entry = entry.unwrap_or_else(|e| panic!("dir entry in {}: {e}", dir.display()));
        let path = entry.path();
        let file_type = entry
            .file_type()
            .unwrap_or_else(|e| panic!("file_type({}): {e}", path.display()));
        if file_type.is_dir() {
            walk(root, &path, out);
        } else if file_type.is_file() {
            let rel = path
                .strip_prefix(root)
                .unwrap_or_else(|e| panic!("strip_prefix({}): {e}", path.display()));
            let mut rel_str = String::new();
            for (i, comp) in rel.components().enumerate() {
                if i > 0 {
                    rel_str.push('/');
                }
                let s = comp
                    .as_os_str()
                    .to_str()
                    .unwrap_or_else(|| panic!("non-UTF-8 fixture path: {}", path.display()));
                rel_str.push_str(s);
            }
            let bytes =
                fs::read(&path).unwrap_or_else(|e| panic!("read({}): {e}", path.display()));
            out.push((rel_str, bytes));
        }
    }
}
