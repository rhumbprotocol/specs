#!/usr/bin/env node
/**
 * Sync the RWP version from the VERSION file into every file that references it.
 *
 * Usage:
 *   node scripts/sync-version.mjs --from 0.26.0
 *
 * Reads the target version from ./VERSION, replaces every occurrence of the
 * old version (--from) across the protocol repo, website, and Rust crates.
 */
import { readFileSync, writeFileSync, readdirSync, statSync } from 'node:fs';
import { join, extname, relative } from 'node:path';
import { parseArgs } from 'node:util';

const root = new URL('..', import.meta.url).pathname;
const version = readFileSync(join(root, 'VERSION'), 'utf8').trim();

if (!/^[0-9]+\.[0-9]+\.[0-9]+$/.test(version)) {
  throw new Error(`VERSION must be MAJOR.MINOR.PATCH, got "${version}"`);
}

const { values } = parseArgs({
  options: {
    from: { type: 'string' },
    'dry-run': { type: 'boolean', default: false }
  },
  strict: true
});

if (!values.from) {
  console.error('Usage: node scripts/sync-version.mjs --from <old-version>');
  console.error('  e.g. node scripts/sync-version.mjs --from 0.26.0');
  process.exit(1);
}

const oldVersion = values.from;
const dryRun = values['dry-run'];

if (oldVersion === version) {
  console.log(`Nothing to do — old and new version are both ${version}`);
  process.exit(0);
}

console.log(`${dryRun ? '[DRY RUN] ' : ''}Syncing RWP version: ${oldVersion} → ${version}`);
console.log();

// --- Targeted replacements (structured files with known patterns) ---

const targeted = [
  {
    file: 'conformance/src/lib.rs',
    pattern: /pub const RWP_VERSION: &str = "[^"]+";/,
    value: `pub const RWP_VERSION: &str = "${version}";`
  },
  {
    file: '../rhumbprotocol.dev/src/lib/config/site.ts',
    pattern: /version: '[^']+',/,
    value: `version: '${version}',`
  },
  {
    file: '../rhumbprotocol.dev/package.json',
    pattern: /"version": "[^"]+"/,
    value: `"version": "${version}"`
  },
  {
    file: '../yakkl-meridian-rs/rhumb-spec/Cargo.toml',
    pattern: /version = "0\.0\.0"/,
    value: `version = "${version}"`
  }
];

let targetedCount = 0;

for (const { file, pattern, value } of targeted) {
  const path = join(root, file);
  let source;
  try {
    source = readFileSync(path, 'utf8');
  } catch {
    console.log(`  skip (not found): ${file}`);
    continue;
  }
  if (!pattern.test(source)) {
    console.log(`  skip (pattern not matched): ${file}`);
    continue;
  }
  const updated = source.replace(pattern, value);
  if (updated !== source) {
    if (!dryRun) writeFileSync(path, updated);
    console.log(`  targeted: ${file}`);
    targetedCount++;
  }
}

// --- Bulk replacement across protocol repo files ---

const BULK_EXTENSIONS = new Set(['.md', '.yaml', '.yml', '.json', '.ts', '.svg', '.rs']);
const SKIP_DIRS = new Set(['node_modules', '.git', 'target', '.svelte-kit', 'build']);

function walk(dir, results = []) {
  for (const entry of readdirSync(dir)) {
    if (SKIP_DIRS.has(entry)) continue;
    const full = join(dir, entry);
    const stat = statSync(full);
    if (stat.isDirectory()) {
      walk(full, results);
    } else if (BULK_EXTENSIONS.has(extname(entry))) {
      results.push(full);
    }
  }
  return results;
}

const bulkDirs = [
  root,
  join(root, '../rhumbprotocol.dev/static/brand')
];

let bulkCount = 0;

for (const dir of bulkDirs) {
  let files;
  try {
    files = walk(dir);
  } catch {
    console.log(`  skip dir (not found): ${dir}`);
    continue;
  }

  for (const file of files) {
    const source = readFileSync(file, 'utf8');
    if (!source.includes(oldVersion)) continue;

    const updated = source.replaceAll(oldVersion, version);
    if (updated !== source) {
      if (!dryRun) writeFileSync(file, updated);
      const rel = relative(join(root, '..'), file);
      console.log(`  bulk: ${rel}`);
      bulkCount++;
    }
  }
}

// --- Also update rhumb-spec Cargo.toml description (old version in prose) ---
const specToml = join(root, '../yakkl-meridian-rs/rhumb-spec/Cargo.toml');
try {
  const source = readFileSync(specToml, 'utf8');
  if (source.includes(oldVersion)) {
    const updated = source.replaceAll(oldVersion, version);
    if (updated !== source) {
      if (!dryRun) writeFileSync(specToml, updated);
      console.log(`  bulk: yakkl-meridian-rs/rhumb-spec/Cargo.toml (description)`);
      bulkCount++;
    }
  }
} catch { /* not found, skip */ }

console.log();
console.log(`Done. ${targetedCount} targeted + ${bulkCount} bulk replacements.`);
if (dryRun) console.log('(dry run — no files were modified)');
else console.log('Verify with: git diff --stat');
