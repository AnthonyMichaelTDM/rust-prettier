#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_ts_format_1_c500bc37() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare module 'autoprefixer';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "declare module \"autoprefixer\";");
}
#[test]
fn test_global_ts_format_1_25c11501() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("namespace global {}\nmodule global {}\nglobal {}\ndeclare global {}\ndeclare /* module */ global {}\ndeclare /* namespace */ global {}\ndeclare module  global {}\ndeclare namespace global {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "namespace global {}\nmodule global {}\nglobal {}\ndeclare global {}\ndeclare /* module */ global {}\ndeclare /* namespace */ global {}\ndeclare module global {}\ndeclare namespace global {}");
}
#[test]
fn test_keyword_ts_format_1_15e5d6c3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("module X {}\n\nmodule X {\n  const x = 1;\n}\n\nmodule X {\n  module X {}\n}\n\nmodule X {\n  module X {\n    const x = 1;\n  }\n}\n\nnamespace X {}\n\nnamespace X {\n  const x = 1;\n}\n\nnamespace X {\n  namespace X {}\n}\n\nnamespace X {\n  namespace X {\n    const x = 1;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "module X {}\n\nmodule X {\n  const x = 1;\n}\n\nmodule X {\n  module X {}\n}\n\nmodule X {\n  module X {\n    const x = 1;\n  }\n}\n\nnamespace X {}\n\nnamespace X {\n  const x = 1;\n}\n\nnamespace X {\n  namespace X {}\n}\n\nnamespace X {\n  namespace X {\n    const x = 1;\n  }\n}");
}
#[test]
fn test_module_nested_ts_format_1_787c6a5d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module abc1.def {}\n\nexport declare module abc2.def {}\n\nexport module abc3.def {}\n\nmodule abc4.def {}\n\ndeclare module abc5.def.ghi {}\n\nexport declare module abc2.def.ghi {}\n\nexport module abc3.def.ghi {}\n\nmodule abc4.def.ghi {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare module abc1.def {}\n\nexport declare module abc2.def {}\n\nexport module abc3.def {}\n\nmodule abc4.def {}\n\ndeclare module abc5.def.ghi {}\n\nexport declare module abc2.def.ghi {}\n\nexport module abc3.def.ghi {}\n\nmodule abc4.def.ghi {}");
}
#[test]
fn test_namespace_function_ts_format_1_24ac58c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("namespace X {\n    declare function f();\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "namespace X {\n  declare function f();\n}");
}
#[test]
fn test_namespace_nested_ts_format_1_dfa89a6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare namespace abc1.def {}\n\nexport declare namespace abc2.def {}\n\nexport namespace abc3.def {}\n\nnamespace abc4.def {}\n\ndeclare namespace abc5.def.ghi {}\n\nexport declare namespace abc2.def.ghi {}\n\nexport namespace abc3.def.ghi {}\n\nnamespace abc4.def.ghi {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare namespace abc1.def {}\n\nexport declare namespace abc2.def {}\n\nexport namespace abc3.def {}\n\nnamespace abc4.def {}\n\ndeclare namespace abc5.def.ghi {}\n\nexport declare namespace abc2.def.ghi {}\n\nexport namespace abc3.def.ghi {}\n\nnamespace abc4.def.ghi {}");
}
