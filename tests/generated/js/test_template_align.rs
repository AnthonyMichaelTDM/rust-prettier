#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_indent_js_format_1_332ebd6b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("`\nMixed tabs and spaces:\n${() => {a}}\n \t${() => {a}}\n  \t${() => {a}}\n   \t${() => {a}}\n    \t${() => {a}}\n     \t${() => {a}}\n      \t${() => {a}}\n       \t${() => {a}}\n        \t${() => {a}}\n\nTabs:\n\t${() => {a}}\n\t\t${() => {a}}\n`") ? ;
    assert_eq ! (formatted , "`\nMixed tabs and spaces:\n${() => {\n  a;\n}}\n \t${() => {\n    a;\n  }}\n  \t${() => {\n      a;\n    }}\n   \t${() => {\n      a;\n    }}\n    \t${() => {\n        a;\n      }}\n     \t${() => {\n        a;\n      }}\n      \t${() => {\n          a;\n        }}\n       \t${() => {\n          a;\n        }}\n        \t${() => {\n            a;\n          }}\n\nTabs:\n\t${() => {\n    a;\n  }}\n\t\t${() => {\n      a;\n    }}\n`;");
    Ok(())
}
