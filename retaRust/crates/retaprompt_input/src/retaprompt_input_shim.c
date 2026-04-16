extern int reta_retaprompt_run_kind_from_env(int kind);
extern int reta_retaprompt_input_run_rp_from_env(void);
extern int reta_retaprompt_input_run_rpl_from_env(void);
extern int reta_retaprompt_input_run_rpe_from_env(void);
extern int retaprompt_commands_run_rpb_from_env(void);

int retaprompt_input_run_kind_from_env(int kind) {
    switch (kind) {
        case 1:
            return reta_retaprompt_input_run_rp_from_env();
        case 2:
            return reta_retaprompt_input_run_rpl_from_env();
        case 4:
            return reta_retaprompt_input_run_rpe_from_env();
        default:
            return 1;
    }
}

int retaprompt_input_run_current_executable_from_env(void) {
    return reta_retaprompt_run_kind_from_env(0);
}

int retaprompt_input_run_any_current_executable_from_env(void) {
    return reta_retaprompt_run_kind_from_env(0);
}

int retaprompt_input_run_launcher_kind_from_env(int kind) {
    switch (kind) {
        case 1:
            return reta_retaprompt_input_run_rp_from_env();
        case 2:
            return reta_retaprompt_input_run_rpl_from_env();
        case 3:
            return retaprompt_commands_run_rpb_from_env();
        case 4:
            return reta_retaprompt_input_run_rpe_from_env();
        default:
            return 1;
    }
}

int retaprompt_input_run_rp_from_env(void) {
    return reta_retaprompt_input_run_rp_from_env();
}

int retaprompt_input_run_rpl_from_env(void) {
    return reta_retaprompt_input_run_rpl_from_env();
}

int retaprompt_input_run_rpe_from_env(void) {
    return reta_retaprompt_input_run_rpe_from_env();
}
