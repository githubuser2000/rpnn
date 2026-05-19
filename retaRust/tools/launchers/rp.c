#include "../../crates/retaprompt_input/include/retaprompt_input.h"
#include "../../crates/retaprompt_commands/include/retaprompt_commands.h"
#include <stdio.h>

#if defined(__GNUC__) || defined(__clang__)
#define RETAPROMPT_USED __attribute__((used))
#else
#define RETAPROMPT_USED
#endif

typedef int (*retaprompt_command_entrypoint_argv)(size_t argc, const char *const *argv);

/*
 * rrp executes through libretaprompt_input.so, but libretaprompt_commands.so is
 * intentionally retained as a direct dependency.  The launcher contains no
 * prompt behavior; it only checks ABI generations and forwards argc/argv.
 */
RETAPROMPT_USED static retaprompt_command_entrypoint_argv retaprompt_commands_abi_anchor =
    retaprompt_commands_run_rp_argv;

static int retaprompt_check_generation(const char *library_name, unsigned int actual, unsigned int expected) {
    if (actual != expected) {
        fprintf(stderr,
                "%s has ABI generation %u, expected %u. Rebuild with ./build.sh debug or ./build.sh release.\n",
                library_name,
                actual,
                expected);
        return 127;
    }
    return 0;
}

int main(int argc, char **argv) {
    int status = retaprompt_check_generation("libretaprompt_input.so", retaprompt_input_abi_generation(), RETAPROMPT_INPUT_ABI_GENERATION);
    if (status != 0) return status;
    status = retaprompt_check_generation("libretaprompt_commands.so", retaprompt_commands_abi_generation(), RETAPROMPT_COMMANDS_ABI_GENERATION);
    if (status != 0) return status;
    return retaprompt_input_run_rp_argv((size_t)argc, (const char *const *)argv);
}
