use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_binaryish_js_format_1_5fe10847() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f() {\n  return (\n    property.isIdentifier() &&\n     FUNCTIONS[property.node.name] &&\n     (object.isIdentifier(JEST_GLOBAL) ||\n       (callee.isMemberExpression() && shouldHoistExpression(object))) &&\n    FUNCTIONS[property.node.name](expr.get('arguments'))\n  );\n\n  return (\n    chalk.bold(\n      'No tests found related to files changed since last commit.\\\\n',\n    ) +\n    chalk.dim(\n      patternInfo.watch ?\n        'Press \\`a\\` to run all tests, or run Jest with \\`--watchAll\\`.' :\n        'Run Jest without \\`-o\\` to run all tests.',\n    )\n  );\n\n  return !filePath.includes(coverageDirectory) &&\n    !filePath.endsWith(\\`.\\${SNAPSHOT_EXTENSION}\\`);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  return (\n    property.isIdentifier() &&\n    FUNCTIONS[property.node.name] &&\n    (object.isIdentifier(JEST_GLOBAL) ||\n      (callee.isMemberExpression() && shouldHoistExpression(object))) &&\n    FUNCTIONS[property.node.name](expr.get(\"arguments\"))\n  );\n\n  return (\n    chalk.bold(\"No tests found related to files changed since last commit.\\\\n\") +\n    chalk.dim(\n      patternInfo.watch\n        ? \"Press \\`a\\` to run all tests, or run Jest with \\`--watchAll\\`.\"\n        : \"Run Jest without \\`-o\\` to run all tests.\",\n    )\n  );\n\n  return (\n    !filePath.includes(coverageDirectory) &&\n    !filePath.endsWith(\\`.\\${SNAPSHOT_EXTENSION}\\`)\n  );\n}");
}
#[test]
fn test_comment_js_format_1_08d5dbc9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f() {\n  return /* a */;\n}\n\nfunction f() {\n  return // a\n  ;\n}\n\nfunction f() {\n  return // a\n  /* b */;\n}\n\nfunction f() {\n  return /* a */\n  // b\n  ;\n}\n\nfunction x() {\n  return func2\n      //comment\n      .bar();\n}\n\nfunction f() {\n  return (\n    foo\n      // comment\n      .bar()\n  );\n}\n\nfn(function f() {\n  return (\n    foo\n      // comment\n      .bar()\n  );\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  return /* a */;\n}\n\nfunction f() {\n  return; // a\n}\n\nfunction f() {\n  return // a\n  /* b */;\n}\n\nfunction f() {\n  return; /* a */\n  // b\n}\n\nfunction x() {\n  return (\n    func2\n      //comment\n      .bar()\n  );\n}\n\nfunction f() {\n  return (\n    foo\n      // comment\n      .bar()\n  );\n}\n\nfn(function f() {\n  return (\n    foo\n      // comment\n      .bar()\n  );\n});");
}
