#![allow(non_snake_case)]

use reta::shared::grundstruk_exact::{grundstruk_html_from_i18n, I18nLike};

fn main() {
    let blank = std::env::args()
        .nth(1)
        .as_deref()
        .map(|arg| arg == "blank")
        .unwrap_or(false);
    print!("{}", grundstruk_html_from_i18n(&I18nLike::new(), blank));
}
