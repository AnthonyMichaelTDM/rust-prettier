#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_type_ts_single_quotetrue_format_1_2de9bd8f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// ref: https://github.com/Microsoft/TypeScript/pull/22592\n\nexport const x: import(\"./foo\") = { x: 0, y: 0 };\n\nexport let y: import(\"./foo2\").Bar.I = { a: \"\", b: 0 };\n\nexport let shim: typeof import(\"./foo2\") = {\n    Bar: Bar2\n};\n\nexport interface Foo {\n    bar: import('immutable').Map<string, int>;\n}\n\ntype X = A<import(\"B\").C<any>>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// ref: https://github.com/Microsoft/TypeScript/pull/22592\n\nexport const x: import('./foo') = { x: 0, y: 0 };\n\nexport let y: import('./foo2').Bar.I = { a: '', b: 0 };\n\nexport let shim: typeof import('./foo2') = {\n  Bar: Bar2,\n};\n\nexport interface Foo {\n  bar: import('immutable').Map<string, int>;\n}\n\ntype X = A<import('B').C<any>>;");
}
#[test]
fn test_import_type_ts_format_1_2de9bd8f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// ref: https://github.com/Microsoft/TypeScript/pull/22592\n\nexport const x: import(\"./foo\") = { x: 0, y: 0 };\n\nexport let y: import(\"./foo2\").Bar.I = { a: \"\", b: 0 };\n\nexport let shim: typeof import(\"./foo2\") = {\n    Bar: Bar2\n};\n\nexport interface Foo {\n    bar: import('immutable').Map<string, int>;\n}\n\ntype X = A<import(\"B\").C<any>>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// ref: https://github.com/Microsoft/TypeScript/pull/22592\n\nexport const x: import(\"./foo\") = { x: 0, y: 0 };\n\nexport let y: import(\"./foo2\").Bar.I = { a: \"\", b: 0 };\n\nexport let shim: typeof import(\"./foo2\") = {\n  Bar: Bar2,\n};\n\nexport interface Foo {\n  bar: import(\"immutable\").Map<string, int>;\n}\n\ntype X = A<import(\"B\").C<any>>;");
}
