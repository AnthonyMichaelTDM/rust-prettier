#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_apply_rule_scss_format_1_bdad1509() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* http://tabatkins.github.io/specs/css-apply-rule/#defining */\n\n:root {\n  --toolbar-theme: {\n    background-color: hsl(120, 70%, 95%);\n    border-radius: 4px;\n    border: 1px solid var(--theme-color late);\n  };\n  --toolbar-title-theme: {\n    color: green;\n  };\n}\n\n:root {\n  --without-semi: {color:red;}\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-prop: blue;\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-one-like-a-apply-rule: {\n    color:red;\n  };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* http://tabatkins.github.io/specs/css-apply-rule/#defining */\n\n:root {\n  --toolbar-theme: {\n    background-color: hsl(120, 70%, 95%);\n    border-radius: 4px;\n    border: 1px solid var(--theme-color late);\n  };\n  --toolbar-title-theme: {\n    color: green;\n  };\n}\n\n:root {\n  --without-semi: {\n    color: red;\n  };\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-prop: blue;\n}\n\n:root {\n  --like-a-apply-rule: {\n  color:red;} /* no semi here*/\n  --another-one-like-a-apply-rule: {\n    color:red;\n  };\n}");
}
#[test]
fn test_postcss_8_improment_scss_format_1_3a47d3ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\nThis test is copied from \\`postcss@8\\` release note\n\nhttps://github.com/postcss/postcss/releases/tag/8.0.0\n*/\n\n:root {\n  --empty: ;\n  --JSON: [1, \"2\", {\"three\": {\"a\":1}}, [4]];\n  --javascript: function(rule) { console.log(rule) };\n}\n\n@supports (--element(\".minwidth\", { \"minWidth\": 300 })) {\n  [--self] {\n    background: greenyellow;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\nThis test is copied from \\`postcss@8\\` release note\n\nhttps://github.com/postcss/postcss/releases/tag/8.0.0\n*/\n\n:root {\n  --empty: ;\n  --JSON: [1, \"2\", {\"three\": {\"a\": 1}}, [4]];\n  --javascript: function(rule) {console.log(rule)};\n}\n\n@supports (--element(\".minwidth\", {\"minWidth\": 300})) {\n  [--self] {\n    background: greenyellow;\n  }\n}");
}
#[test]
fn test_variables_scss_format_1_dad4aeac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n  --#{$prop}: 10px;\n  #{$prop}: 10px;\n  prop1: var(--#{$var});\n  prop2: var(#{$var}, --my-#{$var}, pink);\n  prop3: calc(var(--#{$var}) * 1px);\n  prop4: var(--spacer#{(1) + 2});\n}\n\n@supports (--#{$prop}: green) {\n  body {\n    color: var(--#{$var});\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  --#{$prop}: 10px;\n  #{$prop}: 10px;\n  prop1: var(--#{$var});\n  prop2: var(#{$var}, --my-#{$var}, pink);\n  prop3: calc(var(--#{$var}) * 1px);\n  prop4: var(--spacer#{(1) + 2});\n}\n\n@supports (--#{$prop}: green) {\n  body {\n    color: var(--#{$var});\n  }\n}");
}
