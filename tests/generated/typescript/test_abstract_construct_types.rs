#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_abstract_construct_types_ts_format_1_c8e86828() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type T = abstract new () => void;\ntype T = abstract    new () => void;\ntype T = abstract\n  new () => void;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = abstract new () => void;\ntype T = abstract new () => void;\ntype T = abstract new () => void;");
}
