#ifndef RETAPROMPT_H
#define RETAPROMPT_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * ABI kind values for retaprompt_run_kind_from_env:
 *   0 = auto
 *   1 = rp
 *   2 = rpl
 *   3 = rpb
 *   4 = rpe
 */
int retaprompt_run_kind_from_env(int kind);
int retaprompt_run_auto_from_env(void);
int retaprompt_run_rp_from_env(void);
int retaprompt_run_rpl_from_env(void);
int retaprompt_run_rpb_from_env(void);
int retaprompt_run_rpe_from_env(void);

#ifdef __cplusplus
}
#endif

#endif
