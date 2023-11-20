#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_text_js_format_1_f797e405() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n// Example found at\n// https://github.com/sebmarkbage/art/blob/51ffce8164a555d652843241c2fdda52e186cbbd/parsers/svg/text.js#L137\nconst evil_regex_as_a_string = \"/[\\\\s�]*$/\";\n\nconst error: string = 123; // Should be an error, but can't lex this file") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n// Example found at\n// https://github.com/sebmarkbage/art/blob/51ffce8164a555d652843241c2fdda52e186cbbd/parsers/svg/text.js#L137\nconst evil_regex_as_a_string = \"/[s�]*$/\";\n\nconst error: string = 123; // Should be an error, but can't lex this file");
}
