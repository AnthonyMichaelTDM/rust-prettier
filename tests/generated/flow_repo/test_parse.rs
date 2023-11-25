#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_no_parse_error_js_format_1_6c7117b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n@flow\n*/\n\nvar x = 'Test';\nvar y = 5 / x;\n\nvar z: {\n    type: number,\n    y: string\n} = {type: 1, y: 'hey'};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n@flow\n*/\n\nvar x = \"Test\";\nvar y = 5 / x;\n\nvar z: {\n  type: number,\n  y: string,\n} = { type: 1, y: \"hey\" };");
}
