#!/usr/bin/env node
/**
 * Build the orbit-mcp sidecar binary and copy it to tauri/binaries/
 * with the target-triple suffix that Tauri's externalBin expects.
 *
 * Usage:
 *   node scripts/build-sidecar.mjs           # debug build
 *   node scripts/build-sidecar.mjs --release  # release build
 */

import { execSync } from 'node:child_process';
import { copyFileSync, mkdirSync, existsSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const isRelease = process.argv.includes('--release');

// Detect Rust target triple
function getTargetTriple() {
  // CI or explicit override
  if (process.env.TAURI_ENV_TARGET_TRIPLE) {
    return process.env.TAURI_ENV_TARGET_TRIPLE;
  }
  const rustcOutput = execSync('rustc -vV', { encoding: 'utf-8' });
  const match = rustcOutput.match(/^host:\s*(.+)$/m);
  if (!match) {
    throw new Error('Could not detect Rust target triple from `rustc -vV`');
  }
  return match[1].trim();
}

const triple = getTargetTriple();
const isWindows = triple.includes('windows');
const ext = isWindows ? '.exe' : '';

console.log(`[sidecar] target: ${triple}`);
console.log(`[sidecar] mode: ${isRelease ? 'release' : 'debug'}`);

// Ensure tauri/binaries/ exists with a placeholder so Tauri's build.rs passes validation
const destDir = join('tauri', 'binaries');
const dest = join(destDir, `orbit-mcp-${triple}${ext}`);

if (!existsSync(destDir)) {
  mkdirSync(destDir, { recursive: true });
}
if (!existsSync(dest)) {
  writeFileSync(dest, '');
  console.log(`[sidecar] created placeholder: ${dest}`);
}

// Build
const profile = isRelease ? '--release' : '';
execSync(`cargo build --manifest-path tauri/Cargo.toml --bin orbit-mcp ${profile}`.trim(), {
  stdio: 'inherit',
});

// Copy real binary over placeholder
const profileDir = isRelease ? 'release' : 'debug';
const src = join('tauri', 'target', profileDir, `orbit-mcp${ext}`);

copyFileSync(src, dest);
console.log(`[sidecar] ${src} -> ${dest}`);
