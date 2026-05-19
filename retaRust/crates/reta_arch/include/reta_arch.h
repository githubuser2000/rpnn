#ifndef RETA_RETA_ARCH_H
#define RETA_RETA_ARCH_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t reta_arch_abi_version(void);
uint64_t reta_arch_abi_anchor(void);
const char *reta_arch_abi_library_name(void);
const char *reta_arch_abi_crate_name(void);
const char *reta_arch_abi_role_de(void);
const char *reta_arch_abi_role_en(void);
const char *reta_arch_abi_math_de(void);
const char *reta_arch_abi_math_en(void);
const char *reta_arch_abi_manifest_json(void);
size_t reta_arch_architecture_morphism_count(void);
size_t reta_arch_architecture_csv_asset_count(void);

#ifdef __cplusplus
}
#endif

#endif /* RETA_RETA_ARCH_H */
