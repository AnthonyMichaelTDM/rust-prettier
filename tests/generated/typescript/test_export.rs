use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_ts_format_1_bf0f0462() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export function match(): string /* the matching pattern */\na");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export function match(): string; /* the matching pattern */\na;"
    );
}
#[test]
fn test_default_ts_format_1_7569fe6d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export default interface Foo {\n  readonly bar?: string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export default interface Foo {\n  readonly bar?: string;\n}"
    );
}
#[test]
fn test_export_ts_format_1_c4414b49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module \"hello\" {\n    export default Hello;\n}\n\ndeclare module \"hello\" {\n    export = Hello;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare module \"hello\" {\n  export default Hello;\n}\n\ndeclare module \"hello\" {\n  export = Hello;\n}");
}
#[test]
fn test_export_as_ns_ts_format_1_92a42b90() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export * as utilities from \"./utilities.js\";");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export * as utilities from \"./utilities.js\";");
}
#[test]
fn test_export_class_ts_format_1_0f2b979a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class A {}\nexport default class B {}\nexport abstract class C {}\nexport default abstract class D {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class A {}\nexport default class B {}\nexport abstract class C {}\nexport default abstract class D {}");
}
#[test]
fn test_export_type_star_from_ts_format_1_1a5f5104() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export type * from './mod';\nexport type * as ns from './mod';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export type * from \"./mod\";\nexport type * as ns from \"./mod\";"
    );
}
#[test]
fn test_export_type_star_from_2_ts_babel_ts_format_1_dfeb050b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Note: TSC doesn't support string module specifiers yet,\n// but it's easier for us to support them than not.\nexport type * as \"ns2\" from './mod';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Note: TSC doesn't support string module specifiers yet,\n// but it's easier for us to support them than not.\nexport type * as \"ns2\" from \"./mod\";");
}
#[test]
fn test_export_type_star_from_2_ts_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
