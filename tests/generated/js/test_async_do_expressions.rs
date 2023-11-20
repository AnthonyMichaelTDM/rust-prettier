#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_do_expressions_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_async_do_expressions_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_async_do_expressions_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_async_do_expressions_js_format_1_d8ef2089() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async do {\n  1;\n};\n\n(async do {});\n\nlet x = async do {\n  if (foo()) { f() }\n  else if (bar()) { g() }\n  else { h() }\n};\n\nasync do {\n  await 42\n}\n\nfunction iter() {\n  return async do {\n    return 1;\n  }\n};\n\nlet x = async do {\n  let tmp = f();\n  tmp * tmp + 1\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(async do {\n  1;\n});\n\n(async do {});\n\nlet x = async do {\n  if (foo()) {\n    f();\n  } else if (bar()) {\n    g();\n  } else {\n    h();\n  }\n};\n\n(async do {\n  await 42;\n});\n\nfunction iter() {\n  return async do {\n    return 1;\n  };\n}\n\nlet x = async do {\n  let tmp = f();\n  tmp * tmp + 1;\n};");
}
