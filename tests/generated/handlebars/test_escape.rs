#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_backslashes_hbs_format_1_72d6de8e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>\\\\</p>\n<p>\\\\\\\\</p>\n<p>\\\\\\\\\\\\</p>\n<p>\\\\\\\\\\\\ \\\\\\\\{{non-escaped-moustache}}</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>\\\\</p>\n<p>\\\\\\\\</p>\n<p>\\\\\\\\\\\\</p>\n<p>\\\\\\\\\\\\ \\\\\\\\{{non-escaped-moustache}}</p");
}
#[test]
fn test_backslashes_in_attributes_hbs_format_1_64e791cb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p data-attr=\"backslash \\\\ in an attribute\"></p>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<p data-attr=\"backslash \\\\ in an attribute\"></p"
    );
}
#[test]
fn test_html_entities_hbs_format_1_d05e734c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p>Some escaped characters: &lt; &gt; &amp;</p");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<p>Some escaped characters: &lt; &gt; &amp;</p");
}
#[test]
fn test_mustache_hbs_format_1_ce3c1b2d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<div title=\"foo \\\\{{\"> \\\\{{ </div>\n<div class=\" bar \\\\{{\">\\\\{{</div>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<div title=\"foo \\\\{{\"> \\\\{{ </div>\n<div class=\"bar \\\\{{\">\\\\{{</div"
    );
}
#[test]
fn test_numeric_entities_hbs_format_1_84d76e9c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span class=\"stampFont\" style=\"font-family: 'stampfont'\">&#xf000;</span>\n\nΣ: &#931; &#0931; &#x3A3; &#x03A3; &#x3a3;\n\n[&amp;nbsp;] [&amp;#160;]\n\n<div title=\"A &amp; &#931;\"></div>\n\n<a title=\"&#38;#160;\"></a>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span class=\"stampFont\" style=\"font-family: 'stampfont'\">&#xf000;</span>\n\nΣ: &#931; &#0931; &#x3A3; &#x03A3; &#x3a3; [&amp;nbsp;] [&amp;#160;]\n\n<div title=\"A &amp; &#931;\"></div>\n\n<a title=\"&#38;#160;\"></a");
}
