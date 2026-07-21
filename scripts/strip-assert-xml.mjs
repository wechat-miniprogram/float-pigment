// Strips pure assert_xml! tests from a .rs file, keeping code-constructed tests.
// A "pure assert_xml! test" = #[test] fn() { body } where body has assert_xml!
// but NO Node::new_ptr (i.e. not code-constructed).
// Usage: node scripts/strip-assert-xml.mjs <file.rs>
import { readFileSync, writeFileSync } from 'node:fs';

const file = process.argv[2];
if (!file) { console.error('Usage: node strip-assert-xml.mjs <file.rs>'); process.exit(1); }

let src = readFileSync(file, 'utf8');
let stripped = 0;

// Match #[test] blocks: #[test]\n fn name() { body }\n}
const re = /(#\[test\][\s\S]*?fn\s+\w+\s*\(\s*\)\s*\{[\s\S]*?\n\s*\}\n)/g;
src = src.replace(re, (match) => {
  const hasXml = /assert_xml!/.test(match);
  const hasCode = /Node::new_ptr|NodeProperties|convert_node_ref|set_style|set_display|set_width|set_height|set_margin|set_flex|set_aspect|set_padding|set_border|set_box_sizing|set_writing_mode|set_min|set_max/.test(match);
  if (hasXml && !hasCode) {
    stripped++;
    return ''; // delete pure assert_xml! test
  }
  return match; // keep code-constructed test
});

writeFileSync(file, src);
console.error(`stripped ${stripped} pure assert_xml! tests from ${file}`);
