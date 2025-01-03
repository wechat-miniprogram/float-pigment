// Copyright 2024 wechat-miniprogram. MIT license.
#pragma once

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


namespace float_pigment {
extern "C" {

void array_str_ref_free(Array<StrRef> *x);

void array_warning_free(Array<Warning> *warnings);

void buffer_free(uint8_t *buffer_ptr, size_t buffer_len);

StrRef *css_parser_version();

void generate_warning(CParserHooksContext *self, const char *message);

void inline_style_free(InlineRule *inline_rule);

ColorValue parse_color_from_string(const char *source);

InlineRule *parse_inline_style(const char *inline_style_text_ptr, Array<Warning> **warnings);

Selector *parse_selector_from_string(const char *selector_text_ptr);

StyleSheet *parse_style_sheet_from_string(const char *style_text_ptr);

void selector_free(Selector *selector);

void str_free(const char *ptr);

size_t str_len(const StrRef *self);

const uint8_t *str_ptr(const StrRef *self);

StrRef *style_sheet_bincode_version(uint8_t *buffer_ptr, size_t buffer_len);

void style_sheet_free(StyleSheet *style_sheet);

void style_sheet_import_index_add_bincode(StyleSheetImportIndexPtr *this_,
const char *path,
uint8_t *buffer_ptr,
size_t buffer_len,
void (*drop_fn)(void*),
void *drop_args,
Array<Warning> **warnings);

StyleSheetImportIndexPtr style_sheet_import_index_deserialize_bincode(uint8_t *buffer_ptr,
size_t buffer_len,
void (*drop_fn)(void*),
void *drop_args);

StyleSheetImportIndexPtr style_sheet_import_index_deserialize_json(const char *json);

void style_sheet_import_index_free(StyleSheetImportIndexPtr *this_);

StyleSheet *style_sheet_import_index_get_style_sheet(StyleSheetImportIndexPtr *this_,
const StrRef *path);

Array<StrRef> *style_sheet_import_index_list_dependencies(StyleSheetImportIndexPtr *this_,
const char *path);

Array<StrRef> *style_sheet_import_index_list_dependency(StyleSheetImportIndexPtr *this_,
const char *path);

void style_sheet_import_index_merge_bincode(StyleSheetImportIndexPtr *this_,
uint8_t *buffer_ptr,
size_t buffer_len,
void (*drop_fn)(void*),
void *drop_args);

StyleSheetImportIndexPtr style_sheet_import_index_new();

Array<StrRef> *style_sheet_import_index_query_and_mark_dependencies(StyleSheetImportIndexPtr *this_,
const char *path);

void style_sheet_import_index_remove_bincode(StyleSheetImportIndexPtr *this_, const char *path);

uint8_t *style_sheet_import_index_serialize_bincode(StyleSheetImportIndexPtr *this_,
size_t *ret_buffer_len);

uint8_t *style_sheet_import_index_serialize_json(StyleSheetImportIndexPtr *this_,
size_t *ret_buffer_len);

void style_sheet_resource_add_bincode(StyleSheetResourcePtr *this_,
const char *path,
uint8_t *buffer_ptr,
size_t buffer_len,
void (*drop_fn)(void*),
void *drop_args,
Array<Warning> **warnings);

void style_sheet_resource_add_source(StyleSheetResourcePtr *this_,
const char *path,
const char *source,
Array<Warning> **warnings);

void style_sheet_resource_add_source_with_hooks(StyleSheetResourcePtr *this_,
CParserHooks hooks,
const char *path,
const char *source,
Array<Warning> **warnings);

void style_sheet_resource_add_tag_name_prefix(StyleSheetResourcePtr *this_,
const char *path,
const char *prefix);

Array<StrRef> *style_sheet_resource_direct_dependencies(StyleSheetResourcePtr *this_,
const char *path);

void style_sheet_resource_free(StyleSheetResourcePtr *this_);

StyleSheetImportIndexPtr style_sheet_resource_generate_import_index(StyleSheetResourcePtr *this_);

StyleSheetResourcePtr style_sheet_resource_new();

uint8_t *style_sheet_resource_serialize_bincode(StyleSheetResourcePtr *this_,
const char *path,
size_t *ret_buffer_len);

uint8_t *style_sheet_resource_serialize_json(StyleSheetResourcePtr *this_,
const char *path,
size_t *ret_buffer_len);

const char *substitute_variable(const char *expr_ptr,
void *map,
CustomPropertyGetter getter,
CustomPropertySetter setter);

}  // extern "C"

}  // namespace float_pigment
