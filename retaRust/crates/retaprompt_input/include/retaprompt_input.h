#ifndef RETAPROMPT_INPUT_H
#define RETAPROMPT_INPUT_H

#define RETAPROMPT_INPUT_ABI_GENERATION 2026051902u

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

unsigned int retaprompt_input_abi_generation(void);
int retaprompt_input_run_kind_from_env(int kind);
int retaprompt_input_run_kind_argv(int kind, size_t argc, const char *const *argv);
int retaprompt_input_run_current_executable_from_env(void);
int retaprompt_input_run_any_current_executable_from_env(void);
int retaprompt_input_run_launcher_kind_from_env(int kind);
int retaprompt_input_run_rp_from_env(void);
int retaprompt_input_run_rpl_from_env(void);
int retaprompt_input_run_rpe_from_env(void);
int retaprompt_input_run_rp_argv(size_t argc, const char *const *argv);
int retaprompt_input_run_rpl_argv(size_t argc, const char *const *argv);
int retaprompt_input_run_rpe_argv(size_t argc, const char *const *argv);
char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
void retaprompt_input_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif
