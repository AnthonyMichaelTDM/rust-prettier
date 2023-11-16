#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_inline_javascript_less_format_1_2ac1927b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Deprecated feature https://lesscss.org/usage/#less-options-enable-inline-javascript-deprecated-\n\n.calcPxMixin() {\n  @functions: ~\\`(function() {\n    const designWidth = 3840\n    const actualWidth = 5760\n    this.calcPx = function(_) {\n      return _ * actualWidth / designWidth + 'px'\n    }\n  })()\\`\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Deprecated feature https://lesscss.org/usage/#less-options-enable-inline-javascript-deprecated-\n\n.calcPxMixin() {\n  @functions: ~\\`(function() {\n    const designWidth = 3840\n    const actualWidth = 5760\n    this.calcPx = function(_) {\n      return _ * actualWidth / designWidth + 'px'\n    }\n  })()\\`;\n}");
}
