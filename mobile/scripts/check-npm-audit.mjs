import { spawnSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const reportPath = process.argv[2];
const result = reportPath
  ? { stdout: readFileSync(reportPath, 'utf8'), stderr: '' }
  : spawnSync('npm', ['audit', '--omit=dev', '--json'], {
      cwd: new URL('..', import.meta.url),
      encoding: 'utf8',
    });

if (!result.stdout) {
  process.stderr.write(result.stderr || 'npm audit produced no report\n');
  process.exit(1);
}

let report;
try {
  report = JSON.parse(result.stdout);
} catch {
  process.stderr.write('npm audit did not return valid JSON\n');
  process.exit(1);
}

const vulnerabilities = report.vulnerabilities ?? {};
const expected = {
  '@expo/cli': {
    via: ['@expo/config', '@expo/metro-config', '@expo/prebuild-config', 'zod'],
    nodes: ['node_modules/expo/node_modules/@expo/cli'],
  },
  '@expo/config': {
    via: ['deepmerge'],
    nodes: ['node_modules/@expo/config'],
  },
  '@expo/local-build-cache-provider': {
    via: ['@expo/config'],
    nodes: ['node_modules/@expo/local-build-cache-provider'],
  },
  '@expo/metro-config': {
    via: ['@expo/config'],
    nodes: ['node_modules/@expo/metro-config'],
  },
  '@expo/prebuild-config': {
    via: ['@expo/config'],
    nodes: ['node_modules/@expo/prebuild-config'],
  },
  deepmerge: {
    via: ['https://github.com/advisories/GHSA-gcqx-w48m-w2h4'],
    nodes: ['node_modules/deepmerge'],
  },
  expo: {
    via: ['@expo/cli', '@expo/config', '@expo/local-build-cache-provider', '@expo/metro-config'],
    nodes: ['node_modules/expo'],
  },
  zod: {
    via: ['https://github.com/advisories/GHSA-mrhx-c7xc-m3w5'],
    nodes: ['node_modules/zod'],
  },
};

function normalizedGraph(input) {
  return Object.fromEntries(
    Object.entries(input).sort(([a], [b]) => a.localeCompare(b)).map(([name, vulnerability]) => [
      name,
      {
        via: (vulnerability.via ?? []).map((entry) =>
          typeof entry === 'string' ? entry : entry?.url ?? '<missing advisory URL>').sort(),
        nodes: [...(vulnerability.nodes ?? [])].sort(),
      },
    ]),
  );
}

if (JSON.stringify(normalizedGraph(vulnerabilities)) !== JSON.stringify(normalizedGraph(expected))) {
  process.stderr.write(result.stdout);
  process.stderr.write('\nThe mobile dependency advisory set changed; review it before merging.\n');
  process.exit(1);
}

process.stdout.write(
  'Known Expo CLI advisories only: deepmerge GHSA-gcqx-w48m-w2h4 and zod GHSA-mrhx-c7xc-m3w5.\n' +
    'They are build-tool dependencies in the current Expo SDK and have no compatible upstream fix.\n',
);
