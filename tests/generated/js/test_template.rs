#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_js_format_1_74d2b2ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("() => a\\`\n\ta\n\\`;\n\n() => \\`\n\ta\n\\`;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "() => a\\`\n\ta\n\\`;\n\n() => \\`\n\ta\n\\`;");
}
#[test]
fn test_call_js_format_1_61c263e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("insertRule(\\`*, *:before, *:after {\n  box-sizing: inherit;\n}\\`);\n\ninsertRule\\`*, *:before, *:after {\n  box-sizing: inherit;\n}\\`;\n\nnew Error(formatErrorMessage\\`\n  This a really bad error.\n  Which has more than one line.\n\\`);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "insertRule(\\`*, *:before, *:after {\n  box-sizing: inherit;\n}\\`);\n\ninsertRule\\`*, *:before, *:after {\n  box-sizing: inherit;\n}\\`;\n\nnew Error(formatErrorMessage\\`\n  This a really bad error.\n  Which has more than one line.\n\\`);");
}
#[test]
fn test_comment_js_format_1_b4c75e03() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\n(?:\\${escapeChar}[\\\\\\\\S\\\\\\\\s]|(?:(?!\\${// Using \\`XRegExp.union\\` safely rewrites backreferences in \\`left\\` and \\`right\\`.\n// Intentionally not passing \\`basicFlags\\` to \\`XRegExp.union\\` since any syntax\n// transformation resulting from those flags was already applied to \\`left\\` and\n// \\`right\\` when they were passed through the XRegExp constructor above.\nXRegExp.union([left, right], '', {conjunction: 'or'}).source})[^\\${escapeChar}])+)+\n\\`;\n\n\\`a\\${/* b */c/* d */}e\\${// f\ng\n// h\n}\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\n(?:\\${escapeChar}[\\\\\\\\S\\\\\\\\s]|(?:(?!\\${\n  // Using \\`XRegExp.union\\` safely rewrites backreferences in \\`left\\` and \\`right\\`.\n  // Intentionally not passing \\`basicFlags\\` to \\`XRegExp.union\\` since any syntax\n  // transformation resulting from those flags was already applied to \\`left\\` and\n  // \\`right\\` when they were passed through the XRegExp constructor above.\n  XRegExp.union([left, right], \"\", { conjunction: \"or\" }).source\n})[^\\${escapeChar}])+)+\n\\`;\n\n\\`a\\${/* b */ c /* d */}e\\${\n  // f\n  g\n  // h\n}\\`;");
}
#[test]
fn test_faulty_locations_js_format_1_215f85ac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o = {\n  [\\`key\\`]: () => {\n    // Comment\n  }\n};\n\nvar x = {\n  y: () => Relay.QL\\`\n    query {\n      \\${foo},\n      field,\n    }\n  \\`\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o = {\n  [\\`key\\`]: () => {\n    // Comment\n  },\n};\n\nvar x = {\n  y: () => Relay.QL\\`\n    query {\n      \\${foo},\n      field,\n    }\n  \\`,\n};");
}
#[test]
fn test_graphql_js_format_1_d0d1d249() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("module.exports = Relay.createContainer(\n  // ...\n  {\n    fragments: {\n      nodes: ({solution_type, time_frame}) => Relay.QL\\`\n        fragment on RelatedNode @relay(plural: true) {\n          __typename\n          \\${OptimalSolutionsSection\n            .getFragment(\n              'node',\n              {solution_type, time_frame},\n            )\n          }\n        }\n      \\`,\n    },\n  },\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "module.exports = Relay.createContainer(\n  // ...\n  {\n    fragments: {\n      nodes: ({ solution_type, time_frame }) => Relay.QL\\`\n        fragment on RelatedNode @relay(plural: true) {\n          __typename\n          \\${OptimalSolutionsSection.getFragment(\"node\", {\n            solution_type,\n            time_frame,\n          })}\n        }\n      \\`,\n    },\n  },\n);");
}
#[test]
fn test_indent_js_format_1_7cf98fc9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo = () => {\n  {\n    {\n      {\n        return \\`\nline 1\nline 2\n...\nline n\n\\${foo({\n  many: keys,\n  many: keys\n})}\nline n + 1\nline n + 2\nline n + n\n\\`;\n      }\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = () => {\n  {\n    {\n      {\n        return \\`\nline 1\nline 2\n...\nline n\n\\${foo({\n  many: keys,\n  many: keys,\n})}\nline n + 1\nline n + 2\nline n + n\n\\`;\n      }\n    }\n  }\n};");
}
#[test]
fn test_inline_js_format_1_6978d2af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("this._pipe.write(\\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`);\nthis._pipe.write(\n  \\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`\n);\nthis._pipe\n  .write(\n    \\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`\n  );\n\nthis._pipe.write(\\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\\`);\n\nthis._pipe.write(\n  \\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\\`\n);\n\nthis._pipe.write(\\`\n  \\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\n\\`);\n\n\n() => \\`\n  a\n\\`;\n\n() =>\n  \\`\n    a\n  \\`;\n\n\n// https://github.com/prettier/prettier/issues/5529\neditTitle += \\`\\${iconHTML({ class: \"reply-to-glyph\" })}\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "this._pipe.write(\\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`);\nthis._pipe.write(\\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`);\nthis._pipe.write(\\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest}\\`);\n\nthis._pipe.write(\n  \\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\\`,\n);\n\nthis._pipe.write(\n  \\`\\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\\`,\n);\n\nthis._pipe.write(\\`\n  \\\\n\\\\n Pattern matches \\${total} \\${pluralizeTest} but that's long\n\\`);\n\n() => \\`\n  a\n\\`;\n\n() =>\n  \\`\n    a\n  \\`;\n\n// https://github.com/prettier/prettier/issues/5529\neditTitle += \\`\\${iconHTML({ class: \"reply-to-glyph\" })}\\`;");
}
#[test]
fn test_parenthesis_js_format_1_48c26ba9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// \"ArrowFunctionExpression\"\n(() => {})\\`\\`;\n\n// \"AssignmentExpression\"\n(b = c)\\`\\`;\n\n// \"AwaitExpression\"\nasync function f() {\n  (await b)\\`\\`;\n}\n\n// \"BinaryExpression\"\n(b + c)\\`\\`;\n\n// \"CallExpression\"\nb()\\`\\`;\n\n// \"ClassExpression\"\n(class {})\\`\\`;\n\n// \"ConditionalExpression\"\n(b ? c : d)\\`\\`;\n\n// \"FunctionExpression\"\n(function() {})\\`\\`;\n\n// \"LogicalExpression\"\n(b || c)\\`\\`;\n\n// \"MemberExpression\"\nb.c\\`\\`;\n\n// \"NewExpression\"\n(new B())\\`\\`;\n\n// \"ObjectExpression\"\n({})\\`\\`;\n\n// \"SequenceExpression\"\n(b, c)\\`\\`;\n\n// \"TaggedTemplateExpression\"\n(\\`\\`)\\`\\`;\n\n// \"UnaryExpression\"\n(void b)\\`\\`;\n\n// \"UpdateExpression\"\n(++b)\\`\\`;\n\n// \"YieldExpression\"\nfunction* d() {\n  (yield 1)\\`\\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// \"ArrowFunctionExpression\"\n(() => {})\\`\\`;\n\n// \"AssignmentExpression\"\n(b = c)\\`\\`;\n\n// \"AwaitExpression\"\nasync function f() {\n  (await b)\\`\\`;\n}\n\n// \"BinaryExpression\"\n(b + c)\\`\\`;\n\n// \"CallExpression\"\nb()\\`\\`;\n\n// \"ClassExpression\"\n(class {})\\`\\`;\n\n// \"ConditionalExpression\"\n(b ? c : d)\\`\\`;\n\n// \"FunctionExpression\"\n(function () {})\\`\\`;\n\n// \"LogicalExpression\"\n(b || c)\\`\\`;\n\n// \"MemberExpression\"\nb.c\\`\\`;\n\n// \"NewExpression\"\nnew B()\\`\\`;\n\n// \"ObjectExpression\"\n({})\\`\\`;\n\n// \"SequenceExpression\"\n(b, c)\\`\\`;\n\n// \"TaggedTemplateExpression\"\n\\`\\`\\`\\`;\n\n// \"UnaryExpression\"\n(void b)\\`\\`;\n\n// \"UpdateExpression\"\n(++b)\\`\\`;\n\n// \"YieldExpression\"\nfunction* d() {\n  (yield 1)\\`\\`;\n}");
}
