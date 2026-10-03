import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import vm from 'node:vm';

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
