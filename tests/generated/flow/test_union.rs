use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_js_format_1_ef7ca4db() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const myValue = (callcallcallcallcallcall(87689769876876897698768768976987687689769876):\n                 // Comment\n                 one | two| thre | jdkxhflksjdhfglkjsdhfglkjhsdkfljghskdjhfgkljshdfgkjhsdkljfhgkljshdfgjdfklgjhklj );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const myValue = (callcallcallcallcallcall(\n  87689769876876897698768768976987687689769876,\n): // Comment\n| one\n  | two\n  | thre\n  | jdkxhflksjdhfglkjsdhfglkjhsdkfljghskdjhfgkljshdfgkjhsdkljfhgkljshdfgjdfklgjhklj);");
}
#[test]
fn test_comments_js_format_1_9906b5bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n)[]; // Final comment1\n\ntype Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n) & Bar; // Final comment2") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n)[]; // Final comment1\n\ntype Foo = (\n  | \"thing1\" // Comment1\n  | \"thing2\" // Comment2\n) &\n  Bar; // Final comment2");
}
#[test]
fn test_union_js_format_1_4e88b271() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface RelayProps {\n  articles: Array<{\n    __id: string,\n  } | null> | null | void | 1,\n}\n\ninterface RelayProps2 {\n  articles: Array<{\n    __id: string,\n  } | null> | null | void,\n}\n\nexport function aPrettyLongFunction(aRatherLongParamName: string | null): string {}\n\nexport function aPrettyLongFunctionA(aRatherLongParameterName: {} | null): string[] {}\nexport function aPrettyLongFunctionB(aRatherLongParameterName: Function | null): string[] {}\nexport interface MyInterface {}\nexport function aPrettyLongFunctionC(aRatherLongParameterName: MyInterface | null): string[] {}\nexport type MyType = MyInterface\nexport function aPrettyLongFunctionD(aRatherLongParameterName: MyType | null): string[] {}\n\nexport function aShortFn(aShortParmName: MyType | null): string[] {}\n\nexport function aPrettyLongFunctionE(aRatherLongParameterName: Array<{\n  __id: string,\n} | null> | null | void): string[] {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface RelayProps {\n  articles:\n    | Array<{\n        __id: string,\n      } | null>\n    | null\n    | void\n    | 1;\n}\n\ninterface RelayProps2 {\n  articles: Array<{\n    __id: string,\n  } | null> | null | void;\n}\n\nexport function aPrettyLongFunction(\n  aRatherLongParamName: string | null,\n): string {}\n\nexport function aPrettyLongFunctionA(\n  aRatherLongParameterName: {} | null,\n): string[] {}\nexport function aPrettyLongFunctionB(\n  aRatherLongParameterName: Function | null,\n): string[] {}\nexport interface MyInterface {}\nexport function aPrettyLongFunctionC(\n  aRatherLongParameterName: MyInterface | null,\n): string[] {}\nexport type MyType = MyInterface;\nexport function aPrettyLongFunctionD(\n  aRatherLongParameterName: MyType | null,\n): string[] {}\n\nexport function aShortFn(aShortParmName: MyType | null): string[] {}\n\nexport function aPrettyLongFunctionE(\n  aRatherLongParameterName: Array<{\n    __id: string,\n  } | null> | null | void,\n): string[] {}");
}
#[test]
fn test_within_tuple_js_format_1_3da5a932() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]\n\ntype B = [\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD\n]\n\ntype C = [\n  | [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]\n  | [AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB | CCCCCCCCCCCCCCCCCCCCCC | DDDDDDDDDDDDDDDDDDDDDD]\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = [\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD,\n];\n\ntype B = [\n  | AAAAAAAAAAAAAAAAAAAAAA\n  | BBBBBBBBBBBBBBBBBBBBBB\n  | CCCCCCCCCCCCCCCCCCCCCC\n  | DDDDDDDDDDDDDDDDDDDDDD,\n];\n\ntype C = [\n  | [\n      | AAAAAAAAAAAAAAAAAAAAAA\n      | BBBBBBBBBBBBBBBBBBBBBB\n      | CCCCCCCCCCCCCCCCCCCCCC\n      | DDDDDDDDDDDDDDDDDDDDDD,\n    ]\n  | [\n      | AAAAAAAAAAAAAAAAAAAAAA\n      | BBBBBBBBBBBBBBBBBBBBBB\n      | CCCCCCCCCCCCCCCCCCCCCC\n      | DDDDDDDDDDDDDDDDDDDDDD,\n    ],\n];");
}
