# HTML Test Framework Plan B: Chrome Cross-check Runner + CI

> **For agentic workers:** REQUIRED SUB-SKILL: superpowers:subagent-driven-development or superpowers:executing-plans. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Add a Chrome headless cross-check runner (node puppeteer) that runs all `data-chrome="true"` HTML cases and reports where human-written `data-expect-*` disagrees with Chrome's actual layout; wire it into CI.

**Architecture:** A node script `scripts/chrome-cross-check.mjs` uses puppeteer to open each HTML case, reads `getBoundingClientRect()` on `[data-expect-*]` elements, compares against the `data-expect-*` attribute values, and prints a pass/fail/diff summary. CI adds a step that installs Chrome + puppeteer and runs the script. float-pigment's cargo test is unchanged.

**Tech Stack:** node, puppeteer (ships its own Chrome), GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-07-17-html-test-framework-design.md` (sections "Chrome cross-check runner" + "CI 集成").

---

## File Structure

- Create: `scripts/chrome-cross-check.mjs` — the runner.
- Create: `scripts/package.json` — declares the puppeteer dependency (keeps node deps out of the Rust crate; `npm install` / `pnpm install` in this dir).
- Modify: `.github/workflows/rust.yml` (or a new `.github/workflows/chrome-cross-check.yml`) — add the Chrome cross-check job.

---

### Task 1: node package + puppeteer dependency

**Files:**
- Create: `scripts/package.json`

- [ ] **Step 1: Write package.json**

`scripts/package.json`:
```json
{
  "name": "float-pigment-chrome-cross-check",
  "private": true,
  "type": "module",
  "scripts": {
    "cross-check": "node chrome-cross-check.mjs"
  },
  "dependencies": {
    "puppeteer": "^23.0.0"
  }
}
```

- [ ] **Step 2: Install puppeteer (local验证)**

Run: `cd scripts && pnpm install && cd ..`
Expected: `node_modules/puppeteer` + downloaded Chromium. (If no pnpm, `npm install` works too — puppeteer downloads Chrome on install.)

- [ ] **Step 3: Commit**
```bash
git add scripts/package.json scripts/pnpm-lock.yaml
git commit -m "test(framework): add puppeteer dep for Chrome cross-check"
```

### Task 2: chrome-cross-check.mjs runner

**Files:**
- Create: `scripts/chrome-cross-check.mjs`

- [ ] **Step 1: Write the runner**

`scripts/chrome-cross-check.mjs`:
```javascript
// Chrome cross-check: open each tests/cases/**/*.html with data-chrome!="false",
// compare getBoundingClientRect of [data-expect-*] elements vs the attribute values.
// Usage: node chrome-cross-check.mjs [--dir <cases-dir>] [--tolerance <px>]
import puppeteer from 'puppeteer';
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';

const root = process.argv.includes('--dir')
  ? process.argv[process.argv.indexOf('--dir') + 1]
  : new URL('../float-pigment-forest/tests/cases/', import.meta.url).pathname;
const tolerance = process.argv.includes('--tolerance')
  ? parseFloat(process.argv[process.argv.indexOf('--tolerance') + 1])
  : 1.0; // px tolerance for float rounding

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

const browser = await puppeteer.launch({ headless: 'new' });
let pass = 0, fail = 0, skipped = 0;
const failures = [];

