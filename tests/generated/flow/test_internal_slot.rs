use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_internal_slot_js_format_1_9b24cbe0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class C1 { static [[foo]]: T }\ndeclare class C2 { [[foo]]: T }\ninterface T1 { [[foo]]: X }\ninterface T2 { [[foo]](): X }\ntype T3 = { [[foo]]: X }\ntype T4 = { [[foo]](): X }\ntype T5 = { [[foo]]?: X }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class C1 {\n  static [[foo]]: T;\n}\ndeclare class C2 {\n  [[foo]]: T;\n}\ninterface T1 {\n  [[foo]]: X;\n}\ninterface T2 {\n  [[foo]](): X;\n}\ntype T3 = { [[foo]]: X };\ntype T4 = { [[foo]](): X };\ntype T5 = { [[foo]]?: X };");
}
