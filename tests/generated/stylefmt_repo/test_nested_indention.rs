#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_nested_indention_css_format_1_b07feed0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format (".y-photo-grid\n{\n    display: flex;\n    width: 100%;\n    margin: 0;\n    padding: 0;\n    flex-direction: column;\n    align-items: stretch;\n    justify-content: flex-start;\n\n    &__row\n    {\n        display: flex;\n        width: 100%;\n        margin: 0;\n        padding: 0;\n        flex-direction: row;\n        align-items: center;\n        justify-content: flex-start;\n    }\n\n    &__item\n    {\n        display: block;\n        margin: 0;\n        background: #eee;\n\n        &:first-child\n        {\n            margin-left: 0;\n        }\n    }\n\n    &__link\n    {\n        display: block;\n        height: 100%;\n        width: 100%;\n    }\n\n    &__image\n    {\n        width: 100%;\n        height: 100%;\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".y-photo-grid {\n  display: flex;\n  width: 100%;\n  margin: 0;\n  padding: 0;\n  flex-direction: column;\n  align-items: stretch;\n  justify-content: flex-start;\n\n  &__row {\n    display: flex;\n    width: 100%;\n    margin: 0;\n    padding: 0;\n    flex-direction: row;\n    align-items: center;\n    justify-content: flex-start;\n  }\n\n  &__item {\n    display: block;\n    margin: 0;\n    background: #eee;\n\n    &:first-child {\n      margin-left: 0;\n    }\n  }\n\n  &__link {\n    display: block;\n    height: 100%;\n    width: 100%;\n  }\n\n  &__image {\n    width: 100%;\n    height: 100%;\n  }\n}");
}
