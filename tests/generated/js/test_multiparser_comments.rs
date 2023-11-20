#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_inside_js_format_1_6d5a28e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// #9274\nhtml\\`\n  <div>\n    \\${this.set && this.set.artist\n    /* avoid console errors if \\`this.set\\` is undefined */}\n  </div>\n\\`;\n\nhtml\\`\\${\n      foo\n  /* comment */\n}\\`;\nhtml\\`\n\\${\n      foo\n  /* comment */\n}\n\\`;\n\n\ngraphql\\`\\${\n      foo\n  /* comment */\n}\\`;\ngraphql\\`\n\\${\n      foo\n  /* comment */\n}\n\\`;\n\n\ncss\\`\\${\n      foo\n  /* comment */\n}\\`;\ncss\\`\n\\${\n      foo\n  /* comment */\n}\n\\`;\n\nmarkdown\\`\\${\n      foo\n  /* comment */\n}\\`;\nmarkdown\\`\n\\${\n      foo\n  /* comment */\n}\n\\`;\n\n// https://github.com/prettier/prettier/pull/9278#issuecomment-700589195\nexpr1 = html\\`\n  <div>\n    \\${x(foo, // fg\n        bar\n      )}</div>\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #9274\nhtml\\`\n  <div>\n    \\${\n      this.set && this.set.artist\n      /* avoid console errors if \\`this.set\\` is undefined */\n    }\n  </div>\n\\`;\n\nhtml\\`\\${\n  foo\n  /* comment */\n}\\`;\nhtml\\`\n  \\${\n    foo\n    /* comment */\n  }\n\\`;\n\ngraphql\\`\n  \\${\n    foo\n    /* comment */\n  }\n\\`;\ngraphql\\`\n  \\${\n    foo\n    /* comment */\n  }\n\\`;\n\ncss\\`\n  \\${\n    foo\n    /* comment */\n  }\n\\`;\ncss\\`\n  \\${\n    foo\n    /* comment */\n  }\n\\`;\n\nmarkdown\\`\\${\n  foo\n  /* comment */\n}\\`;\nmarkdown\\`\n\\${\n  foo\n  /* comment */\n}\n\\`;\n\n// https://github.com/prettier/prettier/pull/9278#issuecomment-700589195\nexpr1 = html\\`\n  <div>\n    \\${x(\n      foo, // fg\n      bar,\n    )}\n  </div>\n\\`;");
}
#[test]
fn test_comments_js_format_1_6320afa6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("graphql.experimental\\`\n  # required by createPaginationContainer\n  fragment MobileHomeDiffsSearchList_search on DifferentialRevisionSearch {\n    # required by createPaginationContainer\n    name\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "graphql.experimental\\`\n  # required by createPaginationContainer\n  fragment MobileHomeDiffsSearchList_search on DifferentialRevisionSearch {\n    # required by createPaginationContainer\n    name\n  }\n\\`;");
}
#[test]
fn test_tagged_js_format_1_6e6b0ae8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo(html // oops\n\\` <div><p>bar</p>foo</div> \\`);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo(\n  html // oops\n  \\`\n    <div>\n      <p>bar</p>\n      foo\n    </div>\n  \\`,\n);");
}
