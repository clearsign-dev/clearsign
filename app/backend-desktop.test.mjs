import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import vm from 'node:vm';

test('the shared page declares UTF-8 before its non-ASCII content', () => {
  const template = readFileSync(new URL('./index.template.html', import.meta.url), 'utf8');
  assert.ok(template.startsWith('<meta charset="utf-8">'));
});

test('desktop configuration exposes the bridge used by its backend', async () => {
  const config = JSON.parse(readFileSync(new URL('../desktop/src-tauri/tauri.conf.json', import.meta.url)));
  assert.equal(config.app.withGlobalTauri, true);
  const calls = [];
  const context = vm.createContext({
    window: { __TAURI__: { core: { invoke: async (name, args) => {
      calls.push({ name, args });
      return { ok: true };
    } } } },
  });
  vm.runInContext(readFileSync(new URL('./backend-desktop.js', import.meta.url), 'utf8'), context);
  assert.equal((await context.review('{}', 1, 3)).ok, true);
  assert.equal(JSON.stringify(calls), JSON.stringify([
    { name: 'review_transaction', args: { jsonText: '{}', chainId: 1, version: 3 } },
  ]));
});
