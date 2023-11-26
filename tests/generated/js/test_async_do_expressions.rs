#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_do_expressions_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_format_1_d8ef2089() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async do {\n  1;\n};\n\n(async do {});\n\nlet x = async do {\n  if (foo()) { f() }\n  else if (bar()) { g() }\n  else { h() }\n};\n\nasync do {\n  await 42\n}\n\nfunction iter() {\n  return async do {\n    return 1;\n  }\n};\n\nlet x = async do {\n  let tmp = f();\n  tmp * tmp + 1\n};") ? ;
    assert_eq ! (formatted , "(async do {\n  1;\n});\n\n(async do {});\n\nlet x = async do {\n  if (foo()) {\n    f();\n  } else if (bar()) {\n    g();\n  } else {\n    h();\n  }\n};\n\n(async do {\n  await 42;\n});\n\nfunction iter() {\n  return async do {\n    return 1;\n  };\n}\n\nlet x = async do {\n  let tmp = f();\n  tmp * tmp + 1;\n};");
    Ok(())
}
