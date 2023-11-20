#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_tag_js_format_1_3e8c9ca6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const query = /* GraphQL */\\`\n      {\n    user(   id :   5  )  {\n      firstName\n\n      lastName\n    }\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const query = /* GraphQL */ \\`\n  {\n    user(id: 5) {\n      firstName\n\n      lastName\n    }\n  }\n\\`;");
}
#[test]
fn test_definitions_js_format_1_85735e87() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "graphql\\`\n  fragment x on y {\n    z\n  }\n\n  fragment a on b {\n    c\n  }\n\\`;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "graphql\\`\n  fragment x on y {\n    z\n  }\n\n  fragment a on b {\n    c\n  }\n\\`;"
    );
}
#[test]
fn test_escape_js_format_1_569b4cc1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("gql\\`\n  \"\\\\\\`foo\\\\\\` mutation payload.\"\n  type      FooPayload       {\n    \tbar: String\n  }\n\\`\n\ngql\\`\ntype Project {\n    \"Pattern: \\\\\\`\\\\\\${project}\\\\\\`\"\n    pattern: String\n    \"\"\"\n    Pattern: \\\\\\`\\\\\\${project}\\\\\\`\n    \"\"\"\n    pattern: String\n\n\t# Also: Escaping the first parentheses...\n\t\"Pattern: \\\\\\`$\\\\{project}\\\\\\`\"\n    pattern: String\n    # Or escaping the first and second parentheses...\n\t\"Pattern: \\\\\\`$\\\\{project\\\\}\\\\\\`\"\n    pattern: String\n}\n\\`\n\ngql\\`\n  \"\"\"\n  - \\\\\\`\n  - \\\\\\\\\\\\\\`\n  - \\\\\\\\ a\n  - \\\\\\\\\\\\\\\\\n  - $\n  - \\\\$\n  - \\\\\\${\n  - \\\\\\\\\\\\\\${\n  - \\\\u1234\n  \"\"\"\n  type A {\n    a\n  }\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "gql\\`\n  \"\\\\\\`foo\\\\\\` mutation payload.\"\n  type FooPayload {\n    bar: String\n  }\n\\`;\n\ngql\\`\n  type Project {\n    \"Pattern: \\\\\\`\\\\\\${project}\\\\\\`\"\n    pattern: String\n    \"\"\"\n    Pattern: \\\\\\`\\\\\\${project}\\\\\\`\n    \"\"\"\n    pattern: String\n\n    # Also: Escaping the first parentheses...\n    \"Pattern: \\\\\\`\\\\\\${project}\\\\\\`\"\n    pattern: String\n    # Or escaping the first and second parentheses...\n    \"Pattern: \\\\\\`\\\\\\${project}\\\\\\`\"\n    pattern: String\n  }\n\\`;\n\ngql\\`\n  \"\"\"\n  - \\\\\\`\n  - \\\\\\\\\\\\\\`\n  - \\\\\\\\ a\n  - \\\\\\\\\\\\\\\\\n  - $\n  - \\\\$\n  - \\\\\\${\n  - \\\\\\\\\\\\\\${\n  - \\\\u1234\n  \"\"\"\n  type A {\n    a\n  }\n\\`;");
}
#[test]
fn test_expressions_js_format_1_bd7127ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("graphql(schema, \\`\nquery allPartsByManufacturerName($name: String!) {\n  allParts(filter:{manufacturer: {name: $name}}) {\n...    PartAll\n}}\n\\${fragments.all}\n\\`)\n\nconst veryLongVariableNameToMakeTheLineBreak = graphql(schema, \\`\nquery allPartsByManufacturerName($name: String!) {\n  allParts(filter:{manufacturer: {name: $name}}) {\n...    PartAll\n}}\n\\${fragments.all}\n\\`)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "graphql(\n  schema,\n  \\`\n    query allPartsByManufacturerName($name: String!) {\n      allParts(filter: { manufacturer: { name: $name } }) {\n        ...PartAll\n      }\n    }\n    \\${fragments.all}\n  \\`,\n);\n\nconst veryLongVariableNameToMakeTheLineBreak = graphql(\n  schema,\n  \\`\n    query allPartsByManufacturerName($name: String!) {\n      allParts(filter: { manufacturer: { name: $name } }) {\n        ...PartAll\n      }\n    }\n    \\${fragments.all}\n  \\`,\n);");
}
#[test]
fn test_graphql_js_format_1_55a22759() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("graphql(schema, \\`\nmutation     MarkReadNotificationMutation(\n    $input\n    : MarkReadNotificationData!\n  )\n{ markReadNotification(data: $input ) { notification {seenState} } }\\`)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "graphql(\n  schema,\n  \\`\n    mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {\n      markReadNotification(data: $input) {\n        notification {\n          seenState\n        }\n      }\n    }\n  \\`,\n);");
}
#[test]
fn test_graphql_tag_js_format_1_6bdd2dd6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import gql from \"graphql-tag\";\n\nconst query = gql\\`\n      {\n    user(   id :   5  )  {\n      firstName\n\n      lastName\n    }\n  }\n\\`;\n\n\n// With interpolations:\n\ngql\\`\nquery User {\n  user(id:5){\n    ...UserDetails\n    ...Friends\n  }\n}\n\n\\${USER_DETAILS_FRAGMENT}\\${FRIENDS_FRAGMENT}\n\\`\n\n\n// Skip if non-toplevel interpolation:\n\ngql\\`\nquery User {\n  user(id:\\${id}){ name }\n}\n\\`\n\n\n// Skip if top-level interpolation within comment:\n\ngql\\`\nquery User {\n  user(id:5){ name }\n}\n#\\${test}\n\\`\n\n\n// Comment on last line:\n\ngql\\`\nquery User {\n  user(id:5){ name }\n}\n# comment\\`\n// \\` <-- editor syntax highlighting workaround\n\n\n// Preserve up to one blank line between things and enforce linebreak between\n// interpolations:\n\ngql\\`\n# comment\n\\${one}\\${two}  \\${three}\n\\${four}\n\n\\${five}\n# comment\n\\${six}\n\n# comment\n\\${seven}\n# comment\n\n\\${eight}\n\n  # comment with trailing whitespace      \n\n\n# blank line above this comment\n\n\n\\`\n\n\n// Interpolation directly before and after query:\n\ngql\\`\\${one} query Test { test }\\${two}\\`\n\n\n// Only interpolation:\n\ngql\\`\\${test}\\`\n\n\n// Only comment:\n\ngql\\`# comment\\`\n// \\` <-- editor syntax highlighting workaround\n\n\n// Only whitespace:\n\ngql\\`   \\`\n\n\n// Empty:\n\ngql\\`\\`\n\n\n// Comments after other things:\n// Currently, comments after interpolations are moved to the next line.\n// We might want to keep them on the next line in the future.\n\ngql\\`\n  \\${test} # comment\n\n  query Test { # comment\n    test # comment\n  } # comment\n  \\${test} # comment\n  \\${test} # comment\n\n  \\${test} # comment\n\n  # comment\n  \\${test} # comment\n\\`\n\n\n// Larger mixed test:\n\ngql\\`\n\n\n\nquery User {\n  test\n}\n\n    \n\t\n\\${USER_DETAILS_FRAGMENT}\n\n   # Comment    \n   # that continues on a new line\n\n    \n   # and has a blank line in the middle\n\n    \\${FRIENDS_FRAGMENT}\n  \\${generateFragment({\n     totally:  \"a good idea\"\n    })}\n\n\\${fragment}#comment\n\nfragment another on User { name\n}\\${ fragment }\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import gql from \"graphql-tag\";\n\nconst query = gql\\`\n  {\n    user(id: 5) {\n      firstName\n\n      lastName\n    }\n  }\n\\`;\n\n// With interpolations:\n\ngql\\`\n  query User {\n    user(id: 5) {\n      ...UserDetails\n      ...Friends\n    }\n  }\n\n  \\${USER_DETAILS_FRAGMENT}\n  \\${FRIENDS_FRAGMENT}\n\\`;\n\n// Skip if non-toplevel interpolation:\n\ngql\\`\nquery User {\n  user(id:\\${id}){ name }\n}\n\\`;\n\n// Skip if top-level interpolation within comment:\n\ngql\\`\nquery User {\n  user(id:5){ name }\n}\n#\\${test}\n\\`;\n\n// Comment on last line:\n\ngql\\`\n  query User {\n    user(id: 5) {\n      name\n    }\n  }\n  # comment\n\\`;\n// \\` <-- editor syntax highlighting workaround\n\n// Preserve up to one blank line between things and enforce linebreak between\n// interpolations:\n\ngql\\`\n  # comment\n  \\${one}\n  \\${two}\n  \\${three}\n  \\${four}\n\n  \\${five}\n  # comment\n  \\${six}\n\n  # comment\n  \\${seven}\n  # comment\n\n  \\${eight}\n\n  # comment with trailing whitespace\n\n  # blank line above this comment\n\\`;\n\n// Interpolation directly before and after query:\n\ngql\\`\n  \\${one}\n  query Test {\n    test\n  }\n  \\${two}\n\\`;\n\n// Only interpolation:\n\ngql\\`\n  \\${test}\n\\`;\n\n// Only comment:\n\ngql\\`\n  # comment\n\\`;\n// \\` <-- editor syntax highlighting workaround\n\n// Only whitespace:\n\ngql\\`\\`;\n\n// Empty:\n\ngql\\`\\`;\n\n// Comments after other things:\n// Currently, comments after interpolations are moved to the next line.\n// We might want to keep them on the next line in the future.\n\ngql\\`\n  \\${test}\n  # comment\n\n  query Test {\n    # comment\n    test # comment\n  } # comment\n  \\${test}\n  # comment\n  \\${test}\n  # comment\n\n  \\${test}\n  # comment\n\n  # comment\n  \\${test}\n  # comment\n\\`;\n\n// Larger mixed test:\n\ngql\\`\n  query User {\n    test\n  }\n\n  \\${USER_DETAILS_FRAGMENT}\n\n  # Comment\n  # that continues on a new line\n\n  # and has a blank line in the middle\n\n  \\${FRIENDS_FRAGMENT}\n  \\${generateFragment({\n    totally: \"a good idea\",\n  })}\n\n  \\${fragment}\n  #comment\n\n  fragment another on User {\n    name\n  }\n  \\${fragment}\n\\`;");
}
#[test]
fn test_invalid_js_format_1_fd645794() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// none of the embedded GraphQL should be formatted\n// for they have an invalid escape sequence\n\ngql\\`\n  \"\\\\x\"\n  type   Foo    {\n      a: string\n  }\n\\`;\n\ngql\\`\n  type   Foo {\n      a:   string\n  }\n\n  \\${stuff}\n\n  \"\\\\x\"\n  type  Bar   {\n       b :   string\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// none of the embedded GraphQL should be formatted\n// for they have an invalid escape sequence\n\ngql\\`\n  \"\\\\x\"\n  type   Foo    {\n      a: string\n  }\n\\`;\n\ngql\\`\n  type   Foo {\n      a:   string\n  }\n\n  \\${stuff}\n\n  \"\\\\x\"\n  type  Bar   {\n       b :   string\n  }\n\\`;");
}
#[test]
fn test_react_relay_js_format_1_711441bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const { graphql } = require(\"react-relay\");\n\ngraphql\\`\n mutation     MarkReadNotificationMutation(\n    $input\n    : MarkReadNotificationData!\n  )\n{ markReadNotification(data: $input ) { notification {seenState} } }\n\\`;\n\ngraphql.experimental\\`\n mutation     MarkReadNotificationMutation(\n    $input\n    : MarkReadNotificationData!\n  )\n{ markReadNotification(data: $input ) { notification {seenState} } }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const { graphql } = require(\"react-relay\");\n\ngraphql\\`\n  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {\n    markReadNotification(data: $input) {\n      notification {\n        seenState\n      }\n    }\n  }\n\\`;\n\ngraphql.experimental\\`\n  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {\n    markReadNotification(data: $input) {\n      notification {\n        seenState\n      }\n    }\n  }\n\\`;");
}
