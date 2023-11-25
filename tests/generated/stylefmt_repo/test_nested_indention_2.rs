use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nested_indention_2_css_format_1_4004e8ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".y-photo-grid{\n    display: flex;\n    width: 100%;\n    margin: 0;\n    padding: 0;\n    flex-direction: column;\n    align-items: stretch;\n    justify-content: flex-start;\n\n    &__row, &__item, &__link, &__image{\n        display: flex;\n        width: 100%;\n        margin: 0;\n        padding: 0;\n        flex-direction: row;\n        align-items: center;\n        justify-content: flex-start;\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".y-photo-grid {\n  display: flex;\n  width: 100%;\n  margin: 0;\n  padding: 0;\n  flex-direction: column;\n  align-items: stretch;\n  justify-content: flex-start;\n\n  &__row,\n  &__item,\n  &__link,\n  &__image {\n    display: flex;\n    width: 100%;\n    margin: 0;\n    padding: 0;\n    flex-direction: row;\n    align-items: center;\n    justify-content: flex-start;\n  }\n}");
}
