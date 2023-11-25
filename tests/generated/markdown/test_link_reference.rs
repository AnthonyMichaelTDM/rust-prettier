#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cjk_md_prose_wrapalways_format_1_44e7f1dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]");
}
#[test]
fn test_cjk_md_prose_wrapnever_format_1_44e7f1dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[這是一段很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長很長的段落][]");
}
#[test]
fn test_collapsed_md_prose_wrapalways_format_1_734309c8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello][]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello][]");
}
#[test]
fn test_collapsed_md_prose_wrapnever_format_1_734309c8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello][]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello][]");
}
#[test]
fn test_definition_md_prose_wrapalways_format_1_33b8b728() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[just-url]: https://example.com\n[url-with-short-title]: https://example.com \"title\"\n[url-with-long-title]: https://example.com \"a long, long title. It's really really long. Here have words.\"\n[empty-title]: https://example.com \"\" \n\n[long]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \n[long-with-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \"look a title!\"\n[long-with-empty-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \"\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[just-url]: https://example.com\n[url-with-short-title]: https://example.com \"title\"\n[url-with-long-title]:\n  https://example.com\n  \"a long, long title. It's really really long. Here have words.\"\n[empty-title]: https://example.com\n[long]:\n  https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx\n[long-with-title]:\n  https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx\n  \"look a title!\"\n[long-with-empty-title]:\n  https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx");
}
#[test]
fn test_definition_md_prose_wrapnever_format_1_33b8b728() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[just-url]: https://example.com\n[url-with-short-title]: https://example.com \"title\"\n[url-with-long-title]: https://example.com \"a long, long title. It's really really long. Here have words.\"\n[empty-title]: https://example.com \"\" \n\n[long]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \n[long-with-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \"look a title!\"\n[long-with-empty-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \"\"") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[just-url]: https://example.com\n[url-with-short-title]: https://example.com \"title\"\n[url-with-long-title]: https://example.com \"a long, long title. It's really really long. Here have words.\"\n[empty-title]: https://example.com\n[long]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx\n[long-with-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx \"look a title!\"\n[long-with-empty-title]: https://example.com/a-long-url/another-segment/yet-another-segment/a-really-long-file-name.php.aspx");
}
#[test]
fn test_full_md_prose_wrapalways_format_1_e68e07d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello][world]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello][world]");
}
#[test]
fn test_full_md_prose_wrapnever_format_1_e68e07d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello][world]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello][world]");
}
#[test]
fn test_shortcut_md_prose_wrapalways_format_1_c57ed766() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello]");
}
#[test]
fn test_shortcut_md_prose_wrapnever_format_1_c57ed766() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[hello]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[hello]");
}
#[test]
fn test_title_md_prose_wrapalways_format_1_cec5c5ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ref]: https://example.com (bar)\n[other-ref]: https://example.com (Shakespeare's \"Romeo and Juliet\" is a famous play)\n\n[a]: https://example.com \"\\\\\"\"\n[a]: https://example.com '\\\\\"'\n[a]: https://example.com (\\\\\")\n\n[a]: https://example.com \"\\\\'\"\n[a]: https://example.com '\\\\''\n[a]: https://example.com (\\\\')\n\n[a]: https://example.com \"\\\\'\"\n[a]: https://example.com '\\\\)'\n[a]: https://example.com (\\\\))\n\n[a]: https://example.com \"\\\\\\\\\\\\\"\"\n[a]: https://example.com '\\\\\\\\\\\\''\n[a]: https://example.com (\\\\\\\\\\\\))\n\n[a]: https://example.com \"\\\\\\\\'\"\n[a]: https://example.com '\\\\\\\\\"'\n[a]: https://example.com (\\\\\\\\\")\n\n[a]: https://example.com \"\\\\a\\\\a\"\n[a]: https://example.com '\\\\a\\\\a'\n[a]: https://example.com (\\\\a\\\\a)\n\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\a\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\a\\\\\\\\a)\n\n[a]: https://example.com \"\\\\\\\\\\\\a\\\\\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\\\\\a\\\\\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\\\\\a\\\\\\\\\\\\a)\n\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[ref]: https://example.com \"bar\"\n[other-ref]:\n  https://example.com\n  (Shakespeare's \"Romeo and Juliet\" is a famous play)\n\n[a]: https://example.com \"\\\\\"\" [a]: https://example.com '\\\\\"' [a]:\nhttps://example.com (\\\\\")\n\n[a]: https://example.com \"'\"\n\n[a]: https://example.com '\\\\'' [a]: https://example.com (\\\\')\n\n[a]: https://example.com \"'\"\n[a]: https://example.com \")\"\n\n[a]: https://example.com (\\\\))\n\n[a]: https://example.com \"\\\\\\\\\\\\\"\" [a]: https://example.com '\\\\\\\\\\\\'' [a]:\nhttps://example.com (\\\\\\\\\\\\))\n\n[a]: https://example.com \"'\"\n[a]: https://example.com '\"'\n[a]: https://example.com '\"'\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"");
}
#[test]
fn test_title_md_prose_wrapnever_format_1_cec5c5ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ref]: https://example.com (bar)\n[other-ref]: https://example.com (Shakespeare's \"Romeo and Juliet\" is a famous play)\n\n[a]: https://example.com \"\\\\\"\"\n[a]: https://example.com '\\\\\"'\n[a]: https://example.com (\\\\\")\n\n[a]: https://example.com \"\\\\'\"\n[a]: https://example.com '\\\\''\n[a]: https://example.com (\\\\')\n\n[a]: https://example.com \"\\\\'\"\n[a]: https://example.com '\\\\)'\n[a]: https://example.com (\\\\))\n\n[a]: https://example.com \"\\\\\\\\\\\\\"\"\n[a]: https://example.com '\\\\\\\\\\\\''\n[a]: https://example.com (\\\\\\\\\\\\))\n\n[a]: https://example.com \"\\\\\\\\'\"\n[a]: https://example.com '\\\\\\\\\"'\n[a]: https://example.com (\\\\\\\\\")\n\n[a]: https://example.com \"\\\\a\\\\a\"\n[a]: https://example.com '\\\\a\\\\a'\n[a]: https://example.com (\\\\a\\\\a)\n\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\a\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\a\\\\\\\\a)\n\n[a]: https://example.com \"\\\\\\\\\\\\a\\\\\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\\\\\a\\\\\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\\\\\a\\\\\\\\\\\\a)\n\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com '\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a'\n[a]: https://example.com (\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[ref]: https://example.com \"bar\"\n[other-ref]: https://example.com (Shakespeare's \"Romeo and Juliet\" is a famous play)\n\n[a]: https://example.com \"\\\\\"\" [a]: https://example.com '\\\\\"' [a]: https://example.com (\\\\\")\n\n[a]: https://example.com \"'\"\n\n[a]: https://example.com '\\\\'' [a]: https://example.com (\\\\')\n\n[a]: https://example.com \"'\"\n[a]: https://example.com \")\"\n\n[a]: https://example.com (\\\\))\n\n[a]: https://example.com \"\\\\\\\\\\\\\"\" [a]: https://example.com '\\\\\\\\\\\\'' [a]: https://example.com (\\\\\\\\\\\\))\n\n[a]: https://example.com \"'\"\n[a]: https://example.com '\"'\n[a]: https://example.com '\"'\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\a\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"\n[a]: https://example.com \"\\\\\\\\\\\\\\\\a\\\\\\\\\\\\\\\\a\"");
}
