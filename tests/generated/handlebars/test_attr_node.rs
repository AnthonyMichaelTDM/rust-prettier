#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attribute_hbs_format_1_4e9f7525() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div title='\n\n  a title with random\n\n\n  newlines'></div>\n\n<div TITLE='\n\n  a title with random\n\n\n  newlines'></div>\n\n<div title='\n\n  a title with random\n\n\n  newlines\n  and {{mustaches}}\n\n'></div>\n\n<div TITLE='\n\n  a title with random\n\n\n  newlines\n  and {{mustaches}}\n\n'></div>") ? ;
    assert_eq ! (formatted , "<div\n  title=\"\n\n  a title with random\n\n\n  newlines\"\n></div>\n\n<div\n  TITLE=\"\n\n  a title with random\n\n\n  newlines\"\n></div>\n\n<div\n  title=\"\n\n  a title with random\n\n\n  newlines\n  and {{mustaches}}\n\n\"\n></div>\n\n<div\n  TITLE=\"\n\n  a title with random\n\n\n  newlines\n  and {{mustaches}}\n\n\"\n></div");
    Ok(())
}
#[test]
fn test_class_name_hbs_format_1_76790309() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div class='class and other'></div>\n<div class=' class and other '></div>\n<div CLASs=' class and other '></div>\n<p class=\" a list of\n\n\nweirdly formatted\n\n\nclasses\">String</p>\n<p class=\" a list of\n\n\nweirdly formatted\n\n\n{{this.classes}}\">String</p>\n<p CLASS=\" a list of\n\n\nweirdly formatted\n\n\n{{this.classes}}\">String</p>\n\n<Foo class=\"\n\"/>") ? ;
    assert_eq ! (formatted , "<div class=\"class and other\"></div>\n<div class=\"class and other\"></div>\n<div CLASs=\"class and other\"></div>\n<p class=\"a list of weirdly formatted classes\">String</p>\n<p class=\"a list of weirdly formatted {{this.classes}}\">String</p>\n<p CLASS=\"a list of weirdly formatted {{this.classes}}\">String</p>\n\n<Foo class=\"\" /");
    Ok(())
}
#[test]
fn test_order_hbs_format_1_3bf15b5e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<MyComponent\n  {{! this is a comment for arg 1}}\n  @arg1=\"hello\"\n  {{on \"clik\" this.modify}}\n  @arg2=\"hello\"\n  {{! this is a comment for arg 3}}\n  @arg3=\"hello\"\n  @arg4=\"hello\"\n  {{! this is a comment for arg 5}}\n  @arg5=\"hello\"\n  ...arguments\n/>") ? ;
    assert_eq ! (formatted , "<MyComponent\n  {{! this is a comment for arg 1}}\n  @arg1=\"hello\"\n  {{on \"clik\" this.modify}}\n  @arg2=\"hello\"\n  {{! this is a comment for arg 3}}\n  @arg3=\"hello\"\n  @arg4=\"hello\"\n  {{! this is a comment for arg 5}}\n  @arg5=\"hello\"\n  ...arguments\n/");
    Ok(())
}
#[test]
fn test_quotes_hbs_format_1_980a1b89() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<div title=\"'' {{ foo }}\"></div>\n<div title='\"\" {{ bar }}'></div>")?;
    assert_eq!(
        formatted,
        "<div title=\"'' {{foo}}\"></div>\n<div title='\"\" {{bar}}'></div"
    );
    Ok(())
}
