#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_backslashes_hbs_format_1_72d6de8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "<p>\\</p>\n<p>\\\\</p>\n<p>\\\\\\</p>\n<p>\\\\\\ \\\\{{non-escaped-moustache}}</p>",
    )?;
    assert_eq!(
        formatted,
        "<p>\\</p>\n<p>\\\\</p>\n<p>\\\\\\</p>\n<p>\\\\\\ \\\\{{non-escaped-moustache}}</p"
    );
    Ok(())
}
#[test]
fn test_backslashes_in_attributes_hbs_format_1_64e791cb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p data-attr=\"backslash \\ in an attribute\"></p>")?;
    assert_eq!(
        formatted,
        "<p data-attr=\"backslash \\ in an attribute\"></p"
    );
    Ok(())
}
#[test]
fn test_html_entities_hbs_format_1_d05e734c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p>Some escaped characters: &lt; &gt; &amp;</p")?;
    assert_eq!(formatted, "<p>Some escaped characters: &lt; &gt; &amp;</p");
    Ok(())
}
#[test]
fn test_mustache_hbs_format_1_ce3c1b2d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<div title=\"foo \\{{\"> \\{{ </div>\n<div class=\" bar \\{{\">\\{{</div>")?;
    assert_eq!(
        formatted,
        "<div title=\"foo \\{{\"> \\{{ </div>\n<div class=\"bar \\{{\">\\{{</div"
    );
    Ok(())
}
#[test]
fn test_numeric_entities_hbs_format_1_84d76e9c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span class=\"stampFont\" style=\"font-family: 'stampfont'\">&#xf000;</span>\n\nΣ: &#931; &#0931; &#x3A3; &#x03A3; &#x3a3;\n\n[&amp;nbsp;] [&amp;#160;]\n\n<div title=\"A &amp; &#931;\"></div>\n\n<a title=\"&#38;#160;\"></a>") ? ;
    assert_eq ! (formatted , "<span class=\"stampFont\" style=\"font-family: 'stampfont'\">&#xf000;</span>\n\nΣ: &#931; &#0931; &#x3A3; &#x03A3; &#x3a3; [&amp;nbsp;] [&amp;#160;]\n\n<div title=\"A &amp; &#931;\"></div>\n\n<a title=\"&#38;#160;\"></a");
    Ok(())
}
