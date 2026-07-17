// Converts a Rust test file of `assert_xml!(r#"..."#)` cases into HTML files.
// Usage: node scripts/convert-assert-xml.mjs <input.rs> <output-cases-dir> <topic>
// Output: <output-cases-dir>/<topic>/<test_name>.html with data-expect-* + data-chrome="true".
import { readFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const [, , inputPath, outDir, topic] = process.argv;
if (!inputPath || !outDir || !topic) {
  console.error('Usage: node scripts/convert-assert-xml.mjs <input.rs> <output-cases-dir> <topic>');
  process.exit(1);
}
const src = readFileSync(inputPath, 'utf8');

// Match: #[test]\n fn name() { assert_xml!(r#"XML"#) } or assert_xml!(r#"XML"#, true)
const re =
  /#\[test\][\s\S]*?(#\[ignore\]\s*)?fn\s+(\w+)\s*\(\s*\)\s*\{[\s\S]*?assert_xml!\(\s*r#"([\s\S]*?)"#\s*(?:,\s*(?:true|false))?\s*\)/g;
let m;
let count = 0;
while ((m = re.exec(src)) !== null) {
  const ignored = !!m[1];
  const name = m[2];
  const xml = m[3].trim();
  // expect_* -> data-expect-*
  let body = xml.replace(/expect_(width|height|left|top)/g, 'data-expect-$1');
  // add data-chrome="true" to the root (first) opening tag, unless already present
  if (!/\bdata-chrome=/.test(body)) {
    body = body.replace(/(<[a-zA-Z][^>]*?)(\s*>)/, '$1 data-chrome="true"$2');
  }
  if (ignored && !/\bdata-ignore=/.test(body)) {
    body = body.replace(/(<[a-zA-Z][^>]*?)(\s*>)/, '$1 data-ignore="true"$2');
  }
  const html = `<!DOCTYPE html>\n${body}\n`;
  mkdirSync(join(outDir, topic), { recursive: true });
  writeFileSync(join(outDir, topic, `${name}.html`), html);
  count++;
}
console.error(`converted ${count} cases from ${inputPath} -> ${outDir}/${topic}/`);
