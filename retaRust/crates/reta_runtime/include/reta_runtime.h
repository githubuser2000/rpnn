#ifndef RETA_RETA_RUNTIME_H
#define RETA_RETA_RUNTIME_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

uint32_t reta_runtime_abi_version(void);
uint64_t reta_runtime_abi_anchor(void);
const char *reta_runtime_abi_library_name(void);
const char *reta_runtime_abi_crate_name(void);
const char *reta_runtime_abi_role_de(void);
const char *reta_runtime_abi_role_en(void);
const char *reta_runtime_abi_math_de(void);
const char *reta_runtime_abi_math_en(void);
const char *reta_runtime_abi_manifest_json(void);

typedef struct RetaRuntimeCoreFfiResponse {
  char *stdout_text;
  size_t stdout_len;
  char *stderr_text;
  size_t stderr_len;
  int32_t exit_code;
} RetaRuntimeCoreFfiResponse;

/* Private engine ABI used by libreta.so.  External users should prefer include/reta.h. */
int32_t reta_runtime_core_run_and_print_from_env_ffi(void);
uint32_t reta_runtime_core_abi_version(void);
RetaRuntimeCoreFfiResponse reta_runtime_core_run_argv(
  size_t argc,
  const char *const *argv,
  const char *stdin_text,
  size_t terminal_width,
  uint8_t stdout_is_tty,
  uint8_t stderr_is_tty,
  uint8_t stdin_is_tty
);
void reta_runtime_core_free_string(char *ptr);
char *reta_runtime_core_shared_words_json(void);
char *reta_runtime_core_all_main_alias_groups_json(void);
char *reta_runtime_core_parameter_alias_groups_for_main_json(const char *canonical_main);

#ifdef __cplusplus
}
#endif

#endif /* RETA_RETA_RUNTIME_H */
