#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_semifalse_format_1_4e9b38cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let error = new Error(response.statusText);\n// comment\n(error: any).response = response\n\nx;\n\n/* comment */ (error: any).response = response\n\nx;\n\n(error: any).response = response; /* comment */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let error = new Error(response.statusText)\n// comment\n;(error: any).response = response\n\nx\n\n/* comment */ ;(error: any).response = response\n\nx\n\n;(error: any).response = response /* comment */");
}
#[test]
fn test_comments_js_format_1_4e9b38cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let error = new Error(response.statusText);\n// comment\n(error: any).response = response\n\nx;\n\n/* comment */ (error: any).response = response\n\nx;\n\n(error: any).response = response; /* comment */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let error = new Error(response.statusText);\n// comment\n(error: any).response = response;\n\nx;\n\n/* comment */ (error: any).response = response;\n\nx;\n\n(error: any).response = response; /* comment */");
}
#[test]
fn test_flow_class_properties_js_semifalse_format_1_fab6bf1c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  +one = function() {};\n  -two = val();\n  static +three = val();\n  +#privOne = val();\n  static +#privTwo = val();\n  +[computed] = val();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  +one = function () {};\n  -two = val()\n  static +three = val();\n  +#privOne = val()\n  static +#privTwo = val();\n  +[computed] = val()\n}");
}
#[test]
fn test_flow_class_properties_js_format_1_fab6bf1c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  +one = function() {};\n  -two = val();\n  static +three = val();\n  +#privOne = val();\n  static +#privTwo = val();\n  +[computed] = val();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  +one = function () {};\n  -two = val();\n  static +three = val();\n  +#privOne = val();\n  static +#privTwo = val();\n  +[computed] = val();\n}");
}
#[test]
fn test_flow_interfaces_js_semifalse_format_1_032b36ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class A {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare class B {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ndeclare interface C {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare interface D {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ninterface E {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// interface F {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class A {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare class B {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ndeclare interface C {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare interface D {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ninterface E {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// interface F {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }");
}
#[test]
fn test_flow_interfaces_js_format_1_032b36ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class A {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare class B {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ndeclare interface C {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare interface D {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ninterface E {\n    one: boolean;\n    two: { three: string }\n    | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// interface F {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class A {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare class B {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ndeclare interface C {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// declare interface D {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }\n\ninterface E {\n  one: boolean;\n  two: { three: string } | number;\n}\n\n// NOTE: Flow and Babel both fail to apply ASI here\n// interface F {\n//     one: boolean\n//     two: { three: string }\n//     | number\n// }");
}
#[test]
fn test_no_semi_js_semifalse_format_1_9df4c8de() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// flow\n\n(x: void);\n(y: void)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// flow\n\n;(x: void)\n;(y: void)");
}
#[test]
fn test_no_semi_js_format_1_9df4c8de() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// flow\n\n(x: void);\n(y: void)");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// flow\n\n(x: void);\n(y: void);");
}
