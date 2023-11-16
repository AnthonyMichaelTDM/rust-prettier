#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_argument_expansion_js_format_1_f02b4b24() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const bar1 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([]: Array<string>));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const bar1 = [1, 2, 3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([]: Array<string>));");
}
#[test]
fn test_expression_js_format_1_d213ea43() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let x: string = (foo: string);\n\n// https://github.com/prettier/prettier/issues/3936\nconst foo = ((1?2:3): number);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let x: string = (foo: string);\n\n// https://github.com/prettier/prettier/issues/3936\nconst foo = ((1 ? 2 : 3): number);");
}
#[test]
fn test_statement_js_format_1_e208dd80() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("foo: string;\nbar: number;\n(foo.bar: SomeType);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "foo: string;\nbar: number;\n(foo.bar: SomeType);"
    );
}
