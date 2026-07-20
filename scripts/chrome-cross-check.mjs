// Chrome cross-check: open each tests/cases/**/*.html with data-chrome!="false",
// compare getBoundingClientRect of [data-expect-*] elements vs the attribute values.
// Usage: node chrome-cross-check.mjs [--dir <cases-dir>] [--tolerance <px>]
import puppeteer from 'puppeteer';
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const rootArgIdx = process.argv.indexOf('--dir');
const root = rootArgIdx >= 0
  ? resolve(process.argv[rootArgIdx + 1])
  : resolve(__dirname, '../float-pigment-forest/tests/cases');
const tolArgIdx = process.argv.indexOf('--tolerance');
const tolerance = tolArgIdx >= 0
  ? parseFloat(process.argv[tolArgIdx + 1])
  : 1.0;

function walkHtml(dir, out = []) {
  for (const e of readdirSync(dir)) {
    const p = join(dir, e);
    if (statSync(p).isDirectory()) walkHtml(p, out);
    else if (e.endsWith('.html')) out.push(p);
  }
  return out;
}

const files = walkHtml(root);
console.error(`scanning ${files.length} HTML cases in ${root}`);

const browser = await puppeteer.launch({
  headless: true,
  protocolTimeout: 60000,
  args: ['--no-sandbox', '--disable-setuid-sandbox'],
});
let pass = 0, fail = 0, skipped = 0;
const failures = [];

for (const file of files) {
  const html = readFileSync(file, 'utf8');
  if (/data-chrome="false"/.test(html)) { skipped++; continue; }
  const page = await browser.newPage();
  try {
    await page.setViewport({ width: 375, height: 750 });
    await page.goto('file://' + file, { waitUntil: 'load' });
    await page.addStyleTag({ content: 'html,body { margin: 0; padding: 0; }' });
    const diffs = await page.evaluate(() => {
      const els = [...document.querySelectorAll(
        '[data-expect-width],[data-expect-height],[data-expect-left],[data-expect-top]'
      )];
      return els.map(el => {
        const r = el.getBoundingClientRect();
        const checks = [];
        for (const axis of ['width', 'height', 'left', 'top']) {
          const attr = el.getAttribute('data-expect-' + axis);
          if (attr == null) continue;
          checks.push({ axis, expected: parseFloat(attr), actual: r[axis] });
        }
        return checks;
      });
    });
    let caseFail = false;
    for (const checks of diffs) {
      for (const c of checks) {
        if (Math.abs(c.expected - c.actual) > tolerance) {
          caseFail = true;
          failures.push({ file, axis: c.axis, expected: c.expected, actual: c.actual });
        }
      }
    }
    if (caseFail) fail++; else pass++;
  } catch (e) {
    fail++;
    failures.push({ file, axis: 'ERROR', expected: 0, actual: 0, error: String(e) });
  } finally {
    await page.close();
  }
}
await browser.close();

console.error(`\npass=${pass} fail=${fail} skipped=${skipped}`);
if (failures.length) {
  console.error('\nFAILURES by topic:');
  const byTopic = {};
  for (const f of failures) {
    const topic = f.file.replace(/.*\/cases\//, '').replace(/\/.*$/, '');
    byTopic[topic] = (byTopic[topic] || 0) + 1;
  }
  for (const [t, c] of Object.entries(byTopic).sort((a, b) => b[1] - a[1])) {
    console.error(`  ${c}\t${t}`);
  }
  console.error('\nALL FAILURES:');
  for (const f of failures) {
    if (f.axis === 'ERROR') console.error(`  ${f.file}: ${f.error}`);
    else console.error(`  ${f.file}: ${f.axis} expected=${f.expected} actual=${f.actual.toFixed(1)}`);
  }
  process.exit(1);
}
