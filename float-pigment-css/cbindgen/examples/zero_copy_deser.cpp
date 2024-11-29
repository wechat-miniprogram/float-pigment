#include "../../float_pigment_css.h"
#include <cstdio>

using namespace float_pigment;

struct BufWithLen {
  uint8_t *buf;
  size_t len;
};

void buf_free_fn(void *buf_with_len) {
  BufWithLen *p = static_cast<BufWithLen *>(buf_with_len);
  buffer_free(p->buf, p->len);
}

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

// global resource
BufWithLen empty_index_bin;
BufWithLen index_bin;
BufWithLen a_wxss_bin;
BufWithLen b_wxss_bin;

// generate global resource for testing
void generate_global_resource() {
  size_t buffer_len;
  uint8_t *buf;
  Array<Warning> *warnings = NULL;
  StyleSheetImportIndexPtr ii;

  const char *a_wxss =
      "@media (width: 100px) { .a { color: red; unknown-prop: 1px; } }";
  const char *b_wxss = "@import url(a);";

  // create new style sheet resource store
  StyleSheetResourcePtr ssr = style_sheet_resource_new();

  // serialize an empty index to bincode
  ii = style_sheet_resource_generate_import_index(&ssr);
  buf = style_sheet_import_index_serialize_bincode(&ii, &buffer_len);
  empty_index_bin.buf = buf;
  empty_index_bin.len = buffer_len;

  // compile a style sheet
  style_sheet_resource_add_source(&ssr, "my/sheet/a.wxss", a_wxss, &warnings);
  display_and_free_warnings(warnings);
  buf = style_sheet_resource_serialize_bincode(&ssr, "my/sheet/a.wxss",
                                               &buffer_len);
  a_wxss_bin.buf = buf;
  a_wxss_bin.len = buffer_len;

  // compile another style sheet
  style_sheet_resource_add_source(&ssr, "my/sheet/b.wxss", b_wxss, &warnings);
  display_and_free_warnings(warnings);
  buf = style_sheet_resource_serialize_bincode(&ssr, "my/sheet/b.wxss",
                                               &buffer_len);
  b_wxss_bin.buf = buf;
  b_wxss_bin.len = buffer_len;

  // generate style sheet index
  ii = style_sheet_resource_generate_import_index(&ssr);
  buf = style_sheet_import_index_serialize_bincode(&ii, &buffer_len);
  index_bin.buf = buf;
  index_bin.len = buffer_len;

  // free the resource store and the style sheet index
  style_sheet_import_index_free(&ii);
  style_sheet_resource_free(&ssr);
}

int main() {
  size_t buffer_len;
  generate_global_resource();

  // load style sheet index
  StyleSheetImportIndexPtr ii = style_sheet_import_index_new();

  // also, indexes are mergable
  style_sheet_import_index_merge_bincode(&ii, index_bin.buf, index_bin.len,
                                         buf_free_fn, &index_bin);

  // query index for which style bin to load
  Array<StrRef> *deps = style_sheet_import_index_query_and_mark_dependencies(
      &ii, "my/sheet/b.wxss");

  // load every dep into resource store
  for (int i = 0; i < deps->len; i++) {
    char *dep = str_ref_clone(&deps->ptr[i]);
    if (strcmp(dep, "my/sheet/a.wxss") == 0) {
      // load dep into style sheet resource store
      style_sheet_import_index_add_bincode(&ii, dep, a_wxss_bin.buf,
                                           a_wxss_bin.len, buf_free_fn,
                                           &a_wxss_bin, NULL);
    } else if (strcmp(dep, "my/sheet/b.wxss") == 0) {
      // load dep into style sheet resource store
      style_sheet_import_index_add_bincode(&ii, dep, b_wxss_bin.buf,
                                           b_wxss_bin.len, buf_free_fn,
                                           &b_wxss_bin, NULL);
    }
    free(dep);
  }
  array_str_ref_free(deps);

  // // link style sheet and return the linked sheets
  Array<StrRef> *sheet_names =
      style_sheet_import_index_list_dependencies(&ii, "my/sheet/b.wxss");
  StyleSheet *sheets[2];
  for (int i = 0; i < sheet_names->len; i++) {
    // get dep from resource store
    sheets[i] =
        style_sheet_import_index_get_style_sheet(&ii, &sheet_names->ptr[i]);
  }
  array_str_ref_free(sheet_names);

  // visit sheets
  for (int i = 0; i < sheet_names->len; i++) {
    // check style sheet format major version (currently only V1 is supported)
    if (sheets[i]->tag == StyleSheet::Tag::V1) {
      Array<Rule> *rules = &sheets[i]->v1._0.rules;
      // for each rule in this sheet
      for (int j = 0; j < rules->len; j++) {
        Array<Property> *props = &rules->ptr[j].properties;
        // for each property in this rule
        for (int k = 0; k < props->len; k++) {
          // output if it is color: xxx (for testing)
          if (props->ptr[k].tag == Property::Tag::Color) {
            ColorType color = props->ptr[k].color._0;
            if (color.tag == ColorType::Tag::Specified) {
              printf("color: rgba(%d, %d, %d, %d)\n", 
                     color.specified._0, color.specified._1, 
                     color.specified._2, color.specified._3);
            }
          }
        }
      }
    }
  }

  // sheet can be removed if not used (or free with the whole index)
  style_sheet_import_index_remove_bincode(&ii, "my/sheet/a.wxss");

  // free the the style sheet index
  style_sheet_import_index_free(&ii);
}
