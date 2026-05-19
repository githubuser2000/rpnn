#include "../../crates/retaprompt_input/include/retaprompt_input.h"
#include "../../crates/retaprompt_commands/include/retaprompt_commands.h"

#if defined(__GNUC__) || defined(__clang__)
#define RETAPROMPT_USED __attribute__((used))
#else
#define RETAPROMPT_USED
#endif

typedef int (*retaprompt_command_entrypoint)(void);

/*
 * The interactive prompt frontends execute through libretaprompt_input.so,
 * but they intentionally keep libretaprompt_commands.so as a direct runtime
 * dependency too.  Autocomplete/autosuggest and command execution belong to
 * the prompt split, so the launcher must not collapse to input-only linkage.
 */
RETAPROMPT_USED static retaprompt_command_entrypoint retaprompt_commands_abi_anchor =
    retaprompt_commands_run_rp_from_env;

int main(void) {
    return retaprompt_input_run_rp_from_env();
}
