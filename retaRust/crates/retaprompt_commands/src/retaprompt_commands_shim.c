#include <stdint.h>

extern int reta_retaprompt_commands_run_rp_from_env(void);
extern int reta_retaprompt_commands_run_rpl_from_env(void);
extern int reta_retaprompt_commands_run_rpb_from_env(void);
extern int reta_retaprompt_commands_run_rpe_from_env(void);

int retaprompt_commands_run_rp_from_env(void) {
    return reta_retaprompt_commands_run_rp_from_env();
}

int retaprompt_commands_run_rpl_from_env(void) {
    return reta_retaprompt_commands_run_rpl_from_env();
}

int retaprompt_commands_run_rpb_from_env(void) {
    return reta_retaprompt_commands_run_rpb_from_env();
}

int retaprompt_commands_run_rpe_from_env(void) {
    return reta_retaprompt_commands_run_rpe_from_env();
}
