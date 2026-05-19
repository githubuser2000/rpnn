#include <stdio.h>
#include <string.h>

#include "../../crates/reta_render/include/reta_render.h"

int main(int argc, char **argv) {
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
