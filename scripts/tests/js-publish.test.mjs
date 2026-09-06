import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, writeFileSync, readFileSync, copyFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import test from 'node:test';

for (const scenario of ['missing', 'published', 'resume', 'network', 'auth', 'publish-failure', 'test-failure', 'prerelease', 'invalid']) {
  test(`js-publish: ${scenario}`, () => {
    const dir = mkdtempSync(join(tmpdir(), 'js-publish-'));
    try {
      for (const path of ['scripts', 'bin', 'modules/bitview-client', 'modules/quickmatch-js']) mkdirSync(join(dir,path), {recursive:true});
      copyFileSync(new URL('../js-publish.sh', import.meta.url), join(dir, 'scripts/js-publish.sh'));
      for (const [path,name,version] of [['bitview-client','bitview-client','1.0.0'], ['quickmatch-js','quickmatch-js','0.12.2']]) {
        writeFileSync(join(dir, 'modules', path, 'package.json'), JSON.stringify({name,version}));
      }
      writeFileSync(join(dir,'modules/bitview-client/index.js'), 'const VERSION = "v1.0.0";\n');
      writeFileSync(join(dir,'bin/npm'), `#!/usr/bin/env node
const fs = require('fs');
const args = process.argv.slice(2), scenario = process.env.SCENARIO;
fs.appendFileSync(process.env.NPM_CALLS, JSON.stringify({args, cwd:process.cwd()})+'\\n');
if (args[0] === 'test') process.exit(scenario === 'test-failure' ? 1 : 0);
if (args[0] === 'view') {
  if (scenario === 'published' || scenario === 'resume' && args[1].startsWith('bitview-client@')) { console.log('"1.0.0"'); process.exit(0); }
  console.log(JSON.stringify({error:{code: scenario === 'network' ? 'ENETUNREACH' : scenario === 'auth' ? 'E401' : 'E404'}})); process.exit(1);
}
if (args[0] === 'publish') process.exit(scenario === 'publish-failure' ? 1 : 0);
process.exit(99);
`, {mode:0o755});
      const version = scenario === 'invalid' ? 'invalid' : scenario === 'prerelease' ? '1.2.3-beta.1' : '1.2.3';
      const result = spawnSync('bash', [join(dir,'scripts/js-publish.sh'), version], {encoding:'utf8', env:{...process.env, PATH:join(dir,'bin')+':'+process.env.PATH, SCENARIO:scenario,NPM_CALLS:join(dir,'calls')}});
      const failed = ['network','auth','publish-failure','test-failure','invalid'].includes(scenario);
      assert.equal(result.status === 0, !failed, result.stderr);
      if (scenario === 'invalid') return;
      const calls = readFileSync(join(dir,'calls'),'utf8').trim().split('\n').map(JSON.parse);
      const published = calls.filter(c=>c.args[0]==='publish');
      assert.equal(published.length, ['missing','prerelease'].includes(scenario) ? 2 : ['resume','publish-failure'].includes(scenario) ? 1 : 0);
      assert.equal(calls[0].args[0], 'test');
      assert.ok(calls[0].cwd.endsWith('quickmatch-js'));
      if (scenario === 'prerelease') {
        assert.equal(published[0].args.at(-1),'beta');
        assert.equal(published[1].args.at(-1),'beta');
      }
      if (scenario === 'resume') assert.ok(published[0].cwd.endsWith('quickmatch-js'));
      assert.equal(JSON.parse(readFileSync(join(dir,'modules/quickmatch-js/package.json'))).version,scenario === 'test-failure' ? '0.12.2' : version);
      if (scenario === 'test-failure') return;
      assert.match(readFileSync(join(dir,'modules/bitview-client/index.js'),'utf8'),new RegExp('v'+version.replaceAll('.','\\.')));
    } finally { rmSync(dir, {recursive:true,force:true}); }
  });
}
