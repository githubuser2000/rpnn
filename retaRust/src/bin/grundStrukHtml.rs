use reta_shared::shared::exact_i18n::I18nExact;
use reta_shared::shared::grundstruk_shared::grundstruk_html_from_i18n;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let blank = args.len() > 1 && args[1] == "blank";
    let i18n = I18nExact::from_python_evaluated_shapes_subset();
    print!("{}", grundstruk_html_from_i18n(&i18n, blank));
}
