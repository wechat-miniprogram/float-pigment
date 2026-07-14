const test = require('node:test')
const assert = require('node:assert')

const binding = require('../index.js')

test('exports the expected API surface', () => {
  assert.strictEqual(typeof binding.compileSync, 'function')
  assert.strictEqual(typeof binding.compileSingleSync, 'function')
  assert.strictEqual(typeof binding.compile, 'function')
  assert.strictEqual(typeof binding.compileSingle, 'function')
  assert.ok(binding.OutputType)
})

test('compileSingleSync compiles valid CSS to json', () => {
  const res = binding.compileSingleSync({
    fileName: 'test.wxss',
    fileContent: Buffer.from('.a { display: flex; color: red; }'),
    outputType: 'json',
  })
  assert.strictEqual(res.warnings.length, 0)
  assert.ok(Buffer.isBuffer(res.content))
  assert.ok(res.content.length > 0)
})

test('compileSingleSync compiles to bincode', () => {
  const res = binding.compileSingleSync({
    fileName: 'test.wxss',
    fileContent: Buffer.from('.b { margin: 0; }'),
    outputType: 'bincode',
  })
  assert.ok(Buffer.isBuffer(res.content))
  assert.ok(res.content.length > 0)
})

test('outputType "none" yields no content but still validates', () => {
  const res = binding.compileSingleSync({
    fileName: 'test.wxss',
    fileContent: Buffer.from('.c { padding: 1px; }'),
    outputType: 'none',
  })
  assert.ok(res.content == null)
  assert.ok(Array.isArray(res.warnings))
})

test('compileSync batches multiple files and builds an import index', () => {
  const res = binding.compileSync({
    src: [
      { path: 'a.wxss', content: Buffer.from('.a { color: red; }') },
      { path: 'b.wxss', content: Buffer.from('.b { color: blue; }') },
    ],
    outputType: 'bincode',
  })
  assert.strictEqual(res.files.length, 2)
  assert.strictEqual(res.files[0].path, 'a.wxss')
  assert.ok(Buffer.isBuffer(res.importIndex))
})

test('compile (async) resolves with the same shape as compileSync', async () => {
  const res = await binding.compile({
    src: [{ path: 'app.wxss', content: Buffer.from('page { margin: 0; }') }],
    outputType: 'json',
  })
  assert.strictEqual(res.files.length, 1)
  assert.ok(Buffer.isBuffer(res.files[0].file.content))
})

test('invalid UTF-8 in content is reported as an error', () => {
  assert.throws(() => {
    binding.compileSingleSync({
      fileName: 'bad.wxss',
      fileContent: Buffer.from([0xff, 0xfe, 0xfd]),
      outputType: 'json',
    })
  })
})
