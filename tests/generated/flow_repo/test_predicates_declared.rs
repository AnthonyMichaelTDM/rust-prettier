#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_function_bind_js_format_1_53130c9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity checks:\n//  - use of bind in a position of a function predicate.\n//    (This case should fall through, as method calls\n//    are currently not supported.) The original behavior\n//    (including \\`havoc\\`) should be retained.\n\nclass C {\n  m() {\n    return true;\n  }\n  a: 1;\n\n  n() {\n    if(this.m.bind(this)) {\n      this.a;\n    }\n  }\n}\n\ndeclare var m: Function;\nconst o = { a: 1 };\n\nif (m.bind(o)) {\n  o.a;\n}\n\n\nclass D {\n  m: Function;\n\n  n() {\n    if(this.m({})) { }\n  }\n}\n\ndeclare var m: Function;\nconst x = \"\";\nif (m.bind(this)(x)) { }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity checks:\n//  - use of bind in a position of a function predicate.\n//    (This case should fall through, as method calls\n//    are currently not supported.) The original behavior\n//    (including \\`havoc\\`) should be retained.\n\nclass C {\n  m() {\n    return true;\n  }\n  a: 1;\n\n  n() {\n    if (this.m.bind(this)) {\n      this.a;\n    }\n  }\n}\n\ndeclare var m: Function;\nconst o = { a: 1 };\n\nif (m.bind(o)) {\n  o.a;\n}\n\nclass D {\n  m: Function;\n\n  n() {\n    if (this.m({})) {\n    }\n  }\n}\n\ndeclare var m: Function;\nconst x = \"\";\nif (m.bind(this)(x)) {\n}");
}
#[test]
fn test_function_union_js_format_1_0f9a432d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function f1(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function f2(x: mixed): boolean %checks(Array.isArray(x));\n\ndeclare var cond: boolean;\n\n// Feature check:\nfunction foo(x: number | string | Array<string>): number {\n\n  var f = (cond) ? f1 : f2;\n\n  if (f(x)) {\n    return x.length;\n  } else {\n    return 1;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare function f1(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function f2(x: mixed): boolean %checks(Array.isArray(x));\n\ndeclare var cond: boolean;\n\n// Feature check:\nfunction foo(x: number | string | Array<string>): number {\n  var f = cond ? f1 : f2;\n\n  if (f(x)) {\n    return x.length;\n  } else {\n    return 1;\n  }\n}");
}
#[test]
fn test_is_string_decl_js_format_1_0c7d739b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function is_string(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function is_number(x: mixed): boolean %checks(typeof x === \"number\");\n\n// Feature check:\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare function is_string(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function is_number(x: mixed): boolean %checks(typeof x === \"number\");\n\n// Feature check:\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}");
}
#[test]
fn test_logical_or_js_format_1_e97dbf88() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check:\n// - conditional functions do not affect behavior of conditional\n//   expressions (e.g. \\`||\\`)\n\ndeclare function r(x: string): number;\nvar s = 'a';\nvar n = r(s) || 1;\n(n: number);\n\nvar x = \"\";\nif (x = r(s) || 1) {\n  (x: number);\n}\n\ndeclare var dollars: mixed;\n\nfunction foo(x: mixed) { return 1; }\n(foo(dollars) || 0);\n\n(Number(dollars) || 0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check:\n// - conditional functions do not affect behavior of conditional\n//   expressions (e.g. \\`||\\`)\n\ndeclare function r(x: string): number;\nvar s = \"a\";\nvar n = r(s) || 1;\n(n: number);\n\nvar x = \"\";\nif ((x = r(s) || 1)) {\n  (x: number);\n}\n\ndeclare var dollars: mixed;\n\nfunction foo(x: mixed) {\n  return 1;\n}\nfoo(dollars) || 0;\n\nNumber(dollars) || 0;");
}
#[test]
fn test_object_invariant_js_format_1_6a9a9520() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check:\n// - preserving \\`havoc\\` semantics\n\ntype Meeting = {\n  organizer: ?Invitee,\n  es: Array<Invitee>\n}\n\ntype Invitee = {\n  fbid: number\n}\n\nfunction f(_this: { m: ?Meeting }): string {\n  if (!_this.m) {\n    return \"0\";\n  }\n\n  if (_this.m.es.some((a) => a.fbid === 0)) {\n\n  }\n  return \"3\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check:\n// - preserving \\`havoc\\` semantics\n\ntype Meeting = {\n  organizer: ?Invitee,\n  es: Array<Invitee>,\n};\n\ntype Invitee = {\n  fbid: number,\n};\n\nfunction f(_this: { m: ?Meeting }): string {\n  if (!_this.m) {\n    return \"0\";\n  }\n\n  if (_this.m.es.some((a) => a.fbid === 0)) {\n  }\n  return \"3\";\n}");
}
#[test]
fn test_orig_string_tag_check_js_format_1_ac4e3661() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// The original first-order case\n\nfunction foo(x: string | Array<string>): string {\n  if (typeof x === \"string\") {\n    return x; // [ERROR] x: Array<string> doesn't match return type\n  }\n  else {\n    return x.join(); // [ERROR] x: string doesn't have .join method\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// The original first-order case\n\nfunction foo(x: string | Array<string>): string {\n  if (typeof x === \"string\") {\n    return x; // [ERROR] x: Array<string> doesn't match return type\n  } else {\n    return x.join(); // [ERROR] x: string doesn't have .join method\n  }\n}");
}
#[test]
fn test_sanity_fall_through_js_format_1_cd05cf1a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check:\n// - we should still be getting an error at the second return statement\n\ndeclare function pred<T>(x: T): boolean;\n\nfunction foo(s: Array<string>): string {\n  if (pred(s)) {\n    return \"1\";\n  }\n  return 1;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check:\n// - we should still be getting an error at the second return statement\n\ndeclare function pred<T>(x: T): boolean;\n\nfunction foo(s: Array<string>): string {\n  if (pred(s)) {\n    return \"1\";\n  }\n  return 1;\n}");
}
#[test]
fn test_sanity_invalid_calls_js_format_1_99390154() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check:\n// - invalid calls at predicate positions\n\ndeclare function pred<T>(x: T): boolean;\n\nfunction foo(s: Array<string>): string {\n\n  if ((1)(s)) {\n    return \"1\";\n  }\n\n  if ((pred + 1)(\"s\")) {\n    return \"1\";\n  }\n\n  return \"1\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check:\n// - invalid calls at predicate positions\n\ndeclare function pred<T>(x: T): boolean;\n\nfunction foo(s: Array<string>): string {\n  if (1(s)) {\n    return \"1\";\n  }\n\n  if ((pred + 1)(\"s\")) {\n    return \"1\";\n  }\n\n  return \"1\";\n}");
}
#[test]
fn test_sanity_is_string_bug_js_format_1_91d34514() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function is_string(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function is_number(x: mixed): boolean %checks(typeof x === \"number\");\n\n// Sanity check:\n// - Erroneous logic\n\nfunction bar(x: string | Array<string>): string {\n  if (is_number(x)) {\n    return x;\n  } else {\n    return x.join();    // error: both string and Array<string> can flow to x\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare function is_string(x: mixed): boolean %checks(typeof x === \"string\");\ndeclare function is_number(x: mixed): boolean %checks(typeof x === \"number\");\n\n// Sanity check:\n// - Erroneous logic\n\nfunction bar(x: string | Array<string>): string {\n  if (is_number(x)) {\n    return x;\n  } else {\n    return x.join(); // error: both string and Array<string> can flow to x\n  }\n}");
}
#[test]
fn test_sanity_parameter_mismatch_js_format_1_e22388ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check: make sure the parameters are checked as usual\n\ndeclare function foo(\n  input: mixed,\n  types: string | Array<string>\n): boolean %checks(typeof input === \"string\" || Array.isArray(input));\n\nfoo(3, 3);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check: make sure the parameters are checked as usual\n\ndeclare function foo(\n  input: mixed,\n  types: string | Array<string>,\n): boolean %checks(typeof input === \"string\" || Array.isArray(input));\n\nfoo(3, 3);");
}
#[test]
fn test_sanity_pred_with_body_js_format_1_c39f287d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check:\n// - predicate functions cannot have bodies (can only be declarations)\n\nfunction pred(x: mixed): boolean %checks(typeof x === \"string\") { // error: cannot use pred type here\n  return typeof x === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (pred(x)) {\n    return x;\n  }\n  return \"1\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check:\n// - predicate functions cannot have bodies (can only be declarations)\n\nfunction pred(x: mixed): boolean %checks(typeof x === \"string\") {\n  // error: cannot use pred type here\n  return typeof x === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (pred(x)) {\n    return x;\n  }\n  return \"1\";\n}");
}
#[test]
fn test_sanity_return_type_js_format_1_bd380065() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\ndeclare function f2(x: mixed): string %checks(Array.isArray(x));");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\ndeclare function f2(x: mixed): string %checks(Array.isArray(x));"
    );
}
