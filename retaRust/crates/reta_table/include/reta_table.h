#ifndef RETA_RETA_TABLE_H
#define RETA_RETA_TABLE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t reta_table_abi_version(void);
uint64_t reta_table_abi_anchor(void);
const char *reta_table_abi_library_name(void);
const char *reta_table_abi_crate_name(void);
const char *reta_table_abi_role_de(void);
const char *reta_table_abi_role_en(void);
const char *reta_table_abi_math_de(void);
const char *reta_table_abi_math_en(void);
const char *reta_table_abi_manifest_json(void);

char *reta_table_natural_widths_json(const char *tsv_or_lines);
char *reta_table_shrink_widths_json(const char *tsv_or_lines, size_t budget);
void reta_table_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif /* RETA_RETA_TABLE_H */
