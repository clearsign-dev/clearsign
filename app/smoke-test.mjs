// Load the built page in a real browser and use it.
//
// This exists because a constant was referenced in three places and defined in
// none, the build's parse check passed, every Rust test passed, and the page
// shipped broken. A page that parses is not a page that works. Nothing here
// mocks the front end: it loads dist/clearsign.html, types into it, clicks it,
// and fails on any uncaught error.
import { spawn } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { setTimeout as sleep } from 'node:timers/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { chromeCandidates } from './browser-test-paths.mjs';

// fileURLToPath, not .pathname: this repository lives under a path with a
// space in it, and .pathname hands back %20.
const HERE = path.dirname(fileURLToPath(import.meta.url));
const PAGE = path.join(HERE, 'dist', 'clearsign.html');
const FIXTURE = path.join(
  HERE, '..', 'signing-core', 'crates', 'clearsign-cli', 'tests', 'fixtures', 'bybit-safe-tx.json',
);

const CHROME = chromeCandidates().find(p => existsSync(p));

if (!CHROME) {
  console.error('no Chrome found; set CHROME_PATH');
  process.exit(2);
}
if (!existsSync(PAGE)) {
  console.error(`${PAGE} is missing — run app/build.sh first`);
  process.exit(2);
}

const profile = mkdtempSync(path.join(tmpdir(), 'clearsign-smoke-'));
const chrome = spawn(CHROME, [
  '--headless=new', '--remote-debugging-port=0',
  '--disable-gpu', '--no-first-run', '--no-sandbox',
  `--user-data-dir=${profile}`, 'about:blank',
], { stdio: ['ignore', 'ignore', 'pipe'] });
let launchError;
let chromeLog = '';
chrome.on('error', (error) => { launchError = error; });
chrome.stderr.on('data', (chunk) => { chromeLog = (chromeLog + chunk).slice(-8192); });
const exited = new Promise((resolve) => chrome.once('close', resolve));

let failures = 0;
const check = (name, ok, detail = '') => {
  console.log(`  ${ok ? 'ok  ' : 'FAIL'}  ${name}${ok || !detail ? '' : `\n          ${detail}`}`);
  if (!ok) failures += 1;
};

