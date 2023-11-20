#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_14238_ts_format_1_95cbe190() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export const foo = (\n  // prettier-ignore\n  bar as Baz\n).qux;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export const foo = // prettier-ignore\n(bar as Baz).qux;"
    );
}
#[test]
fn test_mapped_types_ts_format_1_368d2341() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type a= {\n    // prettier-ignore\n    [A in B]: C  |  D\n  }\n\ntype a= {\n    [\n      // prettier-ignore\n      A in B\n    ]: C  |  D\n  }\n\ntype a= {\n    [\n      A in\n      // prettier-ignore\n      B\n    ]: C  |  D\n  }\n\ntype a= {\n    [A in B]:\n      // prettier-ignore\n      C  |  D\n  }\n\ntype a= {\n    [\n      /* prettier-ignore */\n      A in B\n    ]: C  |  D\n  }\n\ntype a= {\n    [\n      A in\n      /* prettier-ignore */\n      B\n    ]: C  |  D\n  }\n\ntype a= {\n    [A in B]:\n      /* prettier-ignore */\n      C  |  D\n  }\n\n\ntype a= {\n    /* prettier-ignore */ [A in B]: C  |  D\n  }\n\ntype a= {\n    [/* prettier-ignore */ A in B ]: C  |  D\n  }\n\ntype a= {\n    [A in /* prettier-ignore */ B]: C  |  D\n  }\n\ntype a= {\n    [A in B]: /* prettier-ignore */ C  |  D\n  }\n\ntype a= {\n    /* prettier-ignore */\n    [A in B]: C  |  D\n  }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type a = {\n    // prettier-ignore\n    [A in B]: C  |  D\n  };\n\ntype a = {\n    [\n      // prettier-ignore\n      A in B\n    ]: C  |  D\n  };\n\ntype a = {\n  [A in // prettier-ignore\n  B]: C | D;\n};\n\ntype a = {\n  [A in B]: // prettier-ignore\n  C | D;\n};\n\ntype a = {\n    [\n      /* prettier-ignore */\n      A in B\n    ]: C  |  D\n  };\n\ntype a = {\n  [A in /* prettier-ignore */\n  B]: C | D;\n};\n\ntype a = {\n  [A in B]: /* prettier-ignore */\n  C | D;\n};\n\ntype a = {\n    /* prettier-ignore */ [A in B]: C  |  D\n  };\n\ntype a = {\n    [/* prettier-ignore */ A in B ]: C  |  D\n  };\n\ntype a = {\n  [A in /* prettier-ignore */ B]: C | D;\n};\n\ntype a = {\n  [A in B /* prettier-ignore */]: C | D;\n};\n\ntype a = {\n    /* prettier-ignore */\n    [A in B]: C  |  D\n  };");
}
#[test]
fn test_prettier_ignore_nested_unions_ts_format_1_02cdfadc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type a =\n  // foo\n  | foo1&foo2\n  // bar\n  | bar1&bar2\n  // prettier-ignore\n  | (\n    | aaaaaaaaaaaaa&1\n    // b\n    | bbbbbbbbbbbbb&2\n  )\n  // baz\n  | baz1&baz2;\n\nexport type b =\n  // foo\n  | foo1&foo2\n  // bar\n  | bar1&bar2\n  | (\n    // prettier-ignore\n    | aaaaaaaaaaaaa&1\n    // b\n    | bbbbbbbbbbbbb&2\n  )\n  // baz\n  | baz1&baz2;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type a =\n  // foo\n  | (foo1 & foo2)\n  // bar\n  | (bar1 & bar2)\n  // prettier-ignore\n  | (| aaaaaaaaaaaaa&1\n    // b\n    | bbbbbbbbbbbbb&2)\n  // baz\n  | (baz1 & baz2);\n\nexport type b =\n  // foo\n  | (foo1 & foo2)\n  // bar\n  | (bar1 & bar2)\n  // prettier-ignore\n  | (| aaaaaaaaaaaaa&1\n    // b\n    | bbbbbbbbbbbbb&2)\n  // baz\n  | (baz1 & baz2);");
}
#[test]
fn test_prettier_ignore_parenthesized_type_ts_format_1_2dbf0181() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type Foo =\n  // prettier-ignore\n  (\n    aa\n  );");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type Foo =\n  // prettier-ignore\n  aa;");
}
