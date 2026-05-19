#ifndef RETA_RETA_SEMANTICS_H
#define RETA_RETA_SEMANTICS_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t reta_semantics_abi_version(void);
uint64_t reta_semantics_abi_anchor(void);
const char *reta_semantics_abi_library_name(void);
const char *reta_semantics_abi_crate_name(void);
const char *reta_semantics_abi_role_de(void);
const char *reta_semantics_abi_role_en(void);
const char *reta_semantics_abi_math_de(void);
const char *reta_semantics_abi_math_en(void);
const char *reta_semantics_abi_manifest_json(void);

char *reta_semantics_choice_counts_json(void);
char *reta_semantics_wahl15_value(const char *key);
char *reta_semantics_wahl16_value(const char *key);
void reta_semantics_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif /* RETA_RETA_SEMANTICS_H */
