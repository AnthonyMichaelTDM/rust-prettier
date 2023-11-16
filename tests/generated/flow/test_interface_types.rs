#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_interface_types_js_format_1_e2793a77() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type A = interface { p: string };\ntype B = interface extends X { p: string };\ntype C = interface extends X, Y { p: string };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = interface { p: string };\ntype B = interface extends X { p: string };\ntype C = interface extends X, Y { p: string };");
}
