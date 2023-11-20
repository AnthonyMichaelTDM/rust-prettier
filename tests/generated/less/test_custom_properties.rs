#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_apply_rule_less_format_1_bdad1509() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* http://tabatkins.github.io/specs/css-apply-rule/#defining */\n\n:root {\n  --toolbar-theme: {\n    background-color: hsl(120, 70%, 95%);\n    border-radius: 4px;\n    border: 1px solid var(--theme-color late);\n  };\n  --toolbar-title-theme: {\n    color: green;\n  };\n}\n\n:root {\n  --without-semi: {color:red;}\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-prop: blue;\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-one-like-a-apply-rule: {\n    color:red;\n  };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* http://tabatkins.github.io/specs/css-apply-rule/#defining */\n\n:root {\n  --toolbar-theme: {\n    background-color: hsl(120, 70%, 95%);\n    border-radius: 4px;\n    border: 1px solid var(--theme-color late);\n  };\n  --toolbar-title-theme: {\n    color: green;\n  };\n}\n\n:root {\n  --without-semi: {\n    color: red;\n  };\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-prop: blue;\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-one-like-a-apply-rule: {\n    color:red;\n  };\n}");
}
