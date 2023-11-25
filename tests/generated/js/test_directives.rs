#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_escaped_js_format_1_12882c12() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Unnecessary escapes. (adapted from tests/quotes/strings.js)\n// Note that in directives, unnecessary escapes should be preserved.\n// See https://github.com/prettier/prettier/issues/1555\n'\\'';\n'\\\"';\n\"\\'\";\n\"\\\"\";\n'\\\\';\n'\\a';\n\"hol\\a\"\n'hol\\a'\n\"hol\\\\a (the a is not escaped)\"\n'hol\\\\a (the a is not escaped)'\n\"multiple \\a unnecessary \\a escapes\"\n'multiple \\a unnecessary \\a escapes'\n\"unnecessarily escaped character preceded by escaped backslash \\\\\\a\"\n'unnecessarily escaped character preceded by escaped backslash \\\\\\a'\n\"unescaped character preceded by two escaped backslashes       \\\\\\\\a\"\n'unescaped character preceded by two escaped backslashes       \\\\\\\\a'\n\"\\a\\a\" // consecutive unnecessarily escaped characters\n'\\a\\a' // consecutive unnecessarily escaped characters\n'escaped \\u2030 \\‰ (should still stay escaped)'\n\n// Meaningful escapes\n// Commented out to avoid `SyntaxError: Octal literals are not allowed in strict mode.`\n// \"octal escapes \\0 \\1 \\2 \\3 \\4 \\5 \\6 \\7\"\n// 'octal escapes \\0 \\1 \\2 \\3 \\4 \\5 \\6 \\7'\n\"meaningfully escaped alphabetical characters \\n \\r \\v \\t \\b \\f \\u2713 \\x61\"\n'meaningfully escaped alphabetical characters \\n \\r \\v \\t \\b \\f \\u2713 \\x61'\n'escaped newline \\\n'\n'escaped carriage return \\\n'\n'escaped \\u2028 \\\u{2028}'\n'escaped \\u2029 \\\u{2029}'") ? ;
    assert_eq ! (formatted , "// Unnecessary escapes. (adapted from tests/quotes/strings.js)\n// Note that in directives, unnecessary escapes should be preserved.\n// See https://github.com/prettier/prettier/issues/1555\n'\\'';\n'\\\"';\n\"\\'\";\n\"\\\"\";\n\"\\\\\";\n\"\\a\";\n\"hol\\a\";\n\"hol\\a\";\n\"hol\\\\a (the a is not escaped)\";\n\"hol\\\\a (the a is not escaped)\";\n\"multiple \\a unnecessary \\a escapes\";\n\"multiple \\a unnecessary \\a escapes\";\n\"unnecessarily escaped character preceded by escaped backslash \\\\\\a\";\n\"unnecessarily escaped character preceded by escaped backslash \\\\\\a\";\n\"unescaped character preceded by two escaped backslashes       \\\\\\\\a\";\n\"unescaped character preceded by two escaped backslashes       \\\\\\\\a\";\n\"\\a\\a\"; // consecutive unnecessarily escaped characters\n\"\\a\\a\"; // consecutive unnecessarily escaped characters\n\"escaped \\u2030 \\‰ (should still stay escaped)\";\n\n// Meaningful escapes\n// Commented out to avoid `SyntaxError: Octal literals are not allowed in strict mode.`\n// \"octal escapes \\0 \\1 \\2 \\3 \\4 \\5 \\6 \\7\"\n// 'octal escapes \\0 \\1 \\2 \\3 \\4 \\5 \\6 \\7'\n\"meaningfully escaped alphabetical characters \\n \\r \\v \\t \\b \\f \\u2713 \\x61\";\n\"meaningfully escaped alphabetical characters \\n \\r \\v \\t \\b \\f \\u2713 \\x61\";\n\"escaped newline \\\n\";\n\"escaped carriage return \\\n\";\n\"escaped \\u2028 \\\u{2028}\";\n\"escaped \\u2029 \\\u{2029}\";");
    Ok(())
}
#[test]
fn test_issue_7346_js_format_1_de0615f0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("('bar'); // parens should not be removed to avoid becoming a directive\n`foo`;\n'bar'; // parens should be added, see https://github.com/prettier/prettier/issues/7346#issuecomment-574823604\n'\"';") ? ;
    assert_eq ! (formatted , "(\"bar\"); // parens should not be removed to avoid becoming a directive\n`foo`;\n(\"bar\"); // parens should be added, see https://github.com/prettier/prettier/issues/7346#issuecomment-574823604\n('\"');");
    Ok(())
}
#[test]
fn test_last_line_0_js_format_1_8c9ee196() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'use strict'")?;
    assert_eq!(formatted, "\"use strict\";");
    Ok(())
}
#[test]
fn test_last_line_1_js_format_1_1207512a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'use strict';")?;
    assert_eq!(formatted, "\"use strict\";");
    Ok(())
}
#[test]
fn test_last_line_2_js_format_1_97651c6f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'use strict';\n")?;
    assert_eq!(formatted, "\"use strict\";");
    Ok(())
}
#[test]
fn test_newline_js_format_1_8d79dcdf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\n\"use strict\";\n\nimport a from \"a\";\n\na();")?;
    assert_eq!(
        formatted,
        "/* @flow */\n\n\"use strict\";\n\nimport a from \"a\";\n\na();"
    );
    Ok(())
}
#[test]
fn test_no_newline_js_format_1_2b75afea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"use strict\";\na")?;
    assert_eq!(formatted, "\"use strict\";\na;");
    Ok(())
}
#[test]
fn test_test_js_format_1_5fd560b9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\"use strict\";\n\nfunction f1() {\n  \"use strict\";\n}\n\nfunction f2() {\n  'ngInject';\n  Object.assign(this, { $log, $uibModal });\n}\n\nfunction f3() {\n\n  'ngInject';\n\n  Object.assign(this, { $log, $uibModal });\n\n}\n\nfunction f4() {\n  'ngInject';\n\n\n  Object.assign(this, { $log, $uibModal });\n}") ? ;
    assert_eq ! (formatted , "\"use strict\";\n\nfunction f1() {\n  \"use strict\";\n}\n\nfunction f2() {\n  \"ngInject\";\n  Object.assign(this, { $log, $uibModal });\n}\n\nfunction f3() {\n  \"ngInject\";\n\n  Object.assign(this, { $log, $uibModal });\n}\n\nfunction f4() {\n  \"ngInject\";\n\n  Object.assign(this, { $log, $uibModal });\n}");
    Ok(())
}
