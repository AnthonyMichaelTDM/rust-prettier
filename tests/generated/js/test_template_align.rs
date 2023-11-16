#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_indent_js_format_1_332ebd6b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\\`\nMixed tabs and spaces:\n\\${() => {a}}\n \t\\${() => {a}}\n  \t\\${() => {a}}\n   \t\\${() => {a}}\n    \t\\${() => {a}}\n     \t\\${() => {a}}\n      \t\\${() => {a}}\n       \t\\${() => {a}}\n        \t\\${() => {a}}\n\nTabs:\n\t\\${() => {a}}\n\t\t\\${() => {a}}\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\nMixed tabs and spaces:\n\\${() => {\n  a;\n}}\n \t\\${() => {\n    a;\n  }}\n  \t\\${() => {\n      a;\n    }}\n   \t\\${() => {\n      a;\n    }}\n    \t\\${() => {\n        a;\n      }}\n     \t\\${() => {\n        a;\n      }}\n      \t\\${() => {\n          a;\n        }}\n       \t\\${() => {\n          a;\n        }}\n        \t\\${() => {\n            a;\n          }}\n\nTabs:\n\t\\${() => {\n    a;\n  }}\n\t\t\\${() => {\n      a;\n    }}\n\\`;");
}
