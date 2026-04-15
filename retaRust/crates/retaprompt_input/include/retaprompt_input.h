#ifndef RETAPROMPT_INPUT_H
#define RETAPROMPT_INPUT_H

#ifdef __cplusplus
extern "C" {
#endif

int retaprompt_input_run_rp_from_env(void);
int retaprompt_input_run_rpl_from_env(void);
int retaprompt_input_run_rpe_from_env(void);
int retaprompt_input_run_launcher_kind_from_env(int kind);

#ifdef __cplusplus
}
#endif

#endif
