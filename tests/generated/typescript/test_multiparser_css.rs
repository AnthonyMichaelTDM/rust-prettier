#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_6259_ts_format_1_cdc57cac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const yesFrame = (\n    ...args: Interpolation<ThemedStyledProps<{}, Theme>>[]\n) => css\\`\n    \\${ChatRoot}[data-frame=\"yes\"] & {\n        \\${css({}, ...args)}\n    }\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const yesFrame = (\n  ...args: Interpolation<ThemedStyledProps<{}, Theme>>[]\n) => css\\`\n  \\${ChatRoot}[data-frame=\"yes\"] & {\n    \\${css({}, ...args)}\n  }\n\\`;");
}