try {
  let target;
  for (let i = 0; i < 80; i++) {
    if (launchError) throw launchError;
    if (chrome.exitCode !== null || chrome.signalCode !== null) {
      throw new Error(`Chrome exited before its debugging endpoint was ready:\n${chromeLog}`);
    }
    try {
      const port = readFileSync(path.join(profile, 'DevToolsActivePort'), 'utf8').split('\n')[0];
      if (/^\d+$/.test(port)) {
        const response = await fetch(`http://127.0.0.1:${port}/json/list`, {
          signal: AbortSignal.timeout(1000),
        });
        const targets = await response.json();
        if (Array.isArray(targets)) target = targets.find((t) => t.type === 'page' && t.webSocketDebuggerUrl);
        if (target) break;
      }
    } catch { /* not up yet */ }
    await sleep(250);
  }
  if (!target) throw new Error(`Chrome debugging endpoint did not become ready:\n${chromeLog}`);
  const ws = new WebSocket(target.webSocketDebuggerUrl);
  await new Promise((res) => (ws.onopen = res));

  let id = 0;
  const pending = new Map();
  const errors = [];
  ws.onmessage = (m) => {
    const msg = JSON.parse(m.data);
    if (msg.id && pending.has(msg.id)) { pending.get(msg.id)(msg); pending.delete(msg.id); return; }
    // Anything the page throws, and anything it logs as an error, is a failure.
    if (msg.method === 'Runtime.exceptionThrown') {
      errors.push(msg.params.exceptionDetails?.exception?.description
        || msg.params.exceptionDetails?.text || 'exception');
    }
    if (msg.method === 'Runtime.consoleAPICalled' && msg.params.type === 'error') {
      errors.push((msg.params.args || []).map((a) => a.value ?? a.description).join(' '));
    }
  };
  const send = (method, params = {}) => new Promise((res) => {
    const n = ++id; pending.set(n, res);
    ws.send(JSON.stringify({ id: n, method, params }));
  });
  const evaluate = async (expression) => {
    const r = await send('Runtime.evaluate', { expression, returnByValue: true, awaitPromise: true });
    if (r.result?.exceptionDetails) {
      errors.push(r.result.exceptionDetails.exception?.description || 'evaluate threw');
      return undefined;
    }
    return r.result?.result?.value;
  };

  await send('Page.enable');
  await send('Runtime.enable');
  await send('Page.navigate', { url: pathToFileURL(PAGE).href });
  await sleep(2500);

  console.log('the built page:');

  check('loads with a title', Boolean(await evaluate('document.title')));
  check('the reviewer is wired up', await evaluate('typeof window.__review === "function" || !!document.querySelector("textarea")'));

  // The regression that prompted this file: a constant referenced in the drop
  // handler and declared nowhere. It cannot be found by looking at the page —
  // module bindings are not visible from here — only by making the code run.
  // So drop a file that is too large and see whether the page says so or throws.
  const dropped = await evaluate(`(async () => {
    // The handler is async, so anything it throws becomes an unhandled
    // rejection rather than an exception this tool would otherwise see.
    const rejections = [];
    const onReject = (e) => rejections.push(String(e.reason && e.reason.message || e.reason));
    window.addEventListener('unhandledrejection', onReject);

    const SIZE = 2 * 1024 * 1024;
    const big = new File(['x'.repeat(SIZE)], 'huge.json', { type: 'application/json' });
    const dt = new DataTransfer();
    dt.items.add(big);
    const target = document.getElementById('json');
    target.dispatchEvent(new DragEvent('drop', { bubbles: true, cancelable: true, dataTransfer: dt }));
    await new Promise(r => setTimeout(r, 900));
    window.removeEventListener('unhandledrejection', onReject);

    return {
      rejections,
      // The exact size, formatted, appears only if the handler reached its own
      // message. Matching on the word "bytes" would match the rest of the page.
      saidSoByName: document.body.innerText.includes(SIZE.toLocaleString()),
      // And it must not have quietly loaded two megabytes into the box.
      boxLeftAlone: document.getElementById('json').value.length < 1000,
    };
  })()`);
  check(
    'an oversized dropped file is refused by name',
    Boolean(dropped && dropped.saidSoByName),
    'the handler never printed the size, so it did not get that far',
  );
  check(
    'and nothing was rejected on the way',
    Boolean(dropped) && dropped.rejections.length === 0,
    (dropped?.rejections || []).join(' | '),
  );
  check(
    'and the file was not read into the box anyway',
    Boolean(dropped && dropped.boxLeftAlone),
  );

  // Review the real Bybit transaction through the page, as a person would.
  const fixture = readFileSync(FIXTURE, 'utf8');
  await evaluate(`(() => {
    const ta = document.querySelector('textarea');
    ta.value = ${JSON.stringify(fixture)};
    ta.dispatchEvent(new Event('input', { bubbles: true }));
    const go = [...document.querySelectorAll('button')].find(b => /review/i.test(b.textContent));
    if (go) go.click();
    return true;
  })()`);
  await sleep(2500);
  const text = (await evaluate('document.body.innerText')) || '';
  check('reviews the Bybit transaction', /DELEGATECALL/i.test(text));
  check('and refuses it', /DO NOT SIGN/i.test(text));

  // A result must not outlive the input that produced it.
  const stale = await evaluate(`(async () => {
    const before = document.getElementById('out').innerText;
    const ta = document.getElementById('json');
    ta.value = ta.value + ' ';
    ta.dispatchEvent(new Event('input', { bubbles: true }));
    await new Promise(r => setTimeout(r, 400));
    return { before: before.length, after: document.getElementById('out').innerText.length };
  })()`);
  check(
    'editing the input drops the result beside it',
    Boolean(stale) && stale.before > 0 && stale.after === 0,
    `result was ${stale?.before} chars, is ${stale?.after} after editing`,
  );

  // Put it back so the clear check below has something to clear.
  await evaluate(`(() => {
    const ta = document.querySelector('textarea');
    ta.value = ${JSON.stringify(readFileSync(FIXTURE, 'utf8'))};
    const go = [...document.querySelectorAll('button')].find(b => /review/i.test(b.textContent));
    if (go) go.click();
    return true;
  })()`);
  await sleep(2000);

  // Clearing must not leave a result behind that looks current.
  await evaluate(`(() => {
    const c = [...document.querySelectorAll('button')].find(b => /clear/i.test(b.textContent));
    if (c) c.click();
    return true;
  })()`);
  await sleep(900);
  const afterClear = (await evaluate('document.body.innerText')) || '';
  check('clearing removes the result', !/DO NOT SIGN/i.test(afterClear));

  const substituted = await evaluate(`(async () => {
    const originalFetch = window.fetch;
    window.fetch = async () => new Response(${JSON.stringify(fixture)}, { status: 200 });
    try {
      document.getElementById('tab-fetch').click();
      document.getElementById('txhash').value = '0x' + '00'.repeat(32);
      document.getElementById('review').click();
      await new Promise(r => setTimeout(r, 500));
      return document.getElementById('out').innerText;
    } finally { window.fetch = originalFetch; }
  })()`);
  check('fetch refuses a different transaction than the requested hash',
    /does not match the requested hash/.test(substituted || ''));

  const matched = await evaluate(`(async () => {
    const originalFetch = window.fetch;
    window.fetch = async () => new Response(${JSON.stringify(fixture)}, { status: 200 });
    try {
      document.getElementById('txhash').value = ${JSON.stringify(JSON.parse(fixture).safeTxHash)};
      document.getElementById('review').click();
      await new Promise(r => setTimeout(r, 500));
      return document.getElementById('out').innerText;
    } finally { window.fetch = originalFetch; }
  })()`);
  check('fetch still reviews a transaction with the requested hash', /DO NOT SIGN/i.test(matched || ''));

  const cancelledFetch = await evaluate(`(async () => {
    const originalFetch = window.fetch;
    let aborted = false;
    window.fetch = async (_, options) => new Promise((resolve, reject) => {
      options.signal.addEventListener('abort', () => {
        aborted = true;
        reject(new DOMException('Cancelled', 'AbortError'));
      }, { once: true });
    });
    try {
      document.getElementById('review').click();
      document.getElementById('network').dispatchEvent(new Event('change'));
      await new Promise(r => setTimeout(r, 50));
      return { aborted, output: document.getElementById('out').innerText };
    } finally { window.fetch = originalFetch; }
  })()`);
  check('changing networks cancels the old request and clears its result',
    cancelledFetch?.aborted && cancelledFetch.output === '');

  const staleDrop = await evaluate(`(async () => {
    document.getElementById('clear').click();
    const file = new File(['{}'], 'delayed.json');
    let finish;
    file.text = () => new Promise(resolve => { finish = resolve; });
    const dt = new DataTransfer(); dt.items.add(file);
    const area = document.getElementById('json');
    area.dispatchEvent(new DragEvent('drop', { bubbles: true, cancelable: true, dataTransfer: dt }));
    area.value = 'newer input';
    area.dispatchEvent(new Event('input', { bubbles: true }));
    finish('{}');
    await new Promise(r => setTimeout(r, 100));
    return { value: area.value, output: document.getElementById('out').innerText };
  })()`);
  check('a delayed file read cannot replace newer input', staleDrop?.value === 'newer input');

  const bounded = await evaluate(`(async () => {
    const originalFetch = window.fetch;
    let cancelled = false;
    let pulls = 0;
    window.fetch = async () => new Response(new ReadableStream({
      pull(controller) {
        pulls += 1;
        if (pulls > 8) controller.close();
        else controller.enqueue(new Uint8Array(256 * 1024));
      },
      cancel() { cancelled = true; },
    }));
    try {
      document.getElementById('tab-fetch').click();
      document.getElementById('txhash').value = '0x' + '00'.repeat(32);
      document.getElementById('review').click();
      await new Promise(r => setTimeout(r, 500));
      return { cancelled, pulls };
    } finally { window.fetch = originalFetch; }
  })()`);
  check('oversized network responses are stopped while streaming', bounded?.cancelled && bounded.pulls < 8);

  check('nothing threw', errors.length === 0, errors.slice(0, 3).join(' | '));

  ws.close();
} finally {
  chrome.kill();
  await Promise.race([exited, sleep(3000)]);
  if (chrome.exitCode === null && chrome.signalCode === null && !launchError) {
    chrome.kill('SIGKILL');
    await exited;
  }
  rmSync(profile, { recursive: true, force: true, maxRetries: 3 });
}

console.log(failures ? `\n${failures} check(s) failed` : '\nall checks passed');
process.exit(failures ? 1 : 0);
