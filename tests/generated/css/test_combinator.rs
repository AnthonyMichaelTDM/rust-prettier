#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_combinator_css_format_1_5d88cca7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("-Option/root .public/section ~ .public/section:before {\n}\n\n.x .y {}\n.x > .y {}\n.x ~ .y {}\n.x + .y {}\n.x.y {}\n.x     .y {}\n.x\n    .y {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "-Option/root .public/section ~ .public/section:before {\n}\n\n.x .y {\n}\n.x > .y {\n}\n.x ~ .y {\n}\n.x + .y {\n}\n.x.y {\n}\n.x .y {\n}\n.x .y {\n}");
}
#[test]
fn test_leading_css_format_1_e1ce6bce() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a {\n  > * {\n  }\n  &, > * {\n  }\n  + * {\n  }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a {\n  > * {\n  }\n  &,\n  > * {\n  }\n  + * {\n  }\n}"
    );
}
