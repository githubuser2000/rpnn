#include "../../crates/retaprompt_commands/include/retaprompt_commands.h"
#include <stdio.h>

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
    int status = retaprompt_check_generation("libretaprompt_commands.so", retaprompt_commands_abi_generation(), RETAPROMPT_COMMANDS_ABI_GENERATION);
    if (status != 0) return status;
    return retaprompt_commands_run_rpb_argv((size_t)argc, (const char *const *)argv);
}
