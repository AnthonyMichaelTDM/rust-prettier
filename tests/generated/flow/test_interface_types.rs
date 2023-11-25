use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_interface_types_js_format_1_e2793a77() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = interface { p: string };\ntype B = interface extends X { p: string };\ntype C = interface extends X, Y { p: string };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = interface { p: string };\ntype B = interface extends X { p: string };\ntype C = interface extends X, Y { p: string };");
}
