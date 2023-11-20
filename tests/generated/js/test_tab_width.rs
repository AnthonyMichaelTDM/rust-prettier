#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_tab_width_4_format_1_038c69eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  method() {\n    var x = 1;\n    while(typeof x == \"number\" || typeof x == \"string\") {\n        x = x + 1;\n        if (true) x = \"\";\n    }\n    var z = x;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n    method() {\n        var x = 1;\n        while (typeof x == \"number\" || typeof x == \"string\") {\n            x = x + 1;\n            if (true) x = \"\";\n        }\n        var z = x;\n    }\n}");
}
#[test]
fn test_class_js_format_1_038c69eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n  method() {\n    var x = 1;\n    while(typeof x == \"number\" || typeof x == \"string\") {\n        x = x + 1;\n        if (true) x = \"\";\n    }\n    var z = x;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  method() {\n    var x = 1;\n    while (typeof x == \"number\" || typeof x == \"string\") {\n      x = x + 1;\n      if (true) x = \"\";\n    }\n    var z = x;\n  }\n}");
}
#[test]
fn test_nested_functions_spec_js_tab_width_4_format_1_d2844d83() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const c = () => {};\n\nfunction a() {\n  return function b() {\n    return () => {\n      return function() {\n        return c;\n      }\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const c = () => {};\n\nfunction a() {\n    return function b() {\n        return () => {\n            return function () {\n                return c;\n            };\n        };\n    };\n}");
}
#[test]
fn test_nested_functions_spec_js_format_1_d2844d83() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const c = () => {};\n\nfunction a() {\n  return function b() {\n    return () => {\n      return function() {\n        return c;\n      }\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const c = () => {};\n\nfunction a() {\n  return function b() {\n    return () => {\n      return function () {\n        return c;\n      };\n    };\n  };\n}");
}
