#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_prettier_ignore_hbs_format_1_f62867a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{! prettier-ignore }}\n{{        ugly}}\n\n{{! prettier-ignore }}{{        ugly}}\n\n{{! prettier-ignore }}\n        ugly\n\n{{! prettier-ignore }}          ugly\n\n{{! prettier-ignore }}\n{{#        ugly}}\n{{/        ugly}}\n\n{{! prettier-ignore }}{{#        ugly}}{{/        ugly}}\n\n{{! prettier-ignore }}\n{{!        ugly}}\n\n{{! prettier-ignore }}{{!        ugly}}\n\n{{! prettier-ignore }}\n<div>\n  \"hello! my parent was ignored\"\n  {{#my-crazy-component     \"shall\"     be=\"preserved\"}}\n    <This\n\n      is=\"preserved\"\n    />\n  {{/my-crazy-component}}\n</div>\n\n{{#a-normal-component    isRestoredTo   = \"order\"    }}\n  <ThisWillBeNormal backTo    =   \"normal\" />\n{{/a-normal-component}}\n\n<div     class=\"this should be fixed\"          >\n  <Sibling isFixed   =  {{true}}      />\n  {{! prettier-ignore }}\n  <div  class = \" left in chaos as well as children   \"     >\n\n      <SomeComponent @broken =   {{    true    }}></SomeComponent>\n\n\n  </div>\n  {{#another-sibling    as     |isFixed|}}\n\n    <div class =    \"fixed\"   >{{   isFixed   }}\n\n        All of this should be fixed\n\n    </div>\n\n  {{/another-sibling}}\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{! prettier-ignore }}\n{{        ugly}}\n\n{{! prettier-ignore }}{{ugly}}\n\n{{! prettier-ignore }}\nugly\n\n{{! prettier-ignore }}\nugly\n\n{{! prettier-ignore }}\n{{#        ugly}}\n{{/        ugly}}\n\n{{! prettier-ignore }}{{#ugly}}{{/ugly}}\n\n{{! prettier-ignore }}\n{{!        ugly}}\n\n{{! prettier-ignore }}{{!        ugly}}\n\n{{! prettier-ignore }}\n<div>\n  \"hello! my parent was ignored\"\n  {{#my-crazy-component     \"shall\"     be=\"preserved\"}}\n    <This\n\n      is=\"preserved\"\n    />\n  {{/my-crazy-component}}\n</div>\n\n{{#a-normal-component isRestoredTo=\"order\"}}\n  <ThisWillBeNormal backTo=\"normal\" />\n{{/a-normal-component}}\n\n<div class=\"this should be fixed\">\n  <Sibling isFixed={{true}} />\n  {{! prettier-ignore }}\n  <div  class = \" left in chaos as well as children   \"     >\n\n      <SomeComponent @broken =   {{    true    }}></SomeComponent>\n\n\n  </div>\n  {{#another-sibling as |isFixed|}}\n\n    <div class=\"fixed\">{{isFixed}}\n\n      All of this should be fixed\n\n    </div>\n\n  {{/another-sibling}}\n</div");
}
