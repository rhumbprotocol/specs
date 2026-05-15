#!/usr/bin/env node
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const root = new URL('..', import.meta.url).pathname;
const version = readFileSync(join(root, 'VERSION'), 'utf8').trim();
const [major, minor, patch] = version.split('.').map(Number);

if (!/^[0-9]+\.[0-9]+\.[0-9]+$/.test(version)) {
  throw new Error(`VERSION must be MAJOR.MINOR.PATCH, got ${version}`);
}

const replacements = [
  {
    file: 'conformance/src/lib.rs',
    pattern: /pub const RWP_VERSION: &str = "[^"]+";/,
    value: `pub const RWP_VERSION: &str = "${version}";`
  },
  {
    file: '../rhumbprotocol.dev/src/lib/config/site.ts',
    pattern: /version: '[^']+',/,
    value: `version: '${version}',`
  }
];

for (const replacement of replacements) {
  const path = join(root, replacement.file);
  const source = readFileSync(path, 'utf8');
  if (!replacement.pattern.test(source)) {
    throw new Error(`Pattern not found in ${replacement.file}`);
  }
  const updated = source.replace(replacement.pattern, replacement.value);
  if (updated !== source) {
    writeFileSync(path, updated);
  }
}

console.log(`Synced RWP version ${version}`);
