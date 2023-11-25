#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_binaryish_js_format_1_8c466efb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f() {\n  throw (\n    property.isIdentifier() &&\n     FUNCTIONS[property.node.name] &&\n     (object.isIdentifier(JEST_GLOBAL) ||\n       (callee.isMemberExpression() && shouldHoistExpression(object))) &&\n    FUNCTIONS[property.node.name](expr.get('arguments'))\n  );\n\n  throw (\n    chalk.bold(\n      'No tests found related to files changed since last commit.\\n',\n    ) +\n    chalk.dim(\n      patternInfo.watch ?\n        'Press `a` to run all tests, or run Jest with `--watchAll`.' :\n        'Run Jest without `-o` to run all tests.',\n    )\n  );\n\n  throw !filePath.includes(coverageDirectory) &&\n    !filePath.endsWith(`.${SNAPSHOT_EXTENSION}`);\n}") ? ;
    assert_eq ! (formatted , "function f() {\n  throw (\n    property.isIdentifier() &&\n    FUNCTIONS[property.node.name] &&\n    (object.isIdentifier(JEST_GLOBAL) ||\n      (callee.isMemberExpression() && shouldHoistExpression(object))) &&\n    FUNCTIONS[property.node.name](expr.get(\"arguments\"))\n  );\n\n  throw (\n    chalk.bold(\"No tests found related to files changed since last commit.\\n\") +\n    chalk.dim(\n      patternInfo.watch\n        ? \"Press `a` to run all tests, or run Jest with `--watchAll`.\"\n        : \"Run Jest without `-o` to run all tests.\",\n    )\n  );\n\n  throw (\n    !filePath.includes(coverageDirectory) &&\n    !filePath.endsWith(`.${SNAPSHOT_EXTENSION}`)\n  );\n}");
    Ok(())
}
#[test]
fn test_comment_js_format_1_5b95e2b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function x() {\n  throw func2\n      //comment\n      .bar();\n}\n \nfunction f() {\n  throw (\n    foo\n      // comment\n      .bar()\n  );\n}\n \nfn(function f() {\n  throw (\n    foo\n      // comment\n      .bar()\n  );\n});") ? ;
    assert_eq ! (formatted , "function x() {\n  throw (\n    func2\n      //comment\n      .bar()\n  );\n}\n\nfunction f() {\n  throw (\n    foo\n      // comment\n      .bar()\n  );\n}\n\nfn(function f() {\n  throw (\n    foo\n      // comment\n      .bar()\n  );\n});");
    Ok(())
}
#[test]
fn test_jsx_js_format_1_49c0914c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n  throw <Bar />;\n}\n\nfunction foo() {\n  throw <Bar>baz</Bar>;\n}\n\nfunction foo() {\n  throw <Bar baz={baz} />;\n}\n\nfunction foo() {\n  throw <Bar baz={baz}>foo</Bar>;\n}\n\nfunction foo() {\n  throw <></>;\n}\n\nfunction foo() {\n  throw <>foo</>;\n}") ? ;
    assert_eq ! (formatted , "function foo() {\n  throw <Bar />;\n}\n\nfunction foo() {\n  throw <Bar>baz</Bar>;\n}\n\nfunction foo() {\n  throw <Bar baz={baz} />;\n}\n\nfunction foo() {\n  throw <Bar baz={baz}>foo</Bar>;\n}\n\nfunction foo() {\n  throw <></>;\n}\n\nfunction foo() {\n  throw <>foo</>;\n}");
    Ok(())
}
