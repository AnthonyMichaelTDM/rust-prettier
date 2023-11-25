#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_chain_js_format_1_003fb777() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass A {\n  _property1: number;\n  static _sProperty: number;\n\n  constructor() {\n    this._property1 = 5;\n  }\n  _method1(): number {\n    return 1;\n  }\n  static _sMethod(): string {\n    return \"some string\";\n  }\n}\nA._sProperty = 48;\n\nclass B extends A {\n  _property1: string;\n  static _sProperty: string;\n\n  constructor() {\n    super();\n    this._property1 = \"another string\";\n  }\n  _method1(): string {\n    return \"yet another string\";\n  }\n  static _sMethod(): number {\n    return 23;\n  }\n}\nB._sProperty = \"B._sProperty string\";") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nclass A {\n  _property1: number;\n  static _sProperty: number;\n\n  constructor() {\n    this._property1 = 5;\n  }\n  _method1(): number {\n    return 1;\n  }\n  static _sMethod(): string {\n    return \"some string\";\n  }\n}\nA._sProperty = 48;\n\nclass B extends A {\n  _property1: string;\n  static _sProperty: string;\n\n  constructor() {\n    super();\n    this._property1 = \"another string\";\n  }\n  _method1(): string {\n    return \"yet another string\";\n  }\n  static _sMethod(): number {\n    return 23;\n  }\n}\nB._sProperty = \"B._sProperty string\";");
    Ok(())
}
