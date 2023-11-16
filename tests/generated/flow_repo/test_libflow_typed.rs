#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_libtest_js_format_1_dd6a7bca() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* @flow */\nconst dino : Dinosaur = \"Stegosaurus\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\nconst dino: Dinosaur = \"Stegosaurus\";"
    );
}
