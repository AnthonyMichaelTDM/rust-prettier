use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_c7bd9545() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar a = [];\nfor (var i = 0; i < 10; ++i) {\n    if (i % 2 == 0) { a[i] = 0; }\n    else { a[i] = ''; };\n}\n\n// \\`i\\` never gets a lower bound, so the array access is stalled until the\n// function is called.\nfunction foo(i): string { return a[i]; }\n\n// here, because we call \\`bar\\`, we the array access constraint is discharged and\n// we realize a type error.\nfunction bar(i): string { return a[i]; }\nbar(0);\n\n// annotations suffice to unblock the access constraint as well, so only\n// uncalled internal functions will not find a type error, which is acceptable\n// behavior as such functions are dead code.\nfunction baz(i:number): string { return a[i]; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nvar a = [];\nfor (var i = 0; i < 10; ++i) {\n  if (i % 2 == 0) {\n    a[i] = 0;\n  } else {\n    a[i] = \"\";\n  }\n}\n\n// \\`i\\` never gets a lower bound, so the array access is stalled until the\n// function is called.\nfunction foo(i): string {\n  return a[i];\n}\n\n// here, because we call \\`bar\\`, we the array access constraint is discharged and\n// we realize a type error.\nfunction bar(i): string {\n  return a[i];\n}\nbar(0);\n\n// annotations suffice to unblock the access constraint as well, so only\n// uncalled internal functions will not find a type error, which is acceptable\n// behavior as such functions are dead code.\nfunction baz(i: number): string {\n  return a[i];\n}");
}
#[test]
fn test_array_2_js_format_1_6489ba6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar a = [];\nfor (var i = 0; i < 10; ++i) {\n    if (i % 2 == 0) { a[i] = 0; }\n    else { a[i] = ''; };\n}\n\nfunction foo(i: number): string { return a[i]; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\nvar a = [];\nfor (var i = 0; i < 10; ++i) {\n  if (i % 2 == 0) {\n    a[i] = 0;\n  } else {\n    a[i] = \"\";\n  }\n}\n\nfunction foo(i: number): string {\n  return a[i];\n}");
}
