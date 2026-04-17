#ifndef RETAPROMPT_COMMANDS_H
#define RETAPROMPT_COMMANDS_H

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
int retaprompt_commands_run_kind_from_env(int kind);
int retaprompt_commands_run_current_executable_from_env(void);
int retaprompt_commands_run_rp_from_env(void);
int retaprompt_commands_run_rpl_from_env(void);
int retaprompt_commands_run_rpb_from_env(void);
int retaprompt_commands_run_rpe_from_env(void);

#ifdef __cplusplus
}
#endif

#endif
