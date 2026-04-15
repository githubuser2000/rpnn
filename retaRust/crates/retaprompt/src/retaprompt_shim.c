#include <stdint.h>

extern int reta_retaprompt_run_kind_from_env(int32_t kind);
extern int reta_retaprompt_run_auto_from_env(void);
extern int reta_retaprompt_run_rp_from_env(void);
extern int reta_retaprompt_run_rpl_from_env(void);
extern int reta_retaprompt_run_rpb_from_env(void);
extern int reta_retaprompt_run_rpe_from_env(void);

int retaprompt_run_kind_from_env(int32_t kind) {
    return reta_retaprompt_run_kind_from_env(kind);
}

int retaprompt_run_auto_from_env(void) {
    return reta_retaprompt_run_auto_from_env();
}

int retaprompt_run_rp_from_env(void) {
    return reta_retaprompt_run_rp_from_env();
}

int retaprompt_run_rpl_from_env(void) {
    return reta_retaprompt_run_rpl_from_env();
}

int retaprompt_run_rpb_from_env(void) {
    return reta_retaprompt_run_rpb_from_env();
}

int retaprompt_run_rpe_from_env(void) {
    return reta_retaprompt_run_rpe_from_env();
}
