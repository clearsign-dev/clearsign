// Drive the installed production app through the native WebDriver. No mocks or
// test-only server are compiled into the application.
import assert from 'node:assert/strict';
import { spawn, execFileSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { setTimeout as sleep } from 'node:timers/promises';

const application = resolve(process.argv[2] || '');
assert.ok(process.argv[2] && existsSync(application), 'installed application must exist');
const version = JSON.parse(readFileSync(new URL('./package.json', import.meta.url))).version;
const fixture = JSON.parse(readFileSync(new URL(
  '../signing-core/crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json', import.meta.url)));
// tauri-driver 2.0.5 binds its intermediary to loopback itself.
const driver = spawn('tauri-driver', [], {
  stdio: 'inherit', detached: process.platform !== 'win32',
});
let driverError;
driver.on('error', error => { driverError = error; });
let session;
async function command(method, path, body) {
  const response = await fetch(`http://127.0.0.1:4444${path}`, {
    method, headers: { 'content-type': 'application/json' },
    body: body === undefined ? undefined : JSON.stringify(body),
    signal: AbortSignal.timeout(60000),
  });
  const result = await response.json();
  assert.ok(response.ok && !result.value?.error, JSON.stringify(result));
  return result.value;
}
async function until(check, label) {
  const deadline = Date.now() + 30000;
  while (Date.now() < deadline) {
    if (driverError) throw driverError;
    assert.equal(driver.exitCode, null, 'native driver exited before the check completed');
    if (await check()) return;
    await sleep(200);
  }
  throw new Error(`Timed out: ${label}`);
}
const evaluate = (script, args = []) => command('POST', `/session/${session}/execute/sync`, { script, args });
async function click(id) {
  const element = await command('POST', `/session/${session}/element`, { using: 'css selector', value: `#${id}` });
  await command('POST', `/session/${session}/element/${element['element-6066-11e4-a52e-4f735466cecf']}/click`, {});
}
const output = () => evaluate('return document.getElementById("out").innerText');
try {
  await until(async () => {
    try { return (await command('GET', '/status')).ready; } catch { return false; }
  }, 'native driver startup');
  const opened = await command('POST', '/session', {
    capabilities: { alwaysMatch: { 'tauri:options': { application } } },
  });
  session = opened.sessionId;
  assert.ok(session, 'native session was created');
  await until(() => evaluate('return document.getElementById("version-line")?.innerText.includes(arguments[0])', [version]), 'Rust build version');
  await click('load-example');
  await until(async () => (await output()).includes('DO NOT SIGN'), 'critical historical review');
  const review = await output();
  assert.match(review, /DELEGATECALL/);
  const displayedHash = await evaluate('return document.querySelector("#out .hash").textContent');
  assert.equal(displayedHash.replace(/\s/g, '').toLowerCase(), fixture.safeTxHash.toLowerCase(), 'exact recomputed Safe hash');
  await evaluate('const el = document.getElementById("json"); el.value += " "; el.dispatchEvent(new Event("input", {bubbles:true}));');
  assert.equal(await output(), '', 'editing removes the old review');
  await click('load-example');
  await until(async () => (await output()).includes('DO NOT SIGN'), 'second review');
  await click('clear');
  assert.equal(await output(), '', 'clear removes the review');
  await evaluate('const el = document.getElementById("json"); el.value = "{"; el.dispatchEvent(new Event("input", {bubbles:true}));');
  await click('review');
  await until(async () => (await output()).length > 0, 'malformed input refusal');
  assert.match(await output(), /cannot|invalid|refus|expected|JSON/i);
  assert.ok(!(await output()).includes(fixture.safeTxHash), 'no stale hash on refusal');
  console.log(`Installed app smoke passed: ${application} (${version})`);
} catch (error) {
  if (session) {
    try {
      console.error(await evaluate('return document.body.innerText'));
      const screenshot = await command('GET', `/session/${session}/screenshot`);
      writeFileSync('desktop-smoke.png', Buffer.from(screenshot, 'base64'));
    } catch (diagnosticError) { console.error(diagnosticError); }
  }
  throw error;
} finally {
  if (session) {
    try { await command('DELETE', `/session/${session}`); } catch (error) { console.error(error); }
  }
  if (driver.pid) {
    try {
      if (process.platform === 'win32') execFileSync('taskkill', ['/pid', String(driver.pid), '/T', '/F']);
      else process.kill(-driver.pid, 'SIGTERM');
    } catch { /* The driver may already have exited with its session. */ }
  }
}
