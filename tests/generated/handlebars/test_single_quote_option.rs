#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_arguments_hbs_single_quotefalse_format_1_18ad1672() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aProp (and this.bProp (get bike \"number\" \"message\"))}}\n/>\n\n<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or this.aProp (and this.bProp (get bike 'number' 'message'))}}\n/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aProp (and this.bProp (get bike \"number\" \"message\"))}}\n/>\n\n<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aProp (and this.bProp (get bike \"number\" \"message\"))}}\n/");
}
#[test]
fn test_arguments_hbs_single_quotetrue_format_1_18ad1672() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<GlimmerComponent\n  class=\"medium\"\n  @autocomplete=\"off\"\n  @errors={{or this.aProp (and this.bProp (get bike \"number\" \"message\"))}}\n/>\n\n<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or this.aProp (and this.bProp (get bike 'number' 'message'))}}\n/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or this.aProp (and this.bProp (get bike 'number' 'message'))}}\n/>\n\n<GlimmerComponent\n  class='medium'\n  @autocomplete='off'\n  @errors={{or this.aProp (and this.bProp (get bike 'number' 'message'))}}\n/");
}
#[test]
fn test_attributes_hbs_single_quotefalse_format_1_c019b94f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div title='My title'></div>\n<div title=\"My title\"></div>\n<div title='My \"title\"'></div>\n<div title=\"My other 'title'\"></div>\n\n<div title=\"{{t 'my.title'}}\" />\n<div title='{{t \"my.title\"}}' />\n\n<a href=\"/{{url}}/{{url}}\"></a>\n<a href='/{{url}}/{{url}}'></a>\n\n<button class=\"padding width-{{locales.length}} {{if (has locale) \"uk-button-primary\"}}\"></button>\n<button class=\"padding width-{{locales.length}} {{if (has locale) 'uk-button-primary'}}\"></button>\n<button class='padding width-{{locales.length}} {{if (has locale) \"uk-button-primary\"}}'></button>\n<button class='padding width-{{locales.length}} {{if (has locale) 'uk-button-primary'}}'></button>\n\n<div class=\"padding {{if foo (if fooAgain \"bar\" (if fooAgainAgain \"bar\" \"foo\"))}} baz\" />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div title=\"My title\"></div>\n<div title=\"My title\"></div>\n<div title='My \"title\"'></div>\n<div title=\"My other 'title'\"></div>\n\n<div title=\"{{t 'my.title'}}\" />\n<div title=\"{{t 'my.title'}}\" />\n\n<a href=\"/{{url}}/{{url}}\"></a>\n<a href=\"/{{url}}/{{url}}\"></a>\n\n<button\n  class=\"padding width-{{locales.length}}\n    {{if (has locale) 'uk-button-primary'}}\"\n></button>\n<button\n  class=\"padding width-{{locales.length}}\n    {{if (has locale) 'uk-button-primary'}}\"\n></button>\n<button\n  class=\"padding width-{{locales.length}}\n    {{if (has locale) 'uk-button-primary'}}\"\n></button>\n<button\n  class=\"padding width-{{locales.length}}\n    {{if (has locale) 'uk-button-primary'}}\"\n></button>\n\n<div\n  class=\"padding\n    {{if foo (if fooAgain 'bar' (if fooAgainAgain 'bar' 'foo'))}}\n    baz\"\n/");
}
#[test]
fn test_attributes_hbs_single_quotetrue_format_1_c019b94f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div title='My title'></div>\n<div title=\"My title\"></div>\n<div title='My \"title\"'></div>\n<div title=\"My other 'title'\"></div>\n\n<div title=\"{{t 'my.title'}}\" />\n<div title='{{t \"my.title\"}}' />\n\n<a href=\"/{{url}}/{{url}}\"></a>\n<a href='/{{url}}/{{url}}'></a>\n\n<button class=\"padding width-{{locales.length}} {{if (has locale) \"uk-button-primary\"}}\"></button>\n<button class=\"padding width-{{locales.length}} {{if (has locale) 'uk-button-primary'}}\"></button>\n<button class='padding width-{{locales.length}} {{if (has locale) \"uk-button-primary\"}}'></button>\n<button class='padding width-{{locales.length}} {{if (has locale) 'uk-button-primary'}}'></button>\n\n<div class=\"padding {{if foo (if fooAgain \"bar\" (if fooAgainAgain \"bar\" \"foo\"))}} baz\" />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div title='My title'></div>\n<div title='My title'></div>\n<div title='My \"title\"'></div>\n<div title=\"My other 'title'\"></div>\n\n<div title='{{t \"my.title\"}}' />\n<div title='{{t \"my.title\"}}' />\n\n<a href='/{{url}}/{{url}}'></a>\n<a href='/{{url}}/{{url}}'></a>\n\n<button\n  class='padding width-{{locales.length}}\n    {{if (has locale) \"uk-button-primary\"}}'\n></button>\n<button\n  class='padding width-{{locales.length}}\n    {{if (has locale) \"uk-button-primary\"}}'\n></button>\n<button\n  class='padding width-{{locales.length}}\n    {{if (has locale) \"uk-button-primary\"}}'\n></button>\n<button\n  class='padding width-{{locales.length}}\n    {{if (has locale) \"uk-button-primary\"}}'\n></button>\n\n<div\n  class='padding\n    {{if foo (if fooAgain \"bar\" (if fooAgainAgain \"bar\" \"foo\"))}}\n    baz'\n/");
}
#[test]
fn test_string_literals_hbs_single_quotefalse_format_1_a4399b89() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("{{\"abc\"}}\n{{'abc'}}\n{{\" \\\\\" \\\\\" ' more double quote than single quote \"}}\n{{' \\\\' \\\\' \" more single quote than double quote '}}\n{{' \" \\\\' \\\\\" \\\\\\\\ '}}\n{{\" \\\\\" \\\\' ' \\\\\\\\ \"}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{\"abc\"}}\n{{\"abc\"}}\n{{' \" \" \\\\' more double quote than single quote '}}\n{{\" ' ' \\\\\" more single quote than double quote \"}}\n{{' \" \\\\' \\\\\" \\\\\\\\ '}}\n{{\" \\\\\" \\\\' ' \\\\\\\\ \"}");
}
#[test]
fn test_string_literals_hbs_single_quotetrue_format_1_a4399b89() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("{{\"abc\"}}\n{{'abc'}}\n{{\" \\\\\" \\\\\" ' more double quote than single quote \"}}\n{{' \\\\' \\\\' \" more single quote than double quote '}}\n{{' \" \\\\' \\\\\" \\\\\\\\ '}}\n{{\" \\\\\" \\\\' ' \\\\\\\\ \"}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{'abc'}}\n{{'abc'}}\n{{' \" \" \\\\' more double quote than single quote '}}\n{{\" ' ' \\\\\" more single quote than double quote \"}}\n{{' \" \\\\' \\\\\" \\\\\\\\ '}}\n{{\" \\\\\" \\\\' ' \\\\\\\\ \"}");
}
