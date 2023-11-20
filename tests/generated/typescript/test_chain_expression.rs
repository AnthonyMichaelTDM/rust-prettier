#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_ts_format_1_157c658e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(a?.b!).c;\n(a?.()!).b;\n(a?.b)!.c;\n(a?.())!.b;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(a?.b)!.c;\n(a?.())!.b;\n(a?.b)!.c;\n(a?.())!.b;"
    );
}
