#ifndef RETA_RETA_DATA_H
#define RETA_RETA_DATA_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t reta_data_abi_version(void);
uint64_t reta_data_abi_anchor(void);
const char *reta_data_abi_library_name(void);
const char *reta_data_abi_crate_name(void);
const char *reta_data_abi_role_de(void);
const char *reta_data_abi_role_en(void);
const char *reta_data_abi_math_de(void);
const char *reta_data_abi_math_en(void);
const char *reta_data_abi_manifest_json(void);

size_t reta_data_words_entry_count(void);
char *reta_data_shared_words_json(void);
char *reta_data_all_main_alias_groups_json(void);
char *reta_data_parameter_alias_groups_for_main_json(const char *canonical_main);
char *reta_data_resolve_parameter_main_alias(const char *main_alias);
void reta_data_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif /* RETA_RETA_DATA_H */
