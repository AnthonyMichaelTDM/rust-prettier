#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_indexed_access_js_trailing_commaall_format_1_640c83a0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const x: Obj['bar'] = 1;\n\ntype A = (T & S)['bar'];\ntype B = (T | S)['bar'];\ntype C = (?T)['bar'];\ntype D = (typeof x)['bar'];\ntype E = (string => void)['bar'];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x: Obj[\"bar\"] = 1;\n\ntype A = (T & S)[\"bar\"];\ntype B = (T | S)[\"bar\"];\ntype C = (?T)[\"bar\"];\ntype D = (typeof x)[\"bar\"];\ntype E = ((string) => void)[\"bar\"];");
}
