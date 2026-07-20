// Converts a Rust test file of `assert_xml!(r#"..."#)` cases into HTML files.
// Handles a single #[test] containing MULTIPLE assert_xml! calls (each becomes
// its own HTML case with _1, _2, ... suffix).
// Usage: node scripts/convert-assert-xml.mjs <input.rs> <output-cases-dir> <topic>
import { readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [, , inputPath, outDir, topic] = process.argv;
if (!inputPath || !outDir || !topic) {
  console.error('Usage: node scripts/convert-assert-xml.mjs <input.rs> <output-cases-dir> <topic>');
  process.exit(1);
}
const src = readFileSync(inputPath, 'utf8');

// Match a #[test] block: optional #[ignore], #[test], fn name() { body }
// body ends at a line that's just "}" (closing the fn).
const testRe = /(#\[ignore\]\s*)?#\[test\][\s\S]*?fn\s+(\w+)\s*\(\s*\)\s*\{([\s\S]*?)\n\s*\}/g;
// Within a test body, find all assert_xml!(r#"..."#) calls.
const xmlRe = /assert_xml!\(\s*r#"([\s\S]*?)"#\s*(?:,\s*(?:true|false))?\s*\)/g;

let tm;
let count = 0;
while ((tm = testRe.exec(src)) !== null) {
  const ignored = !!tm[1];
  const fnName = tm[2];
  const body = tm[3];
  const xmls = [...body.matchAll(xmlRe)].map((x) => x[1].trim());
  xmls.forEach((xml, i) => {
    const caseName = xmls.length > 1 ? `${fnName}_${i + 1}` : fnName;
    // expect_* -> data-expect-*
    let htmlBody = xml.replace(/expect_(width|height|left|top)/g, 'data-expect-$1');
    // add data-chrome="true" to root (first) opening tag, unless already present
    if (!/\bdata-chrome=/.test(htmlBody)) {
      htmlBody = htmlBody.replace(/(<[a-zA-Z][^>]*?)(\s*>)/, '$1 data-chrome="true"$2');
    }
    // add data-ignore="true" if the original test was #[ignore]
    if (ignored && !/\bdata-ignore=/.test(htmlBody)) {
      htmlBody = htmlBody.replace(/(<[a-zA-Z][^>]*?)(\s*>)/, '$1 data-ignore="true"$2');
    }
    const html = `<!DOCTYPE html>\n${htmlBody}\n`;
    mkdirSync(join(outDir, topic), { recursive: true });
    writeFileSync(join(outDir, topic, `${caseName}.html`), html);
    count++;
  });
}
console.error(`converted ${count} cases from ${inputPath} -> ${outDir}/${topic}/`);
