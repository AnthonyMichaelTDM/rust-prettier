#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_no_semi_ts_semifalse_format_1_ab7f7010() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export class Mutation {\n  private set: NQuad[];\n  private delete: NQuad[];\n}\n\nclass Foo {\n  prop1 = 0;\n  [key: string]: any;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class Mutation {\n  private set: NQuad[]\n  private delete: NQuad[]\n}\n\nclass Foo {\n  prop1 = 0;\n  [key: string]: any\n}");
}
