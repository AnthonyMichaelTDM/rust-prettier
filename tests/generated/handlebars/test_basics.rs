#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_component_hbs_format_1_05027681() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<UserGreeting @name=\"Ricardo\" @greeting=\"Olá\" />\n{{@greeting}}, {{@name}}!\n\n<div>\n  <UserGreeting\n    @aVeryLongArgumentNameThatIsStillGoing={{@alsoAVeryLongArgument}}\n  />\n</div>\n\n<Form as |f|>\n  <f.input @title=\"hello\" />\n  <f.input>hello</f.input>\n</Form>\n\n<this.label @title=\"hello\" />\n\n<button onclick={{action next}}>Next</button>\n\n<button disabled class=\"disabled\"></button>\n\n<button disabled=disabled class=\"disabled\"></button>\n\n<img alt=\"\" />\n\n<div ...attributes>Hello</div>\n\n<ul class=\"list-unstyled\n  one-tab\">\n</ul") ? ;
    assert_eq ! (formatted , "<UserGreeting @name=\"Ricardo\" @greeting=\"Olá\" />\n{{@greeting}},\n{{@name}}!\n\n<div>\n  <UserGreeting\n    @aVeryLongArgumentNameThatIsStillGoing={{@alsoAVeryLongArgument}}\n  />\n</div>\n\n<Form as |f|>\n  <f.input @title=\"hello\" />\n  <f.input>hello</f.input>\n</Form>\n\n<this.label @title=\"hello\" />\n\n<button onclick={{action next}}>Next</button>\n\n<button disabled class=\"disabled\"></button>\n\n<button disabled=\"disabled\" class=\"disabled\"></button>\n\n<img alt=\"\" />\n\n<div ...attributes>Hello</div>\n\n<ul class=\"list-unstyled one-tab\">\n</ul");
    Ok(())
}
#[test]
fn test_literals_hbs_format_1_4fd29d7f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{mustache true}}\n{{mustache 5}}\n{{mustache undefined}}\n{{mustache null}}\n<!-- hello world -->\n{{! Mustache Comment}}\n{{!-- Mustache Comment }} --}}") ? ;
    assert_eq ! (formatted , "{{mustache true}}\n{{mustache 5}}\n{{mustache undefined}}\n{{mustache null}}\n<!-- hello world -->\n{{! Mustache Comment}}\n{{!-- Mustache Comment }} --}");
    Ok(())
}
#[test]
fn test_named_block_hbs_format_1_a46fb6d7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<ComponentWithNamedBlocks>\n  <:first-named-block></:first-named-block>\n  <:second-named-block> </:second-named-block>\n  <:named-block-with-comment>{{! Do not convert to an empty element}}</:named-block-with-comment>\n  <:named-block-with-content>Hello</:named-block-with-content>\n</ComponentWithNamedBlocks>") ? ;
    assert_eq ! (formatted , "<ComponentWithNamedBlocks>\n  <:first-named-block></:first-named-block>\n  <:second-named-block> </:second-named-block>\n  <:named-block-with-comment\n  >{{! Do not convert to an empty element}}</:named-block-with-comment>\n  <:named-block-with-content>Hello</:named-block-with-content>\n</ComponentWithNamedBlocks");
    Ok(())
}
#[test]
fn test_nested_path_hbs_format_1_90cb90bf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div class=\"entry\">\n  <h1>{{title}}</h1>\n  <h2>By {{author.name}}</h2>\n\n  <div class=\"body\">\n    {{body}}\n  </div>\n</div>") ? ;
    assert_eq ! (formatted , "<div class=\"entry\">\n  <h1>{{title}}</h1>\n  <h2>By {{author.name}}</h2>\n\n  <div class=\"body\">\n    {{body}}\n  </div>\n</div");
    Ok(())
}
#[test]
fn test_raw_hbs_format_1_96e82d97() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p>{{{raw}}}</p>")?;
    assert_eq!(formatted, "<p>{{{raw}}}</p");
    Ok(())
}
