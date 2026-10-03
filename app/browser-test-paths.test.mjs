import assert from 'node:assert/strict';
import { test } from 'node:test';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { chromeCandidates } from './browser-test-paths.mjs';

test('Windows browser discovery includes machine and user installations', () => {
  const candidates = chromeCandidates({
    CHROME_PATH: 'C:\\custom\\chrome.exe',
    ProgramFiles: 'C:\\Program Files',
    'ProgramFiles(x86)': 'C:\\Program Files (x86)',
    LOCALAPPDATA: 'C:\\Users\\Reviewer\\AppData\\Local',
  }, 'win32');
  assert.equal(candidates[0], 'C:\\custom\\chrome.exe');
  for (const root of ['C:\\Program Files', 'C:\\Program Files (x86)', 'C:\\Users\\Reviewer\\AppData\\Local']) {
    assert.ok(candidates.includes(`${root}\\Google\\Chrome\\Application\\chrome.exe`));
  }
});

test('page URLs preserve spaces and fragment characters in Windows paths', () => {
  const filename = 'D:\\a\\Clear Sign#test\\app\\dist\\clearsign.html';
  const url = pathToFileURL(filename, { windows: true });
  assert.equal(url.protocol, 'file:');
  assert.equal(url.hash, '');
  assert.equal(fileURLToPath(url, { windows: true }), filename);
});

test('Unix discovery works without Windows environment variables', () => {
  assert.ok(chromeCandidates({}, 'linux').includes('/usr/bin/google-chrome'));
  assert.ok(chromeCandidates({}, 'darwin').includes('/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'));
});
