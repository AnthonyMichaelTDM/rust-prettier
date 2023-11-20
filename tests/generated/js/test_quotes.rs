#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_functions_js_single_quotetrue_format_1_6ca951c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const a = () => \"Foo bar\";\n\nfunction b(object, key) {\n  return object['key'];\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const a = () => 'Foo bar';\n\nfunction b(object, key) {\n  return object['key'];\n}"
    );
}
#[test]
fn test_functions_js_format_1_6ca951c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const a = () => \"Foo bar\";\n\nfunction b(object, key) {\n  return object['key'];\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const a = () => \"Foo bar\";\n\nfunction b(object, key) {\n  return object[\"key\"];\n}"
    );
}
#[test]
fn test_objects_js_single_quotetrue_format_1_e33fe9ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .single_quote(true)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("const obj = {\n 'a': true,\n b: true,\n \"𐊧\": true,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const obj = {\n  a: true,\n  b: true,\n  '𐊧': true,\n};"
    );
}
#[test]
fn test_objects_js_format_1_e33fe9ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("const obj = {\n 'a': true,\n b: true,\n \"𐊧\": true,\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const obj = {\n  a: true,\n  b: true,\n  \"𐊧\": true,\n};"
    );
}
#[test]
fn test_strings_js_single_quotetrue_format_1_74b5ff1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .single_quote(true)
        .parsers(vec!["babel", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Prevent strings from being parsed as directives\n// See https://github.com/prettier/prettier/pull/1560#issue-227225960\n0;\n\n// Every string will be changed to double quotes, unless we end up with fewer\n// escaped quotes by using single quotes. (Vice versa if the \"singleQuote\"\n// option is true).\n//\n// Note that even if a string already has the correct enclosing quotes, it is\n// still processed in order to remove unnecessarily escaped quotes within it,\n// for consistency.\n\n// Simple strings.\n\"abc\"\n'abc'\n\n// Escape.\n'\\\\0'\n\n// Emoji.\n'🐶'\n\n// Empty string.\n\"\"\n''\n\n// Single double quote.\n\"\\\\\"\"\n'\"'\n\n// Single single quote.\n\"'\"\n'\\\\''\n\n// Unnecessary escapes.\n\"\\\\'\"\n'\\\\\"'\n\"\\\\a\"\n'\\\\a'\n\"hol\\\\a\"\n'hol\\\\a'\n\"hol\\\\\\\\a (the a is not escaped)\"\n'hol\\\\\\\\a (the a is not escaped)'\n\"multiple \\\\a unnecessary \\\\a escapes\"\n'multiple \\\\a unnecessary \\\\a escapes'\n\"unnecessarily escaped character preceded by escaped backslash \\\\\\\\\\\\a\"\n'unnecessarily escaped character preceded by escaped backslash \\\\\\\\\\\\a'\n\"unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a\"\n'unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a'\n\"\\\\a\\\\a\" // consecutive unnecessarily escaped characters\n'\\\\a\\\\a' // consecutive unnecessarily escaped characters\n'escaped \\\\u2030 \\\\‰ (should not stay escaped)'\n\n// Meaningful escapes\n\"octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7\"\n'octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7'\n\"meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61\"\n'meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61'\n'escaped newline \\\\\n'\n'escaped carriage return \\\\\n'\n'escaped \\\\u2028 \\\\\u{2028}'\n'escaped \\\\u2029 \\\\\u{2029}'\n\n// One of each.\n\"\\\\\"'\"\n'\"\\\\''\n\n// One of each with unnecessary escapes.\n\"\\\\\"\\\\'\"\n'\\\\\"\\\\''\n\n// More double quotes than single quotes.\n\"\\\\\"'\\\\\"\"\n'\"\\\\'\"'\n\n// More single quotes than double quotes.\n\"\\\\\"''\"\n'\"\\\\'\\\\''\n\n// Two of each.\n\"\\\\\"\\\\\"''\"\n'\"\"\\\\'\\\\''\n\n// Single backslash.\n'\\\\\\\\'\n\"\\\\\\\\\"\n\n// Backslases.\n\"\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\\\\\" '\\\\'\\\\\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\'\"\n'\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\\\\\' \"\\\\\"\\\\\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\"'\n\n// Somewhat more real-word example.\n\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\"\n'He\\\\'s sayin\\\\': \"How\\\\'s it goin\\\\'?\" Don\\\\'t ask me why.'\n\n// Somewhat more real-word example 2.\n\"var backslash = \\\\\"\\\\\\\\\\\\\", doubleQuote = '\\\\\"';\"\n'var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Prevent strings from being parsed as directives\n// See https://github.com/prettier/prettier/pull/1560#issue-227225960\n0;\n\n// Every string will be changed to double quotes, unless we end up with fewer\n// escaped quotes by using single quotes. (Vice versa if the \"singleQuote\"\n// option is true).\n//\n// Note that even if a string already has the correct enclosing quotes, it is\n// still processed in order to remove unnecessarily escaped quotes within it,\n// for consistency.\n\n// Simple strings.\n('abc');\n('abc');\n\n// Escape.\n('\\\\0');\n\n// Emoji.\n('🐶');\n\n// Empty string.\n('');\n('');\n\n// Single double quote.\n('\"');\n('\"');\n\n// Single single quote.\n(\"'\");\n(\"'\");\n\n// Unnecessary escapes.\n(\"'\");\n('\"');\n('a');\n('a');\n('hola');\n('hola');\n('hol\\\\\\\\a (the a is not escaped)');\n('hol\\\\\\\\a (the a is not escaped)');\n('multiple a unnecessary a escapes');\n('multiple a unnecessary a escapes');\n('unnecessarily escaped character preceded by escaped backslash \\\\\\\\a');\n('unnecessarily escaped character preceded by escaped backslash \\\\\\\\a');\n('unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a');\n('unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a');\n('aa'); // consecutive unnecessarily escaped characters\n('aa'); // consecutive unnecessarily escaped characters\n('escaped \\\\u2030 ‰ (should not stay escaped)');\n\n// Meaningful escapes\n('octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7');\n('octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7');\n('meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61');\n('meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61');\n('escaped newline \\\\\n');\n('escaped carriage return \\\\\n');\n('escaped \\\\u2028 \\\\\u{2028}');\n('escaped \\\\u2029 \\\\\u{2029}');\n\n// One of each.\n('\"\\\\'');\n('\"\\\\'');\n\n// One of each with unnecessary escapes.\n('\"\\\\'');\n('\"\\\\'');\n\n// More double quotes than single quotes.\n('\"\\\\'\"');\n('\"\\\\'\"');\n\n// More single quotes than double quotes.\n(\"\\\\\"''\");\n(\"\\\\\"''\");\n\n// Two of each.\n('\"\"\\\\'\\\\'');\n('\"\"\\\\'\\\\'');\n\n// Single backslash.\n('\\\\\\\\');\n('\\\\\\\\');\n\n// Backslases.\n(\"\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\\\\\" ''\\\\\\\\'\\\\\\\\'\\\\\\\\\\\\\\\\'\");\n('\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\\\\\' \"\"\\\\\\\\\"\\\\\\\\\"\\\\\\\\\\\\\\\\\"');\n\n// Somewhat more real-word example.\n(\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\");\n(\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\");\n\n// Somewhat more real-word example 2.\n('var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';');\n('var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';');");
}
#[test]
fn test_strings_js_format_1_74b5ff1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Prevent strings from being parsed as directives\n// See https://github.com/prettier/prettier/pull/1560#issue-227225960\n0;\n\n// Every string will be changed to double quotes, unless we end up with fewer\n// escaped quotes by using single quotes. (Vice versa if the \"singleQuote\"\n// option is true).\n//\n// Note that even if a string already has the correct enclosing quotes, it is\n// still processed in order to remove unnecessarily escaped quotes within it,\n// for consistency.\n\n// Simple strings.\n\"abc\"\n'abc'\n\n// Escape.\n'\\\\0'\n\n// Emoji.\n'🐶'\n\n// Empty string.\n\"\"\n''\n\n// Single double quote.\n\"\\\\\"\"\n'\"'\n\n// Single single quote.\n\"'\"\n'\\\\''\n\n// Unnecessary escapes.\n\"\\\\'\"\n'\\\\\"'\n\"\\\\a\"\n'\\\\a'\n\"hol\\\\a\"\n'hol\\\\a'\n\"hol\\\\\\\\a (the a is not escaped)\"\n'hol\\\\\\\\a (the a is not escaped)'\n\"multiple \\\\a unnecessary \\\\a escapes\"\n'multiple \\\\a unnecessary \\\\a escapes'\n\"unnecessarily escaped character preceded by escaped backslash \\\\\\\\\\\\a\"\n'unnecessarily escaped character preceded by escaped backslash \\\\\\\\\\\\a'\n\"unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a\"\n'unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a'\n\"\\\\a\\\\a\" // consecutive unnecessarily escaped characters\n'\\\\a\\\\a' // consecutive unnecessarily escaped characters\n'escaped \\\\u2030 \\\\‰ (should not stay escaped)'\n\n// Meaningful escapes\n\"octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7\"\n'octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7'\n\"meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61\"\n'meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61'\n'escaped newline \\\\\n'\n'escaped carriage return \\\\\n'\n'escaped \\\\u2028 \\\\\u{2028}'\n'escaped \\\\u2029 \\\\\u{2029}'\n\n// One of each.\n\"\\\\\"'\"\n'\"\\\\''\n\n// One of each with unnecessary escapes.\n\"\\\\\"\\\\'\"\n'\\\\\"\\\\''\n\n// More double quotes than single quotes.\n\"\\\\\"'\\\\\"\"\n'\"\\\\'\"'\n\n// More single quotes than double quotes.\n\"\\\\\"''\"\n'\"\\\\'\\\\''\n\n// Two of each.\n\"\\\\\"\\\\\"''\"\n'\"\"\\\\'\\\\''\n\n// Single backslash.\n'\\\\\\\\'\n\"\\\\\\\\\"\n\n// Backslases.\n\"\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\\\\\" '\\\\'\\\\\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\'\"\n'\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\\\\\' \"\\\\\"\\\\\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\"'\n\n// Somewhat more real-word example.\n\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\"\n'He\\\\'s sayin\\\\': \"How\\\\'s it goin\\\\'?\" Don\\\\'t ask me why.'\n\n// Somewhat more real-word example 2.\n\"var backslash = \\\\\"\\\\\\\\\\\\\", doubleQuote = '\\\\\"';\"\n'var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Prevent strings from being parsed as directives\n// See https://github.com/prettier/prettier/pull/1560#issue-227225960\n0;\n\n// Every string will be changed to double quotes, unless we end up with fewer\n// escaped quotes by using single quotes. (Vice versa if the \"singleQuote\"\n// option is true).\n//\n// Note that even if a string already has the correct enclosing quotes, it is\n// still processed in order to remove unnecessarily escaped quotes within it,\n// for consistency.\n\n// Simple strings.\n(\"abc\");\n(\"abc\");\n\n// Escape.\n(\"\\\\0\");\n\n// Emoji.\n(\"🐶\");\n\n// Empty string.\n(\"\");\n(\"\");\n\n// Single double quote.\n('\"');\n('\"');\n\n// Single single quote.\n(\"'\");\n(\"'\");\n\n// Unnecessary escapes.\n(\"'\");\n('\"');\n(\"a\");\n(\"a\");\n(\"hola\");\n(\"hola\");\n(\"hol\\\\\\\\a (the a is not escaped)\");\n(\"hol\\\\\\\\a (the a is not escaped)\");\n(\"multiple a unnecessary a escapes\");\n(\"multiple a unnecessary a escapes\");\n(\"unnecessarily escaped character preceded by escaped backslash \\\\\\\\a\");\n(\"unnecessarily escaped character preceded by escaped backslash \\\\\\\\a\");\n(\"unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a\");\n(\"unescaped character preceded by two escaped backslashes       \\\\\\\\\\\\\\\\a\");\n(\"aa\"); // consecutive unnecessarily escaped characters\n(\"aa\"); // consecutive unnecessarily escaped characters\n(\"escaped \\\\u2030 ‰ (should not stay escaped)\");\n\n// Meaningful escapes\n(\"octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7\");\n(\"octal escapes \\\\0 \\\\1 \\\\2 \\\\3 \\\\4 \\\\5 \\\\6 \\\\7\");\n(\"meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61\");\n(\"meaningfully escaped alphabetical characters \\\\n \\\\r \\\\v \\\\t \\\\b \\\\f \\\\u2713 \\\\x61\");\n(\"escaped newline \\\\\n\");\n(\"escaped carriage return \\\\\n\");\n(\"escaped \\\\u2028 \\\\\u{2028}\");\n(\"escaped \\\\u2029 \\\\\u{2029}\");\n\n// One of each.\n(\"\\\\\"'\");\n(\"\\\\\"'\");\n\n// One of each with unnecessary escapes.\n(\"\\\\\"'\");\n(\"\\\\\"'\");\n\n// More double quotes than single quotes.\n('\"\\\\'\"');\n('\"\\\\'\"');\n\n// More single quotes than double quotes.\n(\"\\\\\"''\");\n(\"\\\\\"''\");\n\n// Two of each.\n(\"\\\\\"\\\\\"''\");\n(\"\\\\\"\\\\\"''\");\n\n// Single backslash.\n(\"\\\\\\\\\");\n(\"\\\\\\\\\");\n\n// Backslases.\n(\"\\\\\"\\\\\\\\\\\\\"\\\\\\\\\\\\\\\\\\\\\" ''\\\\\\\\'\\\\\\\\'\\\\\\\\\\\\\\\\'\");\n('\\\\'\\\\\\\\\\\\'\\\\\\\\\\\\\\\\\\\\' \"\"\\\\\\\\\"\\\\\\\\\"\\\\\\\\\\\\\\\\\"');\n\n// Somewhat more real-word example.\n(\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\");\n(\"He's sayin': \\\\\"How's it goin'?\\\\\" Don't ask me why.\");\n\n// Somewhat more real-word example 2.\n('var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';');\n('var backslash = \"\\\\\\\\\", doubleQuote = \\\\'\"\\\\';');");
}
