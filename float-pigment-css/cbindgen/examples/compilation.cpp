#include "../../float_pigment_css.h"
#include <cstdio>

using namespace float_pigment;

char *str_ref_clone(StrRef *sr) {
  char *s = static_cast<char *>(calloc(str_len(sr) + 1, 1));
  memcpy(s, str_ptr(sr), str_len(sr));
  s[str_len(sr)] = '\0';
  return s;
}

// try output buffer content (as UTF-8, for debugging)
void debug_output_buf(uint8_t *buf, size_t buffer_len) {
  char *str_buf = static_cast<char *>(calloc(buffer_len + 1, 1));
  memcpy(str_buf, buf, buffer_len);
  str_buf[buffer_len] = '\0';
  printf("%s\n", str_buf);
  free(str_buf);
}

// display warnings (as UTF-8)
void display_and_free_warnings(Array<Warning> *warnings) {
  for (int i = 0; i < warnings->len; i += 1) {
    Warning &w = warnings->ptr[i];
    char *msg = str_ref_clone(&w.message);
    printf("%s (from line %d col %d to line %d col %d)\n", msg, w.start_line,
           w.start_col, w.end_line, w.end_col);
    free(msg);
  }
  array_warning_free(warnings);
}

int main() {
  size_t buffer_len;
  uint8_t *buf;
  Array<Warning> *warnings = NULL;

  const char *a_wxss =
      "@media (width: 100px) { .a { color: red; unknown-prop: 1px; } }";
  const char *b_wxss = "@import url(a);";

  // create new style sheet resource store
  StyleSheetResourcePtr ssr = style_sheet_resource_new();

  // compile a style sheet
  style_sheet_resource_add_source(&ssr, "my/sheet/a.wxss", a_wxss, &warnings);
  display_and_free_warnings(warnings);

  // serialize it into JSON and try output it
  buf =
      style_sheet_resource_serialize_json(&ssr, "my/sheet/a.wxss", &buffer_len);
  debug_output_buf(buf, buffer_len);
  buffer_free(buf, buffer_len);

  // compile another style sheet
  style_sheet_resource_add_source(&ssr, "my/sheet/b.wxss", b_wxss, &warnings);
  display_and_free_warnings(warnings);

  // serialize it into bincode
  buf = style_sheet_resource_serialize_bincode(&ssr, "my/sheet/b.wxss",
                                               &buffer_len);
  buffer_free(buf, buffer_len);

  // generate style sheet index
  StyleSheetImportIndexPtr ii =
      style_sheet_resource_generate_import_index(&ssr);

  // serialize it into JSON and try output it
  buf = style_sheet_import_index_serialize_json(&ii, &buffer_len);
  debug_output_buf(buf, buffer_len);
  buffer_free(buf, buffer_len);

  // get bincode version
  buf = style_sheet_resource_serialize_bincode(&ssr, "my/sheet/a.wxss",
                                               &buffer_len);
  StrRef *version = style_sheet_bincode_version(buf, buffer_len);
  buffer_free(buf, buffer_len);
  const char *v = str_ref_clone(version);
  puts(v);

  // free the resource store and the style sheet index
  style_sheet_import_index_free(&ii);
  style_sheet_resource_free(&ssr);
}
