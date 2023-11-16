#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_update_expression_js_format_1_90bf6b65() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "(this.x++).toString()\nnew (r++);\n(x++)();\nconst uuid = String(this._uuidCounter++);",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "(this.x++).toString();\nnew (r++)();\n(x++)();\nconst uuid = String(this._uuidCounter++);"
    );
}
