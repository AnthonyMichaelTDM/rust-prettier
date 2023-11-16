#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_at_rule_with_comments_scss_format_1_52ba49cd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format (".ag-theme-balham {\n    @include ag-theme-balham(\n        (\n            foreground-color: $custom-foreground-color,\n            disabled-foreground-color: null, // TODO some comment\n        )\n    );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".ag-theme-balham {\n  @include ag-theme-balham(\n    (\n      foreground-color: $custom-foreground-color,\n      disabled-foreground-color: null,\n      // TODO some comment\n    )\n  );\n}");
}