for (const file of files) {
  const html = readFileSync(file, 'utf8');
  if (/data-chrome="false"/.test(html)) { skipped++; continue; }
  const page = await browser.newPage();
  try {
    await page.goto('file://' + file, { waitUntil: 'networkidle0' });
    const diffs = await page.evaluate(() => {
      const els = [...document.querySelectorAll('[data-expect-width],[data-expect-height],[data-expect-left],[data-expect-top]')];
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
  } finally {
    await page.close();
  }
}
await browser.close();

console.error(`\npass=${pass} fail=${fail} skipped=${skipped}`);
if (failures.length) {
  console.error('\nFAILURES (expect vs Chrome actual):');
  for (const f of failures.slice(0, 50)) {
    console.error(`  ${f.file}: ${f.axis} expected=${f.expected} actual=${f.actual.toFixed(1)}`);
  }
  if (failures.length > 50) console.error(`  ... and ${failures.length - 50} more`);
  process.exit(1);
}
```

- [ ] **Step 2: Run it locally (摸底)**

Run: `cd scripts && node chrome-cross-check.mjs 2>&1 | tail -30`
Expected: scans ~932 cases. Many likely fail on first run (float-pigment vs Chrome differences + human-written errors). Capture the summary count.

- [ ] **Step 3: Commit**
```bash
git add scripts/chrome-cross-check.mjs
git commit -m "test(framework): chrome cross-check runner (puppeteer)"
```

### Task 3: Triage the first run diffs

**Files:** none (analysis) — optionally modify HTML cases to add `data-chrome="false"` where float-pigment intentionally diverges.

- [ ] **Step 1: Categorize failures**

Read the failure list from Task 2. For each, decide:
- **Human error** (expect wrong, Chrome right): fix the `data-expect-*` in the HTML.
- **float-pigment intentional divergence** (float-pigment特有 / CSS subset): add `data-chrome="false"` to the root element of that case.
- **Real float-pigment bug** (expect right, Chrome right, float-pigment wrong): file separately — don't paper over with `data-chrome=false`.

- [ ] **Step 2: Apply fixes (batch)**

For each category, edit the HTML files accordingly. Re-run `node chrome-cross-check.mjs` until `fail=0` (or only known-divergence cases remain, all marked `data-chrome="false"`).

- [ ] **Step 3: Commit triage**
```bash
git add -A float-pigment-forest/tests/cases/
git commit -m "test(framework): triage Chrome cross-check diffs (fix expects / mark divergences)"
```

### Task 4: CI integration

**Files:**
- Create or modify: `.github/workflows/chrome-cross-check.yml` (or add a job to existing rust.yml)

- [ ] **Step 1: Write the workflow**

`.github/workflows/chrome-cross-check.yml`:
```yaml
name: chrome-cross-check

on:
  push:
    branches: [master]
  pull_request:

jobs:
  cross-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: pnpm/action-setup@v4
        with:
          version: 9
      - uses: actions/setup-node@v4
        with:
          node-version: 20
          cache: 'pnpm'
          cache-dependency-path: scripts/pnpm-lock.yaml
      - name: Install puppeteer
        working-directory: scripts
        run: pnpm install --frozen-lockfile
      - name: Run Chrome cross-check
        run: node scripts/chrome-cross-check.mjs
```

- [ ] **Step 2: Commit**
```bash
git add .github/workflows/chrome-cross-check.yml
git commit -m "ci: add Chrome cross-check job (puppeteer vs data-expect-*)"
```

- [ ] **Step 3: Push + verify CI green**
```bash
git push
```
Watch the Actions tab; iterate on the workflow (cache path, lockfile) until green.

---

## Self-Review

**Spec coverage:**
- Chrome runner (puppeteer, getBoundingClientRect, data-chrome filter, diff report) → Task 2
- CI step (setup Chrome via puppeteer, run script, fail CI) → Task 4
- data-chrome="false" triage → Task 3

**Placeholder scan:** Task 3 is inherently iterative (triage depends on actual diff output); it gives concrete decision criteria + loop until fail=0. Not a placeholder.

**Type/consistency:** `data-expect-*` and `data-chrome` match Plan A's HTML format. Runner reads same attributes.

**Risk note (from spec):** "Chrome vs float-pigment 差异多" — Task 3 (triage) is the unpredictable work. First run may show many diffs; budget time. If diffs reveal real float-pigment bugs (not test errors), file them separately — don't mask with `data-chrome=false`.
