#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_proto_props_js_format_1_70097334() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("declare class A { proto: T; }\ndeclare class B { proto x: T; }\ndeclare class C { proto +x: T; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class A {\n  proto: T;\n}\ndeclare class B {\n  proto x: T;\n}\ndeclare class C {\n  proto +x: T;\n}");
}
