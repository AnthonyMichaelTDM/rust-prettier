#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_hbs_format_1_e096ebc2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>\n  Welcome to the <strong>Ember.js Guides</strong>.\n  This documentation will take you from total beginner to Ember expert.\n</p>\n\n{{!-- newlines text --}}\n<div>\n  hi\n  there\n  how\n\n  are you\n\n\n  are you fine today?\n</div>\n\n{{!-- newlines text spaced --}}\n<div>\n\n  space above\n\n  space below\n\n</div>\n\n{{!-- newlines elems spaced --}}\n<div>\n\n  <span>space above</span>\n\n  <span>space below</span>\n\n</div>\n\n{{!-- newlines mixed --}}\n<div>\n  hi\n  <span>there</span>\n\n  how\n\n  are <strong>you</strong>\n\n\n  are you fine today?\n</div>\n\n{{!-- newlines elems --}}\n<div>\n  <div>\n\n\n    <div></div>\n\n\n  </div>\n\n\n  hi\n\n\n  <div></div>\n\n\n  <Big />\n\n\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>\n  Welcome to the\n  <strong>Ember.js Guides</strong>. This documentation will take you from total\n  beginner to Ember expert.\n</p>\n\n{{! newlines text }}\n<div>\n  hi there how are you are you fine today?\n</div>\n\n{{! newlines text spaced }}\n<div>\n\n  space above space below\n\n</div>\n\n{{! newlines elems spaced }}\n<div>\n\n  <span>space above</span>\n\n  <span>space below</span>\n\n</div>\n\n{{! newlines mixed }}\n<div>\n  hi\n  <span>there</span>\n\n  how are\n  <strong>you</strong>\n\n  are you fine today?\n</div>\n\n{{! newlines elems }}\n<div>\n  <div>\n\n    <div></div>\n\n  </div>\n\n  hi\n\n  <div></div>\n\n  <Big />\n\n</div");
}
