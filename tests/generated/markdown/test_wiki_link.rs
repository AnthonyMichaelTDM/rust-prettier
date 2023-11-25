use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_additional_spacing_md_prose_wrapalways_format_1_909eb6c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[Additional spacing    within the link should be preserved]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Additional spacing    within the link should be preserved]]"
    );
}
#[test]
fn test_additional_spacing_md_prose_wrapnever_format_1_909eb6c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[Additional spacing    within the link should be preserved]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Additional spacing    within the link should be preserved]]"
    );
}
#[test]
fn test_additional_spacing_md_prose_wrappreserve_format_1_909eb6c0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[Additional spacing    within the link should be preserved]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Additional spacing    within the link should be preserved]]"
    );
}
#[test]
fn test_end_of_line_md_prose_wrapalways_format_1_3a342cb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("If I have some markdown text, it should be wrapped properly at the character limit for markdown.\nHowever, if I have a link that overflows the end of line it should be [[wrapped as a single entity]] like this.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "If I have some markdown text, it should be wrapped properly at the character\nlimit for markdown. However, if I have a link that overflows the end of line it\nshould be [[wrapped as a single entity]] like this.");
}
#[test]
fn test_end_of_line_md_prose_wrapnever_format_1_3a342cb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("If I have some markdown text, it should be wrapped properly at the character limit for markdown.\nHowever, if I have a link that overflows the end of line it should be [[wrapped as a single entity]] like this.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "If I have some markdown text, it should be wrapped properly at the character limit for markdown. However, if I have a link that overflows the end of line it should be [[wrapped as a single entity]] like this.");
}
#[test]
fn test_end_of_line_md_prose_wrappreserve_format_1_3a342cb0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("If I have some markdown text, it should be wrapped properly at the character limit for markdown.\nHowever, if I have a link that overflows the end of line it should be [[wrapped as a single entity]] like this.") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "If I have some markdown text, it should be wrapped properly at the character limit for markdown.\nHowever, if I have a link that overflows the end of line it should be [[wrapped as a single entity]] like this.");
}
#[test]
fn test_exceeds_line_length_md_prose_wrapalways_format_1_b4061339() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].");
}
#[test]
fn test_exceeds_line_length_md_prose_wrapnever_format_1_b4061339() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].");
}
#[test]
fn test_exceeds_line_length_md_prose_wrappreserve_format_1_b4061339() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[[Here is an incredibly long wiki-style link that overflows the standard wrap width for markdown]].");
}
#[test]
fn test_exceeds_line_length_in_prose_md_prose_wrapalways_format_1_742c1f44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that\n[[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_exceeds_line_length_in_prose_md_prose_wrapnever_format_1_742c1f44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_exceeds_line_length_in_prose_md_prose_wrappreserve_format_1_742c1f44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_exceeds_line_length_in_prose_broken_md_prose_wrapalways_format_1_d42641e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I\ncontinue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that\n[[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_exceeds_line_length_in_prose_broken_md_prose_wrapnever_format_1_d42641e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I\ncontinue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I continue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_exceeds_line_length_in_prose_broken_md_prose_wrappreserve_format_1_d42641e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I\ncontinue it into an obscenely long wiki-style link thingy]].") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "I have some markdown prose here, with a horrible run-on sentence that [[makes little sense at all as I\ncontinue it into an obscenely long wiki-style link thingy]].");
}
#[test]
fn test_extra_brackets_md_prose_wrapalways_format_1_c14b2378() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over the [[[end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over the\n[[[end like this]]].");
}
#[test]
fn test_extra_brackets_md_prose_wrapnever_format_1_c14b2378() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over the [[[end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over the [[[end like this]]].");
}
#[test]
fn test_extra_brackets_md_prose_wrappreserve_format_1_c14b2378() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over the [[[end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over the [[[end like this]]].");
}
#[test]
fn test_extra_brackets_leading_md_prose_wrapalways_format_1_b8dfc396() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[[the end like this]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over\n[[[the end like this]].");
}
#[test]
fn test_extra_brackets_leading_md_prose_wrapnever_format_1_b8dfc396() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[[the end like this]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over [[[the end like this]].");
}
#[test]
fn test_extra_brackets_leading_md_prose_wrappreserve_format_1_b8dfc396() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[[the end like this]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over [[[the end like this]].");
}
#[test]
fn test_extra_brackets_trailing_md_prose_wrapalways_format_1_ccdde741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[the end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over\n[[the end like this]]].");
}
#[test]
fn test_extra_brackets_trailing_md_prose_wrapnever_format_1_ccdde741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[the end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over [[the end like this]]].");
}
#[test]
fn test_extra_brackets_trailing_md_prose_wrappreserve_format_1_ccdde741() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("A very long line of markdown with additional brackets as it wraps over [[the end like this]]].\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "A very long line of markdown with additional brackets as it wraps over [[the end like this]]].");
}
#[test]
fn test_multi_line_md_prose_wrapalways_format_1_b398fbde() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[a\nb]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[a b]]");
}
#[test]
fn test_multi_line_md_prose_wrapnever_format_1_b398fbde() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[a\nb]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[a b]]");
}
#[test]
fn test_multi_line_md_prose_wrappreserve_format_1_b398fbde() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[a\nb]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[a\nb]]");
}
#[test]
fn test_nested_link_md_prose_wrapalways_format_1_9cb44fd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Here's some text to ensure that the link and wiki link break the line [[a[b](http://www.example.com/)]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Here's some text to ensure that the link and wiki link break the line\n[[a[b](http://www.example.com/)]]");
}
#[test]
fn test_nested_link_md_prose_wrapnever_format_1_9cb44fd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Here's some text to ensure that the link and wiki link break the line [[a[b](http://www.example.com/)]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Here's some text to ensure that the link and wiki link break the line [[a[b](http://www.example.com/)]]");
}
#[test]
fn test_nested_link_md_prose_wrappreserve_format_1_9cb44fd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Here's some text to ensure that the link and wiki link break the line [[a[b](http://www.example.com/)]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Here's some text to ensure that the link and wiki link break the line [[a[b](http://www.example.com/)]]");
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_420b8e05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[A simple wiki link on a single line]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[A simple wiki link on a single line]]");
}
#[test]
fn test_simple_md_prose_wrapnever_format_1_420b8e05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[A simple wiki link on a single line]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[A simple wiki link on a single line]]");
}
#[test]
fn test_simple_md_prose_wrappreserve_format_1_420b8e05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[[A simple wiki link on a single line]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[[A simple wiki link on a single line]]");
}
#[test]
fn test_with_whitespace_md_prose_wrapalways_format_1_735e1269() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[ Here is a link with leading and trailing whitespace.    ]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Here is a link with leading and trailing whitespace.]]"
    );
}
#[test]
fn test_with_whitespace_md_prose_wrapnever_format_1_735e1269() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[ Here is a link with leading and trailing whitespace.    ]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Here is a link with leading and trailing whitespace.]]"
    );
}
#[test]
fn test_with_whitespace_md_prose_wrappreserve_format_1_735e1269() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("[[ Here is a link with leading and trailing whitespace.    ]]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[Here is a link with leading and trailing whitespace.]]"
    );
}
