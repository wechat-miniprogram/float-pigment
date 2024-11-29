#include "../../float_pigment_css.h"
#include <cstdio>

using namespace float_pigment;

char *str_ref_clone(StrRef *sr) {
  char *s = static_cast<char *>(calloc(str_len(sr) + 1, 1));
  memcpy(s, str_ptr(sr), str_len(sr));
  s[str_len(sr)] = '\0';
  return s;
}

int main() {
  const char *css = "color: red; unknown-prop: 1px";

  // compile to json format
  Array<Warning> *warnings = NULL;
  Array<Property> *prop_list_ptr = parse_inline_style(css, &warnings);

  // display warnings (as UTF-8)
  for (int i = 0; i < warnings->len; i += 1) {
    Warning &w = warnings->ptr[i];
    char *msg = str_ref_clone(&w.message);
    printf("%s (from line %d col %d to line %d col %d)\n", msg, w.start_line,
           w.start_col, w.end_line, w.end_col);
    free(msg);
  }
  array_warning_free(warnings);

  // try output some details (for debugging)
  Property *prop_list = prop_list_ptr->ptr;
  uint8_t color_r = prop_list[0].color._0.specified._0;
  uint8_t color_g = prop_list[0].color._0.specified._1;
  uint8_t color_b = prop_list[0].color._0.specified._2;
  uint8_t color_a = prop_list[0].color._0.specified._3;
  printf("color: rgba(%d, %d, %d, %d)\n", color_r, color_g, color_b, color_a);

  // free the json buffer
  inline_style_free(prop_list_ptr);
}
