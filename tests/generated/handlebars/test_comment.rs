use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_hbs_format_1_350e6b15() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  {{! Foo }}\n  {{#if @foo}}\n    Foo\n  {{/if}}\n\n  {{! Bar }}\n  {{#if @bar}}\n    Bar\n  {{/if}}\n</div>\n\n<div class=\"entry\">\n  {{! This comment will not be in the output }}\n  {{!-- This comment as }} and will not be in the output --}}\n  {{~! This comment will not be in the output }}\n  {{! This comment will not be in the output ~}}\n  {{~! This comment will not be in the output ~}}\n  {{~!\ncomment\n ~}}\n  {{~! 1 ~}}{{~! 2 ~}}\n  <!-- This comment will be in the output -->\n</div>\n\n<div>\n  {{~!~}}\n  <span></span>\n  {{~!~}}\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div>\n  {{! Foo }}\n  {{#if @foo}}\n    Foo\n  {{/if}}\n\n  {{! Bar }}\n  {{#if @bar}}\n    Bar\n  {{/if}}\n</div>\n\n<div class=\"entry\">\n  {{! This comment will not be in the output }}\n  {{!-- This comment as }} and will not be in the output --}}\n  {{~! This comment will not be in the output }}\n  {{! This comment will not be in the output ~}}\n  {{~! This comment will not be in the output ~}}\n  {{~!\ncomment\n ~}}\n  {{~! 1 ~}}{{~! 2 ~}}\n  <!-- This comment will be in the output -->\n</div>\n\n<div>\n  {{~!~}}\n  <span></span>\n  {{~!~}}\n</div");
}
