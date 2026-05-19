#include <stddef.h>
extern int reta_retaprompt_input_run_current_executable_from_env(void);
extern int reta_retaprompt_input_run_any_current_executable_from_env(void);
extern int reta_retaprompt_input_run_launcher_kind_from_env(int kind);
extern int reta_retaprompt_input_run_rp_from_env(void);
extern int reta_retaprompt_input_run_rpl_from_env(void);
extern int reta_retaprompt_input_run_rpe_from_env(void);
extern char *reta_retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
extern void reta_retaprompt_input_free_string(char *ptr);

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
    return reta_retaprompt_input_run_current_executable_from_env();
}

int retaprompt_input_run_any_current_executable_from_env(void) {
    return reta_retaprompt_input_run_any_current_executable_from_env();
}

int retaprompt_input_run_launcher_kind_from_env(int kind) {
    return reta_retaprompt_input_run_launcher_kind_from_env(kind);
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

char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor) {
    return reta_retaprompt_input_autosuggestion_at_cursor_json(line, cursor);
}

void retaprompt_input_free_string(char *ptr) {
    reta_retaprompt_input_free_string(ptr);
}
