use grundstruk_shared::shared::grundstruk_exact::{I18nLike, grundstruk_html_from_i18n};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let blank = args.len() > 1 && args[1] == "blank";
    let i18n = I18nLike::new();
    print!("{}", grundstruk_html_from_i18n(&i18n, blank));
}
