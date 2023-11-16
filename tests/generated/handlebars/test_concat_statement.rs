#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_concat_statement_hbs_single_quotetrue_format_1_6436a1ac() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div class=\"hello {{if goodbye true}}\">\n  Hello\n</div>\n\n<div class=\"hello {{if goodbye true}} {{if goodbye false}} {{if goodbye true}} {{if goodbye false}} {{if goodbye true}}\">\n  Hello\n</div>\n\n<a href=\"/{{url}}/{{url}}\"></a>\n\n<div class=\"  class-a{{myClass}}\"></div>\n<div class=\" class-b {{myClass}}\"></div>\n<div class=\"    {{myClass}}class-c\"></div>\n<div class=\"  {{myClass}}   class-d\"></div>\n<div class=\" class-e{{myClass}}    class-f\"></div>\n<div class=\"     class-g{{myClass}}class-h    \"></div>\n<div class=\" class-i       {{myClass}}class-j\"></div>\n<div class=\"class-k {{myClass}}   class-l\"></div>\n<div class=\"  class-m   {{myClass}}     class-n {{myClass}}class-o   \"></div>\n<div class=\"  class-p  class-q\"></div") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div class='hello {{if goodbye true}}'>\n  Hello\n</div>\n\n<div\n  class='hello\n    {{if goodbye true}}\n    {{if goodbye false}}\n    {{if goodbye true}}\n    {{if goodbye false}}\n    {{if goodbye true}}'\n>\n  Hello\n</div>\n\n<a href='/{{url}}/{{url}}'></a>\n\n<div class='class-a{{myClass}}'></div>\n<div class='class-b {{myClass}}'></div>\n<div class='{{myClass}}class-c'></div>\n<div class='{{myClass}} class-d'></div>\n<div class='class-e{{myClass}} class-f'></div>\n<div class='class-g{{myClass}}class-h'></div>\n<div class='class-i {{myClass}}class-j'></div>\n<div class='class-k {{myClass}} class-l'></div>\n<div class='class-m {{myClass}} class-n {{myClass}}class-o'></div>\n<div class='class-p class-q'></div");
}
#[test]
fn test_concat_statement_hbs_format_1_6436a1ac() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div class=\"hello {{if goodbye true}}\">\n  Hello\n</div>\n\n<div class=\"hello {{if goodbye true}} {{if goodbye false}} {{if goodbye true}} {{if goodbye false}} {{if goodbye true}}\">\n  Hello\n</div>\n\n<a href=\"/{{url}}/{{url}}\"></a>\n\n<div class=\"  class-a{{myClass}}\"></div>\n<div class=\" class-b {{myClass}}\"></div>\n<div class=\"    {{myClass}}class-c\"></div>\n<div class=\"  {{myClass}}   class-d\"></div>\n<div class=\" class-e{{myClass}}    class-f\"></div>\n<div class=\"     class-g{{myClass}}class-h    \"></div>\n<div class=\" class-i       {{myClass}}class-j\"></div>\n<div class=\"class-k {{myClass}}   class-l\"></div>\n<div class=\"  class-m   {{myClass}}     class-n {{myClass}}class-o   \"></div>\n<div class=\"  class-p  class-q\"></div") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div class=\"hello {{if goodbye true}}\">\n  Hello\n</div>\n\n<div\n  class=\"hello\n    {{if goodbye true}}\n    {{if goodbye false}}\n    {{if goodbye true}}\n    {{if goodbye false}}\n    {{if goodbye true}}\"\n>\n  Hello\n</div>\n\n<a href=\"/{{url}}/{{url}}\"></a>\n\n<div class=\"class-a{{myClass}}\"></div>\n<div class=\"class-b {{myClass}}\"></div>\n<div class=\"{{myClass}}class-c\"></div>\n<div class=\"{{myClass}} class-d\"></div>\n<div class=\"class-e{{myClass}} class-f\"></div>\n<div class=\"class-g{{myClass}}class-h\"></div>\n<div class=\"class-i {{myClass}}class-j\"></div>\n<div class=\"class-k {{myClass}} class-l\"></div>\n<div class=\"class-m {{myClass}} class-n {{myClass}}class-o\"></div>\n<div class=\"class-p class-q\"></div");
}
#[test]
fn test_in_attr_node_hbs_single_quotetrue_format_1_7e7dd9a5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div class=\"a very long list of classes that exceeds {{eighty}} chars with emtpy spaces\">hey</div>\n<a href=\"a-very-long-href-from-a-third-party-marketing-platform{{id}}longer-than-eighty-chars\">Link</a>\n<button\n  class=\"uk-padding-remove uk-button uk-button-default uk-button-small uk-width-1-{{intl.locales.length}} {{if (contains locale intl.locale) \"uk-button-primary\"}}\"\n>\n  test\n</button>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  class='a very long list of classes that exceeds\n    {{eighty}}\n    chars with emtpy spaces'\n>hey</div>\n<a\n  href='a-very-long-href-from-a-third-party-marketing-platform{{id}}longer-than-eighty-chars'\n>Link</a>\n<button\n  class='uk-padding-remove uk-button uk-button-default uk-button-small uk-width-1-{{intl.locales.length}}\n    {{if (contains locale intl.locale) \"uk-button-primary\"}}'\n>\n  test\n</button");
}
#[test]
fn test_in_attr_node_hbs_format_1_7e7dd9a5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div class=\"a very long list of classes that exceeds {{eighty}} chars with emtpy spaces\">hey</div>\n<a href=\"a-very-long-href-from-a-third-party-marketing-platform{{id}}longer-than-eighty-chars\">Link</a>\n<button\n  class=\"uk-padding-remove uk-button uk-button-default uk-button-small uk-width-1-{{intl.locales.length}} {{if (contains locale intl.locale) \"uk-button-primary\"}}\"\n>\n  test\n</button>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  class=\"a very long list of classes that exceeds\n    {{eighty}}\n    chars with emtpy spaces\"\n>hey</div>\n<a\n  href=\"a-very-long-href-from-a-third-party-marketing-platform{{id}}longer-than-eighty-chars\"\n>Link</a>\n<button\n  class=\"uk-padding-remove uk-button uk-button-default uk-button-small uk-width-1-{{intl.locales.length}}\n    {{if (contains locale intl.locale) 'uk-button-primary'}}\"\n>\n  test\n</button");
}
