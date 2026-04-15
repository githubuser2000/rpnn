#include <stdint.h>

extern int reta_retaprompt_input_run_rp_from_env(void);
extern int reta_retaprompt_input_run_rpl_from_env(void);
extern int reta_retaprompt_input_run_rpe_from_env(void);

int retaprompt_input_run_rp_from_env(void) {
    return reta_retaprompt_input_run_rp_from_env();
}

int retaprompt_input_run_rpl_from_env(void) {
    return reta_retaprompt_input_run_rpl_from_env();
}

int retaprompt_input_run_rpe_from_env(void) {
    return reta_retaprompt_input_run_rpe_from_env();
}
