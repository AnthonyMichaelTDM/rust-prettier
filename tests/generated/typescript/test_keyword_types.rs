#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_conditional_types_ts_format_1_d938b3ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type UnwrappedResultRow<T> = {\n    [P in keyof T]: (\n        T[P] extends Req<infer a> ? (\n            a\n        ) : (\n            T[P] extends Opt<infer b> ? (\n                b\n            ) : (\n                // TEST\n                never\n            )\n        )\n    );\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type UnwrappedResultRow<T> = {\n  [P in keyof T]: T[P] extends Req<infer a>\n    ? a\n    : T[P] extends Opt<infer b>\n      ? b\n      : // TEST\n        never;\n};");
}
#[test]
fn test_keyword_types_with_parens_comments_ts_format_1_1cb6e029() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let foo: (\n    // comment\n    any\n);\nlet foo: (\n    // comment\n    null\n);\nlet foo: (\n    // comment\n    this\n);\nlet foo: (\n    // comment\n    number\n);\nlet foo: (\n    // comment\n    void\n);\nlet foo: (\n    // comment\n    boolean\n);\nlet foo: (\n    // comment\n    bigint\n);\nlet foo: (\n    // comment\n    symbol\n);\nlet foo: (\n    // comment\n    string\n);\nlet foo: (\n    // comment\n    never\n);\nlet foo: (\n    // comment\n    object\n);\nlet foo: (\n    // comment\n    undefined\n);\nlet foo: (\n    // comment\n    unknown\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let foo: // comment\nany;\nlet foo: // comment\nnull;\nlet foo: // comment\nthis;\nlet foo: // comment\nnumber;\nlet foo: // comment\nvoid;\nlet foo: // comment\nboolean;\nlet foo: // comment\nbigint;\nlet foo: // comment\nsymbol;\nlet foo: // comment\nstring;\nlet foo: // comment\nnever;\nlet foo: // comment\nobject;\nlet foo: // comment\nundefined;\nlet foo: // comment\nunknown;");
}
