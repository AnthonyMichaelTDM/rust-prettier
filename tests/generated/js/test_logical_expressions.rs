#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_issue_7024_js_format_1_e271231f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const radioSelectedAttr =\n  (isAnyValueSelected &&\n    node.getAttribute(radioAttr.toLowerCase()) === radioValue) ||\n  ((!isAnyValueSelected && values[a].default === true) || a === 0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const radioSelectedAttr =\n  (isAnyValueSelected &&\n    node.getAttribute(radioAttr.toLowerCase()) === radioValue) ||\n  (!isAnyValueSelected && values[a].default === true) ||\n  a === 0;");
}
#[test]
fn test_logical_expression_operators_js_format_1_6bbca744() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Same operators do not require parens\n(foo && bar) && baz;\nfoo && (bar && baz);\nfoo && ((bar && baz) && qux);\nfoo && (bar && (baz && qux));\nfoo && (bar && ((baz && qux) && xyz));\nfoo && (bar && (baz && (qux && xyz)));\n\n(foo || bar) || baz;\nfoo || (bar || baz);\nfoo || ((bar || baz) || qux);\nfoo || (bar || (baz || qux));\nfoo || (bar || ((baz || qux) || xyz));\nfoo || (bar || (baz || (qux || xyz)));\n\n(foo ?? bar) ?? baz;\nfoo ?? (bar ?? baz);\nfoo ?? ((bar ?? baz) ?? qux);\nfoo ?? (bar ?? (baz ?? qux));\nfoo ?? (bar ?? ((baz ?? qux) ?? xyz));\nfoo ?? (bar ?? (baz ?? (qux ?? xyz)));\n\n// Explicitly parenthesized && and || requires parens\n(foo && bar) || baz;\n(foo || bar) && baz;\n\nfoo && (bar || baz);\nfoo || (bar && baz);\n\n// Implicitly parenthesized && and || requires parens\nfoo && bar || baz;\nfoo || bar && baz;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Same operators do not require parens\nfoo && bar && baz;\nfoo && bar && baz;\nfoo && bar && baz && qux;\nfoo && bar && baz && qux;\nfoo && bar && baz && qux && xyz;\nfoo && bar && baz && qux && xyz;\n\nfoo || bar || baz;\nfoo || bar || baz;\nfoo || bar || baz || qux;\nfoo || bar || baz || qux;\nfoo || bar || baz || qux || xyz;\nfoo || bar || baz || qux || xyz;\n\nfoo ?? bar ?? baz;\nfoo ?? bar ?? baz;\nfoo ?? bar ?? baz ?? qux;\nfoo ?? bar ?? baz ?? qux;\nfoo ?? bar ?? baz ?? qux ?? xyz;\nfoo ?? bar ?? baz ?? qux ?? xyz;\n\n// Explicitly parenthesized && and || requires parens\n(foo && bar) || baz;\n(foo || bar) && baz;\n\nfoo && (bar || baz);\nfoo || (bar && baz);\n\n// Implicitly parenthesized && and || requires parens\n(foo && bar) || baz;\nfoo || (bar && baz);");
}
