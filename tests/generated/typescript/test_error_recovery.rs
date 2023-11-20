#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_generic_ts_trailing_commaall_format_1_8583d589() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n    constructor<>() {}\n}\n\nconst f5 = function<>() {}\n\ninterface f6<> {\n    test<>();\n}\n\nclass f7<> {\n    test<>() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n  constructor<>() {}\n}\n\nconst f5 = function <>() {};\n\ninterface f6<> {\n  test<>();\n}\n\nclass f7<> {\n  test<>() {}\n}");
}
#[test]
fn test_generic_ts_trailing_commaes_5_format_1_8583d589() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n    constructor<>() {}\n}\n\nconst f5 = function<>() {}\n\ninterface f6<> {\n    test<>();\n}\n\nclass f7<> {\n    test<>() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n  constructor<>() {}\n}\n\nconst f5 = function <>() {};\n\ninterface f6<> {\n  test<>();\n}\n\nclass f7<> {\n  test<>() {}\n}");
}
#[test]
fn test_generic_ts_format_1_8583d589() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n    constructor<>() {}\n}\n\nconst f5 = function<>() {}\n\ninterface f6<> {\n    test<>();\n}\n\nclass f7<> {\n    test<>() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "f1<>();\n\nnew f2<>();\n\nfunction f3<>() {}\n\nclass f4 {\n  constructor<>() {}\n}\n\nconst f5 = function <>() {};\n\ninterface f6<> {\n  test<>();\n}\n\nclass f7<> {\n  test<>() {}\n}");
}
#[test]
fn test_index_signature_ts_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_index_signature_ts_trailing_commaall_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_index_signature_ts_trailing_commaall_format_1_dd20c114() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooooooooooooooooooooooooong: string]: string;\n}\ntype TooLong81 = { [loooooooooooooooooooooooooong: string, loooooooooooooooooong: string]: string; }\ntype TooLong80 = { [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string; }\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string]: string;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [\n    loooooooooooooooooooooooooong: string,\n    looooooooooooooooooooooooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong81 = {\n  [\n    loooooooooooooooooooooooooong: string,\n    loooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong80 = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;\n};\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [\n    looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string\n  ]: string;\n};");
}
#[test]
fn test_index_signature_ts_trailing_commaes_5_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_index_signature_ts_trailing_commaes_5_format_1_dd20c114() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooooooooooooooooooooooooong: string]: string;\n}\ntype TooLong81 = { [loooooooooooooooooooooooooong: string, loooooooooooooooooong: string]: string; }\ntype TooLong80 = { [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string; }\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string]: string;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [\n    loooooooooooooooooooooooooong: string,\n    looooooooooooooooooooooooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong81 = {\n  [\n    loooooooooooooooooooooooooong: string,\n    loooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong80 = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;\n};\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [\n    looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string\n  ]: string;\n};");
}
#[test]
fn test_index_signature_ts_format_1_dd20c114() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooooooooooooooooooooooooong: string]: string;\n}\ntype TooLong81 = { [loooooooooooooooooooooooooong: string, loooooooooooooooooong: string]: string; }\ntype TooLong80 = { [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string; }\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string]: string;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = { [key: string] };\n\ntype TwoParams = {\n  [a: string, b: string]: string;\n};\ntype ThreeParams = {\n  [a: string, b: string, c: string]: string;\n};\n\ntype TooLong = {\n  [\n    loooooooooooooooooooooooooong: string,\n    looooooooooooooooooooooooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong81 = {\n  [\n    loooooooooooooooooooooooooong: string,\n    loooooooooooooooooong: string,\n  ]: string;\n};\ntype TooLong80 = {\n  [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;\n};\n\n// note lack of trailing comma in the index signature\ntype TooLongSingleParam = {\n  [\n    looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string\n  ]: string;\n};");
}
#[test]
fn test_jsdoc_only_types_ts_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_jsdoc_only_types_ts_trailing_commaall_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_jsdoc_only_types_ts_trailing_commaall_format_1_ba28118c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);");
}
#[test]
fn test_jsdoc_only_types_ts_trailing_commaes_5_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_jsdoc_only_types_ts_trailing_commaes_5_format_1_ba28118c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);");
}
#[test]
fn test_jsdoc_only_types_ts_format_1_ba28118c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let a: *;\nfunction b(x: ?) {}\nlet c: ?string;\nlet d: string?;\nlet e: ?(string | number);\nlet f: !string;\nlet g: string!;\nlet h: !(string | number);");
}
