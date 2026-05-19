#ifndef RETAPROMPT_COMMANDS_H
#define RETAPROMPT_COMMANDS_H

#define RETAPROMPT_COMMANDS_ABI_GENERATION 2026051902u

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * ABI kind values for retaprompt_commands_run_kind_from_env:
 *   1 = rp
 *   2 = rpl
 *   3 = rpb
 *   4 = rpe
 */
unsigned int retaprompt_commands_abi_generation(void);
int retaprompt_commands_run_kind_from_env(int kind);
int retaprompt_commands_run_kind_argv(int kind, size_t argc, const char *const *argv);
int retaprompt_commands_run_current_executable_from_env(void);
int retaprompt_commands_run_rp_from_env(void);
int retaprompt_commands_run_rpl_from_env(void);
int retaprompt_commands_run_rpb_from_env(void);
int retaprompt_commands_run_rpe_from_env(void);
int retaprompt_commands_run_rp_argv(size_t argc, const char *const *argv);
int retaprompt_commands_run_rpl_argv(size_t argc, const char *const *argv);
int retaprompt_commands_run_rpb_argv(size_t argc, const char *const *argv);
int retaprompt_commands_run_rpe_argv(size_t argc, const char *const *argv);

#ifdef __cplusplus
}
#endif

#endif
