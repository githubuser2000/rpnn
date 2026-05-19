#ifndef RETAPROMPT_INPUT_H
#define RETAPROMPT_INPUT_H

#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

int retaprompt_input_run_kind_from_env(int kind);
int retaprompt_input_run_current_executable_from_env(void);
int retaprompt_input_run_any_current_executable_from_env(void);
int retaprompt_input_run_launcher_kind_from_env(int kind);
int retaprompt_input_run_rp_from_env(void);
int retaprompt_input_run_rpl_from_env(void);
int retaprompt_input_run_rpe_from_env(void);
char *retaprompt_input_autosuggestion_at_cursor_json(const char *line, size_t cursor);
void retaprompt_input_free_string(char *ptr);

#ifdef __cplusplus
}
#endif

#endif
