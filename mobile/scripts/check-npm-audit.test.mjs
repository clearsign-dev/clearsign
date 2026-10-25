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
    '@expo/cli': { via: ['@expo/code-signing-certificates', 'node-forge', 'zod'], nodes: ['node_modules/expo/node_modules/@expo/cli'] },
    '@expo/code-signing-certificates': { via: ['node-forge'], nodes: ['node_modules/@expo/code-signing-certificates'] },
    expo: { via: ['@expo/cli'], nodes: ['node_modules/expo'] },
    'node-forge': { via: [{ url: 'https://github.com/advisories/GHSA-86w9-cpqp-85rv' }], nodes: ['node_modules/node-forge'] },
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
