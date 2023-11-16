#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_chain_js_format_1_003fb777() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass A {\n  _property1: number;\n  static _sProperty: number;\n\n  constructor() {\n    this._property1 = 5;\n  }\n  _method1(): number {\n    return 1;\n  }\n  static _sMethod(): string {\n    return \"some string\";\n  }\n}\nA._sProperty = 48;\n\nclass B extends A {\n  _property1: string;\n  static _sProperty: string;\n\n  constructor() {\n    super();\n    this._property1 = \"another string\";\n  }\n  _method1(): string {\n    return \"yet another string\";\n  }\n  static _sMethod(): number {\n    return 23;\n  }\n}\nB._sProperty = \"B._sProperty string\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass A {\n  _property1: number;\n  static _sProperty: number;\n\n  constructor() {\n    this._property1 = 5;\n  }\n  _method1(): number {\n    return 1;\n  }\n  static _sMethod(): string {\n    return \"some string\";\n  }\n}\nA._sProperty = 48;\n\nclass B extends A {\n  _property1: string;\n  static _sProperty: string;\n\n  constructor() {\n    super();\n    this._property1 = \"another string\";\n  }\n  _method1(): string {\n    return \"yet another string\";\n  }\n  static _sMethod(): number {\n    return 23;\n  }\n}\nB._sProperty = \"B._sProperty string\";");
}
#[test]
fn test_commonjs_export_js_format_1_ad6f5b33() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/* @flow */\n\nclass C {\n  _p: string;\n}\n\nmodule.exports = new C;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nclass C {\n  _p: string;\n}\n\nmodule.exports = new C();"
    );
}
#[test]
fn test_commonjs_import_js_format_1_3aec1212() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\nimport {_p} from \"./commonjs_export\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nimport { _p } from \"./commonjs_export\";"
    );
}
