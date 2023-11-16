#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_value_js_format_1_ab5855fd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var o = {};\no[\"x\"] = 4;\nvar y:string = o[\"x\"];\n\nvar table: { [_: string]: number } = {};\ntable[\"x\"] = \"hello\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o = {};\no[\"x\"] = 4;\nvar y: string = o[\"x\"];\n\nvar table: { [_: string]: number } = {};\ntable[\"x\"] = \"hello\";");
}
