import { execFileSync, spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';

const checker = fileURLToPath(new URL('./check-npm-audit.mjs', import.meta.url));
const expected = {
  vulnerabilities: {
    '@expo/cli': { via: ['@expo/config', '@expo/metro-config', '@expo/prebuild-config', 'zod'], nodes: ['node_modules/expo/node_modules/@expo/cli'] },
    '@expo/config': { via: ['deepmerge'], nodes: ['node_modules/@expo/config'] },
    '@expo/local-build-cache-provider': { via: ['@expo/config'], nodes: ['node_modules/@expo/local-build-cache-provider'] },
    '@expo/metro-config': { via: ['@expo/config'], nodes: ['node_modules/@expo/metro-config'] },
    '@expo/prebuild-config': { via: ['@expo/config'], nodes: ['node_modules/@expo/prebuild-config'] },
    deepmerge: { via: [{ url: 'https://github.com/advisories/GHSA-gcqx-w48m-w2h4' }], nodes: ['node_modules/deepmerge'] },
    expo: { via: ['@expo/cli', '@expo/config', '@expo/local-build-cache-provider', '@expo/metro-config'], nodes: ['node_modules/expo'] },
    zod: { via: [{ url: 'https://github.com/advisories/GHSA-mrhx-c7xc-m3w5' }], nodes: ['node_modules/zod'] },
  },
};

function report(value) {
  const directory = mkdtempSync(join(tmpdir(), 'clearsign-audit-'));
  const path = join(directory, 'audit.json');
  writeFileSync(path, JSON.stringify(value));
  return path;
}

test('accepts only the reviewed advisory graph', () => {
  const output = execFileSync(process.execPath, [checker, report(expected)], { encoding: 'utf8' });
  assert.match(output, /Known Expo CLI advisories only/);
});

test('rejects an additional advisory on an allowed package', () => {
  const changed = structuredClone(expected);
  changed.vulnerabilities.zod.via.push({ url: 'https://github.com/advisories/GHSA-new-advisory' });
  const result = spawnSync(process.execPath, [checker, report(changed)], { encoding: 'utf8' });
  assert.notEqual(result.status, 0);
  assert.match(result.stderr, /advisory set changed/);
});
