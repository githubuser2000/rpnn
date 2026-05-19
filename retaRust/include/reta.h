#ifndef RETA_H
#define RETA_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct RetaFfiResponse {
  char *stdout_text;
  size_t stdout_len;
  char *stderr_text;
  size_t stderr_len;
  int32_t exit_code;
} RetaFfiResponse;

uint32_t reta_abi_version(void);
RetaFfiResponse reta_run_argv(
  size_t argc,
  const char *const *argv,
  const char *stdin_text,
  size_t terminal_width,
  uint8_t stdout_is_tty,
  uint8_t stderr_is_tty,
  uint8_t stdin_is_tty
);
void reta_free_string(char *ptr);
int32_t reta_run_and_print_from_env_ffi(void);

uint32_t reta_core_split_abi_version(void);
uint64_t reta_core_split_abi_anchor(void);
uint8_t reta_core_split_abi_is_linked(void);
const char *reta_core_split_abi_manifest_json(void);

#ifdef __cplusplus
}
#endif

#endif /* RETA_H */
