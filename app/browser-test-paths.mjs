import path from 'node:path';

export function chromeCandidates(env = process.env, platform = process.platform) {
  const windows = platform === 'win32'
    ? [env.ProgramFiles, env['ProgramFiles(x86)'], env.LOCALAPPDATA]
      .filter(Boolean)
      .map(root => path.win32.join(root, 'Google', 'Chrome', 'Application', 'chrome.exe'))
    : [];
  return [
    env.CHROME_PATH,
    ...windows,
    '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
    '/usr/bin/google-chrome',
    '/usr/bin/chromium-browser',
    '/usr/bin/chromium',
  ].filter(Boolean);
}
