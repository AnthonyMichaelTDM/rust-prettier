use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_fail_0_js_format_1_0f28f4fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Error: 'declare', 'checks' but missing predicate\n\ndeclare function f2(x: mixed): boolean %checks;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Error: 'declare', 'checks' but missing predicate\n\ndeclare function f2(x: mixed): boolean %checks;");
}
#[test]
fn test_fail_1_js_format_1_37c490f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Error: no return statement\n\nfunction f6(x: mixed): %checks (x !== null) {  }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Error: no return statement\n\nfunction f6(x: mixed): %checks(x !== null) {}");
}
#[test]
fn test_fail_2_js_format_1_623e54d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar a2 = (x: mixed): %checks (x !== null) => {        // Error: body form\n  var x = 1; return x;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar a2 = (x: mixed): %checks(x !== null) => {\n  // Error: body form\n  var x = 1;\n  return x;\n};");
}
#[test]
fn test_fail_3_js_format_1_7a7c9804() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Cannot declare predicate with a function body is present.\n\nfunction f5(x: mixed): %checks (x !== null) { return x !== null }\n\nvar a2 = (x: mixed): %checks (x !== null) => x !== null;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Cannot declare predicate with a function body is present.\n\nfunction f5(x: mixed): %checks(x !== null) {\n  return x !== null;\n}\n\nvar a2 = (x: mixed): %checks(x !== null) => x !== null;");
}
#[test]
fn test_pass_js_format_1_16121b5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function f1(x: mixed): boolean;\n\ndeclare function f3(x: mixed): boolean %checks (x !== null);\n\ndeclare function f4(x: mixed): boolean %checks (x !== null);\n\nfunction f7(x: mixed):  %checks { return x !== null }\n\nvar a0 = (x: mixed) => x !== null;\n\nvar a1 = (x: mixed): %checks => x !== null;\n\n(x): %checks => x !== null;\n\nconst insert_a_really_big_predicated_arrow_function_name_here = (x)\n  : %checks => x !== null;\n\ndeclare var x: empty;\n(x)\nchecks => 123;\n\ntype checks = any;\n\ndeclare function f(x: mixed): checks\n(typeof x === null);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare function f1(x: mixed): boolean;\n\ndeclare function f3(x: mixed): boolean %checks(x !== null);\n\ndeclare function f4(x: mixed): boolean %checks(x !== null);\n\nfunction f7(x: mixed): %checks {\n  return x !== null;\n}\n\nvar a0 = (x: mixed) => x !== null;\n\nvar a1 = (x: mixed): %checks => x !== null;\n\n(x): %checks => x !== null;\n\nconst insert_a_really_big_predicated_arrow_function_name_here = (x): %checks =>\n  x !== null;\n\ndeclare var x: empty;\nx;\n(checks) => 123;\n\ntype checks = any;\n\ndeclare function f(x: mixed): checks;\ntypeof x === null;");
}
