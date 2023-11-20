#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_export_js_semifalse_format_1_f7524032() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .semi(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare export default 5;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare export default 5");
}
#[test]
fn test_declare_export_js_format_1_f7524032() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare export default 5;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare export default 5;");
}
#[test]
fn test_declare_type_js_semifalse_format_1_779a7df5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare type A = string;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare type A = string");
}
#[test]
fn test_declare_type_js_format_1_779a7df5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare type A = string;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare type A = string;");
}
#[test]
fn test_declare_var_js_semifalse_format_1_bafc3123() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .semi(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare var bool: React$PropType$Primitive<boolean>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare var bool: React$PropType$Primitive<boolean>"
    );
}
#[test]
fn test_declare_var_js_format_1_bafc3123() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare var bool: React$PropType$Primitive<boolean>;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare var bool: React$PropType$Primitive<boolean>;"
    );
}
#[test]
fn test_long_js_semifalse_format_1_0a99f4af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type AdamPlacementValidationSingleErrorKey =\n  'SOME_FANCY_TARGETS.GLOBAL_TARGET';\n\nexport type SomeReallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey = true;\n\nexport type SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey = null;\n\ntype SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey2 = SomeReallyLongLongLongLongLongLongIdentifier;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type AdamPlacementValidationSingleErrorKey =\n  \"SOME_FANCY_TARGETS.GLOBAL_TARGET\"\n\nexport type SomeReallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey =\n  true\n\nexport type SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey =\n  null\n\ntype SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey2 =\n  SomeReallyLongLongLongLongLongLongIdentifier");
}
#[test]
fn test_long_js_format_1_0a99f4af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type AdamPlacementValidationSingleErrorKey =\n  'SOME_FANCY_TARGETS.GLOBAL_TARGET';\n\nexport type SomeReallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey = true;\n\nexport type SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey = null;\n\ntype SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey2 = SomeReallyLongLongLongLongLongLongIdentifier;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type AdamPlacementValidationSingleErrorKey =\n  \"SOME_FANCY_TARGETS.GLOBAL_TARGET\";\n\nexport type SomeReallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey =\n  true;\n\nexport type SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey =\n  null;\n\ntype SomeOtherLongLongLongLongLongLongLongLongLongLongLongLongLongLongKey2 =\n  SomeReallyLongLongLongLongLongLongIdentifier;");
}
#[test]
fn test_opaque_js_semifalse_format_1_17ea8cc2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .semi(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare export opaque type Foo;\ndeclare export opaque type Bar<T>;\ndeclare export opaque type Baz: Foo;\ndeclare export opaque type Foo1<T>: Bar<T>;\ndeclare export opaque type Foo2<T>: Bar;\ndeclare export opaque type Foo3: Bar<T>;\nopaque type ID = string;\nopaque type Foo4<T> = Bar<T>;\nopaque type Maybe<T> = _Maybe<T, *>;\nexport opaque type Foo5 = number;\nopaque type union =\n | {type: \"A\"}\n | {type: \"B\"};\nopaque type overloads =\n  & ((x: string) => number)\n  & ((x: number) => string);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare export opaque type Foo\ndeclare export opaque type Bar<T>\ndeclare export opaque type Baz: Foo\ndeclare export opaque type Foo1<T>: Bar<T>\ndeclare export opaque type Foo2<T>: Bar\ndeclare export opaque type Foo3: Bar<T>\nopaque type ID = string\nopaque type Foo4<T> = Bar<T>\nopaque type Maybe<T> = _Maybe<T, *>\nexport opaque type Foo5 = number\nopaque type union = { type: \"A\" } | { type: \"B\" }\nopaque type overloads = ((x: string) => number) & ((x: number) => string)");
}
#[test]
fn test_opaque_js_format_1_17ea8cc2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare export opaque type Foo;\ndeclare export opaque type Bar<T>;\ndeclare export opaque type Baz: Foo;\ndeclare export opaque type Foo1<T>: Bar<T>;\ndeclare export opaque type Foo2<T>: Bar;\ndeclare export opaque type Foo3: Bar<T>;\nopaque type ID = string;\nopaque type Foo4<T> = Bar<T>;\nopaque type Maybe<T> = _Maybe<T, *>;\nexport opaque type Foo5 = number;\nopaque type union =\n | {type: \"A\"}\n | {type: \"B\"};\nopaque type overloads =\n  & ((x: string) => number)\n  & ((x: number) => string);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare export opaque type Foo;\ndeclare export opaque type Bar<T>;\ndeclare export opaque type Baz: Foo;\ndeclare export opaque type Foo1<T>: Bar<T>;\ndeclare export opaque type Foo2<T>: Bar;\ndeclare export opaque type Foo3: Bar<T>;\nopaque type ID = string;\nopaque type Foo4<T> = Bar<T>;\nopaque type Maybe<T> = _Maybe<T, *>;\nexport opaque type Foo5 = number;\nopaque type union = { type: \"A\" } | { type: \"B\" };\nopaque type overloads = ((x: string) => number) & ((x: number) => string);");
}
