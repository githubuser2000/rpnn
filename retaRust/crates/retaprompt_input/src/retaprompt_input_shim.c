extern int retaprompt_commands_run_rp_from_env(void);
extern int retaprompt_commands_run_rpl_from_env(void);
extern int retaprompt_commands_run_rpb_from_env(void);
extern int retaprompt_commands_run_rpe_from_env(void);

int retaprompt_input_run_kind_from_env(int kind) {
    switch (kind) {
        case 1:
            return retaprompt_commands_run_rp_from_env();
        case 2:
            return retaprompt_commands_run_rpl_from_env();
        case 3:
            return retaprompt_commands_run_rpb_from_env();
        case 4:
            return retaprompt_commands_run_rpe_from_env();
        default:
            return 1;
    }
}

int retaprompt_input_run_current_executable_from_env(void) {
    return retaprompt_commands_run_rp_from_env();
}

int retaprompt_input_run_any_current_executable_from_env(void) {
    return retaprompt_commands_run_rp_from_env();
}

int retaprompt_input_run_launcher_kind_from_env(int kind) {
    return retaprompt_input_run_kind_from_env(kind);
}

int retaprompt_input_run_rp_from_env(void) {
    return retaprompt_commands_run_rp_from_env();
}

int retaprompt_input_run_rpl_from_env(void) {
    return retaprompt_commands_run_rpl_from_env();
}

int retaprompt_input_run_rpe_from_env(void) {
    return retaprompt_commands_run_rpe_from_env();
}
