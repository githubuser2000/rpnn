#include <stdio.h>
#include <string.h>


#include "../../crates/reta_render/include/reta_render.h"

static void print_help(void) {
    puts("Usage: rgrundStrukHtml [blank]");
    puts("");
    puts("Generates the Reta Grundstruktur HTML document.");
    puts("");
    puts("Arguments:");
    puts("  blank        generate the blank/table-template variant");
    puts("  -h, --help   show this help");
}

int main(int argc, char **argv) {
    if (argc > 1 && (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0)) {
        print_help();
        return 0;
    }
    if (argc > 2 || (argc > 1 && strcmp(argv[1], "blank") != 0)) {
        fprintf(stderr, "rgrundStrukHtml: unknown argument\n\n");
        print_help();
        return 2;
    }

    const uint8_t blank = (argc > 1 && strcmp(argv[1], "blank") == 0) ? 1 : 0;
    char *html = reta_render_grundstruk_html(blank);
    if (html == NULL) {
        fputs("rgrundStrukHtml: libreta_render.so returned null\n", stderr);
        return 1;
    }

    fputs(html, stdout);
    reta_render_free_string(html);
    return 0;
}
