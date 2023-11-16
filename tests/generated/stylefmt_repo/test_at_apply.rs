#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_at_apply_css_format_1_3a22f0de() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(".toolbar{@apply   --toolbar-theme   ;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, ".toolbar {\n  @apply --toolbar-theme;\n}");
}
