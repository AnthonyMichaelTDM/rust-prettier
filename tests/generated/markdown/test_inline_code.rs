#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_backtick_md_prose_wrapalways_format_1_26c690ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\\` \\`123\\` \\`\\`\n\n\\`\\`12\\`34\\`\\`\n\n\\`\\` \\`12\\`\\`\n\n\\`\\`34\\` \\`\\`\n\n\\`\\` \\`\\`\\`123\\`\\`\\` \\`\\`\n\n\\`\\`\\` 3 \\`\\`22\\`\\` \\`1\\` \\`\\`\\`\n\n\\`\\` 2 \\`\\`\\`123\\`\\`\\` \\`1\\` \\`\\`\n\n\\`\\`  CODE\\` \\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\\` \\`123\\` \\`\\`\n\n\\`\\`12\\`34\\`\\`\n\n\\`\\` \\`12\\`\\`\n\n\\`\\`34\\` \\`\\`\n\n\\` \\`\\`\\`123\\`\\`\\` \\`\n\n\\`\\`\\` 3 \\`\\`22\\`\\` \\`1\\` \\`\\`\\`\n\n\\`\\` 2 \\`\\`\\`123\\`\\`\\` \\`1\\` \\`\\`\n\n\\`\\`  CODE\\` \\`\\`");
}
#[test]
fn test_backtick_md_prose_wrappreserve_format_1_26c690ce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`\\` \\`123\\` \\`\\`\n\n\\`\\`12\\`34\\`\\`\n\n\\`\\` \\`12\\`\\`\n\n\\`\\`34\\` \\`\\`\n\n\\`\\` \\`\\`\\`123\\`\\`\\` \\`\\`\n\n\\`\\`\\` 3 \\`\\`22\\`\\` \\`1\\` \\`\\`\\`\n\n\\`\\` 2 \\`\\`\\`123\\`\\`\\` \\`1\\` \\`\\`\n\n\\`\\`  CODE\\` \\`\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\\` \\`123\\` \\`\\`\n\n\\`\\`12\\`34\\`\\`\n\n\\`\\` \\`12\\`\\`\n\n\\`\\`34\\` \\`\\`\n\n\\` \\`\\`\\`123\\`\\`\\` \\`\n\n\\`\\`\\` 3 \\`\\`22\\`\\` \\`1\\` \\`\\`\\`\n\n\\`\\` 2 \\`\\`\\`123\\`\\`\\` \\`1\\` \\`\\`\n\n\\`\\`  CODE\\` \\`\\`");
}
#[test]
fn test_cjk_md_prose_wrapalways_format_1_17efebb1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`const x = \"中文123\"\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`const x = \"中文123\"\\`");
}
#[test]
fn test_cjk_md_prose_wrappreserve_format_1_17efebb1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`const x = \"中文123\"\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`const x = \"中文123\"\\`");
}
#[test]
fn test_escape_md_prose_wrapalways_format_1_996dd9a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`1*2*3\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`1*2*3\\`");
}
#[test]
fn test_escape_md_prose_wrappreserve_format_1_996dd9a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`1*2*3\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`1*2*3\\`");
}
#[test]
fn test_inline_code_multiple_spaces_md_prose_wrapalways_format_1_0123880f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`   three   spaces   everywhere   \\`\n\n\\`   three   spaces\n  everywhere   \\`\n\n\\`   three   spaces \n everywhere   \\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`   three   spaces   everywhere   \\`\n\n\\`   three   spaces   everywhere   \\`\n\n\\`   three   spaces   everywhere   \\`");
}
#[test]
fn test_inline_code_multiple_spaces_md_prose_wrappreserve_format_1_0123880f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`   three   spaces   everywhere   \\`\n\n\\`   three   spaces\n  everywhere   \\`\n\n\\`   three   spaces \n everywhere   \\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`   three   spaces   everywhere   \\`\n\n\\`   three   spaces\n  everywhere   \\`\n\n\\`   three   spaces \n everywhere   \\`");
}
#[test]
fn test_inline_code_newline_md_prose_wrapalways_format_1_e665a8fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \\`tempor\nincididunt\\` ut labore et dolore magna aliqua. Ut enim ad minim veniam, \\`quis\nnostrud\\` exercitation ullamco laboris nisi ut aliquip ex ea commodo \\`consequat.\nDuis\\` aute irure dolor in reprehenderit in voluptate velit esse cillum dolore \\`eu\nfugiat\\` nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in\nculpa qui officia deserunt mollit anim id est laborum.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod\n\\`tempor incididunt\\` ut labore et dolore magna aliqua. Ut enim ad minim veniam,\n\\`quis nostrud\\` exercitation ullamco laboris nisi ut aliquip ex ea commodo\n\\`consequat. Duis\\` aute irure dolor in reprehenderit in voluptate velit esse\ncillum dolore \\`eu fugiat\\` nulla pariatur. Excepteur sint occaecat cupidatat non\nproident, sunt in culpa qui officia deserunt mollit anim id est laborum.");
}
#[test]
fn test_inline_code_newline_md_prose_wrappreserve_format_1_e665a8fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \\`tempor\nincididunt\\` ut labore et dolore magna aliqua. Ut enim ad minim veniam, \\`quis\nnostrud\\` exercitation ullamco laboris nisi ut aliquip ex ea commodo \\`consequat.\nDuis\\` aute irure dolor in reprehenderit in voluptate velit esse cillum dolore \\`eu\nfugiat\\` nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in\nculpa qui officia deserunt mollit anim id est laborum.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \\`tempor\nincididunt\\` ut labore et dolore magna aliqua. Ut enim ad minim veniam, \\`quis\nnostrud\\` exercitation ullamco laboris nisi ut aliquip ex ea commodo \\`consequat.\nDuis\\` aute irure dolor in reprehenderit in voluptate velit esse cillum dolore \\`eu\nfugiat\\` nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in\nculpa qui officia deserunt mollit anim id est laborum.");
}
#[test]
fn test_long_md_prose_wrapalways_format_1_bf3f44c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`this is a long long long long long long long long long long long long long long long inline code\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`this is a long long long long long long long long long long long long long long long inline code\\`");
}
#[test]
fn test_long_md_prose_wrappreserve_format_1_bf3f44c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\\`this is a long long long long long long long long long long long long long long long inline code\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`this is a long long long long long long long long long long long long long long long inline code\\`");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_f0263dfb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`123\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`123\\`");
}
#[test]
fn test_simple_md_prose_wrappreserve_format_1_f0263dfb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["markdown"])
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\\`123\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\\`123\\`");
}
