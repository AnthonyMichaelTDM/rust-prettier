#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_sanity_js_format_1_5cf6d528() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check: shouldn't be allowed to declare a predicate AND use \\`chekcs\\`\n\nfunction check(y): %checks(typeof y === \"string\") {\n  return typeof y === \"number\";\n}\n\ndeclare var y: number | boolean;\n\nif (check(y)) {\n  (y: number);\n}\n\n// Sanity: disallowed body\nfunction indirect_is_number(y): %checks {\n  var y = 1;\n  return typeof y === \"number\";\n}\n\nfunction bak(z: string | number): number {\n  if (indirect_is_number(z)) {\n    return z;\n  } else {\n    return z.length;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check: shouldn't be allowed to declare a predicate AND use \\`chekcs\\`\n\nfunction check(y): %checks(typeof y === \"string\") {\n  return typeof y === \"number\";\n}\n\ndeclare var y: number | boolean;\n\nif (check(y)) {\n  (y: number);\n}\n\n// Sanity: disallowed body\nfunction indirect_is_number(y): %checks {\n  var y = 1;\n  return typeof y === \"number\";\n}\n\nfunction bak(z: string | number): number {\n  if (indirect_is_number(z)) {\n    return z;\n  } else {\n    return z.length;\n  }\n}");
}
#[test]
fn test_sanity_multi_params_js_format_1_c806ac42() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Feature: multi params\nfunction multi_param(w,x,y,z): %checks {\n  return typeof z === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (multi_param(\"1\", \"2\", x, \"3\")) {\n    return x;\n  } else {\n    return x.join();\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Feature: multi params\nfunction multi_param(w, x, y, z): %checks {\n  return typeof z === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (multi_param(\"1\", \"2\", x, \"3\")) {\n    return x;\n  } else {\n    return x.join();\n  }\n}");
}
#[test]
fn test_sanity_ordering_js_format_1_c1b6556f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare var key: string;\ndeclare var obj: { page: ?Object; };\n\nif (dotAccess(obj)) {\n  (obj.page: Object);\n}\n\nfunction dotAccess(head, create) {\n  const path = 'path.location';\n  const stack = path.split('.');\n  do {\n    const key = stack.shift();\n    head = head[key] || create && (head[key] = {});\n  } while (stack.length && head);\n  return head;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare var key: string;\ndeclare var obj: { page: ?Object };\n\nif (dotAccess(obj)) {\n  (obj.page: Object);\n}\n\nfunction dotAccess(head, create) {\n  const path = \"path.location\";\n  const stack = path.split(\".\");\n  do {\n    const key = stack.shift();\n    head = head[key] || (create && (head[key] = {}));\n  } while (stack.length && head);\n  return head;\n}");
}
#[test]
fn test_sanity_unbound_var_js_format_1_456027b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare var y: mixed;\n\n// Sanity check: this should fail, because the preficate function\n// checks \\`y\\` instead of \\`x\\`.\nfunction err(x): %checks {\n  return typeof y === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (err(x)) {\n    return x;\n  } else {\n    return x.join();\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare var y: mixed;\n\n// Sanity check: this should fail, because the preficate function\n// checks \\`y\\` instead of \\`x\\`.\nfunction err(x): %checks {\n  return typeof y === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (err(x)) {\n    return x;\n  } else {\n    return x.join();\n  }\n}");
}
#[test]
fn test_simple_predicate_func_js_format_1_0315b95d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction is_string(y): %checks {\n  return typeof y === \"string\";\n}\n\nfunction is_bool(y): %checks {\n  return typeof y === \"boolean\";\n}\n\nfunction is_number(y): %checks {\n  return typeof y === \"number\";\n}\n\n// Feature check:\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}\n\n// Same as above but refining an offset\nfunction bar(z: { f: string | Array<string>}): string {\n  if (is_string(z.f)) {\n    return z.f;\n  } else {\n    return z.f.join();\n  }\n}\n\nfunction is_number_or_bool(y): %checks {\n  return is_number(y) || is_bool(y);\n}\n\nfunction baz(z: string | number): number {\n  if (is_number_or_bool(z)) {\n    return z;\n  } else {\n    return z.length;\n  }\n}\n\n// Feature: multi params\nfunction multi_param(w,x,y,z): %checks {\n  return typeof z === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (multi_param(\"1\", \"2\", \"3\", x)) {\n    return x;\n  } else {\n    return x.join();\n  }\n}\n\nfunction foo(a, b) {\n  if (two_strings(a, b)) {\n    from_two_strings(a, b);\n  }\n}\n\nfunction two_strings(x,y): %checks {\n  return is_string(x) && is_string(y) ;\n}\n\ndeclare function from_two_strings(x: string, y: string): void;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction is_string(y): %checks {\n  return typeof y === \"string\";\n}\n\nfunction is_bool(y): %checks {\n  return typeof y === \"boolean\";\n}\n\nfunction is_number(y): %checks {\n  return typeof y === \"number\";\n}\n\n// Feature check:\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}\n\n// Same as above but refining an offset\nfunction bar(z: { f: string | Array<string> }): string {\n  if (is_string(z.f)) {\n    return z.f;\n  } else {\n    return z.f.join();\n  }\n}\n\nfunction is_number_or_bool(y): %checks {\n  return is_number(y) || is_bool(y);\n}\n\nfunction baz(z: string | number): number {\n  if (is_number_or_bool(z)) {\n    return z;\n  } else {\n    return z.length;\n  }\n}\n\n// Feature: multi params\nfunction multi_param(w, x, y, z): %checks {\n  return typeof z === \"string\";\n}\n\nfunction foo(x: string | Array<string>): string {\n  if (multi_param(\"1\", \"2\", \"3\", x)) {\n    return x;\n  } else {\n    return x.join();\n  }\n}\n\nfunction foo(a, b) {\n  if (two_strings(a, b)) {\n    from_two_strings(a, b);\n  }\n}\n\nfunction two_strings(x, y): %checks {\n  return is_string(x) && is_string(y);\n}\n\ndeclare function from_two_strings(x: string, y: string): void;");
}
#[test]
fn test_simple_predicate_func_post_js_format_1_bebbeb6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Feature check:\n// The predicate function is defined after the conditional check\n\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Feature check:\n// The predicate function is defined after the conditional check\n\nfunction foo(x: string | Array<string>): string {\n  if (is_string(x)) {\n    // The use of \\`is_string\\` as a conditional check\n    // should guarantee the narrowing of the type of \\`x\\`\n    // to string.\n    return x;\n  } else {\n    // Accordingly the negation of the above check\n    // guarantees that \\`x\\` here is an Array<string>\n    return x.join();\n  }\n}\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}");
}
