#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_call_signature_ts_format_1_cd56462d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = {\n  (): void;\n  second: string;\n};\n\ntype T = {\n  (): void; // prettier-ignore\n  second: string;\n};\n\ntype T = {\n  (): void; // comment\n  second: string;\n};\n\ntype T = {\n  first: string;\n  (): void;\n};\n\ntype T = {\n  first: string;\n  (): void; // prettier-ignore\n};\n\ntype T = {\n  first: string;\n  (): void; // comment\n};\n\ninterface I {\n  (): void;\n  second: string;\n}\n\ninterface I {\n  (): void; // prettier-ignore\n  second: string;\n}\n\ninterface I {\n  (): void; // comment\n  second: string;\n}\n\ninterface I {\n  first: string;\n  (): void;\n}\n\ninterface I {\n  first: string;\n  (): void; // prettier-ignore\n}\n\ninterface I {\n  first: string;\n  (): void; // comment\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = {\n  (): void;\n  second: string;\n};\n\ntype T = {\n  (): void; // prettier-ignore\n  second: string;\n};\n\ntype T = {\n  (): void; // comment\n  second: string;\n};\n\ntype T = {\n  first: string;\n  (): void;\n};\n\ntype T = {\n  first: string;\n  (): void; // prettier-ignore\n};\n\ntype T = {\n  first: string;\n  (): void; // comment\n};\n\ninterface I {\n  (): void;\n  second: string;\n}\n\ninterface I {\n  (): void; // prettier-ignore\n  second: string;\n}\n\ninterface I {\n  (): void; // comment\n  second: string;\n}\n\ninterface I {\n  first: string;\n  (): void;\n}\n\ninterface I {\n  first: string;\n  (): void; // prettier-ignore\n}\n\ninterface I {\n  first: string;\n  (): void; // comment\n}");
}
