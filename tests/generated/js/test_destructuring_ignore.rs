use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ignore_js_trailing_commaall_format_1_008fb415() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const {\n  // prettier-ignore\n  bar =           1,\n} = foo\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1,\n} = foo\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1,         // comment\n} = foo\n\nconst {\n  // prettier-ignore\n  bar4 =           1,         /* comment */\n} = foo\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1,\n} = foo\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo\n\n// Nested\nconst {\n  baz: {\n  // prettier-ignore\n  foo2 = [1, 2,    3]\n},\n  // prettier-ignore\n  bar7 =            1,\n} = foo") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const {\n  // prettier-ignore\n  bar =           1,\n} = foo;\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1,\n} = foo;\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1, // comment\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar4 =           1 /* comment */,\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1,\n} = foo;\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo;\n\n// Nested\nconst {\n  baz: {\n    // prettier-ignore\n    foo2 = [1, 2,    3],\n  },\n  // prettier-ignore\n  bar7 =            1,\n} = foo;");
}
#[test]
fn test_ignore_js_trailing_commaes_5_format_1_008fb415() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const {\n  // prettier-ignore\n  bar =           1,\n} = foo\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1,\n} = foo\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1,         // comment\n} = foo\n\nconst {\n  // prettier-ignore\n  bar4 =           1,         /* comment */\n} = foo\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1,\n} = foo\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo\n\n// Nested\nconst {\n  baz: {\n  // prettier-ignore\n  foo2 = [1, 2,    3]\n},\n  // prettier-ignore\n  bar7 =            1,\n} = foo") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const {\n  // prettier-ignore\n  bar =           1,\n} = foo;\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1,\n} = foo;\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1, // comment\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar4 =           1 /* comment */,\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1,\n} = foo;\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo;\n\n// Nested\nconst {\n  baz: {\n    // prettier-ignore\n    foo2 = [1, 2,    3],\n  },\n  // prettier-ignore\n  bar7 =            1,\n} = foo;");
}
#[test]
fn test_ignore_js_trailing_commanone_format_1_008fb415() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const {\n  // prettier-ignore\n  bar =           1,\n} = foo\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1,\n} = foo\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1,         // comment\n} = foo\n\nconst {\n  // prettier-ignore\n  bar4 =           1,         /* comment */\n} = foo\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1,\n} = foo\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo\n\n// Nested\nconst {\n  baz: {\n  // prettier-ignore\n  foo2 = [1, 2,    3]\n},\n  // prettier-ignore\n  bar7 =            1,\n} = foo") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const {\n  // prettier-ignore\n  bar =           1\n} = foo;\n\nconst {\n  _,\n  // prettier-ignore\n  bar2 =           1\n} = foo;\n\n/* comments */\nconst {\n  // prettier-ignore\n  bar3 =           1 // comment\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar4 =           1 /* comment */\n} = foo;\n\nconst {\n  // prettier-ignore\n  bar5 =           /* comment */          1\n} = foo;\n\n/* RestElement */\nconst {\n  // prettier-ignore\n  ...bar6\n} = foo;\n\n// Nested\nconst {\n  baz: {\n    // prettier-ignore\n    foo2 = [1, 2,    3]\n  },\n  // prettier-ignore\n  bar7 =            1\n} = foo;");
}
