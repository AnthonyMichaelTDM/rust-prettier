#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_crlf_less_format_1_940b6a06() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@nice-blue: #5B83AD;\n@light-blue: @nice-blue + #111;\n\n// Comment 1\n/*\n * Comment 2\n */\n#header {\n  // Comment 3\n  /*\n   * Comment 4\n   */\n  color: @light-blue;\n}\n\n@media only screen and (max-width: 600px) {\n  // Comment 1\n  /*\n   * Comment 5\n   */\n  body {\n    background-color: lightblue; // Comment 6\n    color: red; /* Comment 7 */\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@nice-blue: #5b83ad;\n@light-blue: @nice-blue + #111;\n\n// Comment 1\n/*\n * Comment 2\n */\n#header {\n  // Comment 3\n  /*\n   * Comment 4\n   */\n  color: @light-blue;\n}\n\n@media only screen and (max-width: 600px) {\n  // Comment 1\n  /*\n   * Comment 5\n   */\n  body {\n    background-color: lightblue; // Comment 6\n    color: red; /* Comment 7 */\n  }\n}");
}
#[test]
fn test_between_decl_less_format_1_7afc9b11() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("selector {\nprop: // comment\nvalue;\n\nprop: /* block */ value;\n\nprop\n: value;\n}\n\n// #5603\n.grid {\n        grid-template-areas: //\n          \"header header header\" //\n          \"sidebar content content\" //\n          \"footer footer footer\";\n\n        grid-template-areas:\n          \"header header header\" //\n          \"sidebar content content\" //\n          \"footer footer footer\";\n }\n\n// TODO: make these pretty\nselector {\nprop:\n/* block */\nvalue;\n\nprop/* block */:\nvalue;\n\nprop\n/* block */\n:\nvalue;\n\nprop/* before */: // after\nvalue;\n\n\nprop/* before */: /* after*/\nvalue;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "selector {\n  prop: // comment\n    value;\n\n  prop: /* block */ value;\n\n  prop: value;\n}\n\n// #5603\n.grid {\n  grid-template-areas: //\n    \"header header header\" //\n    \"sidebar content content\" //\n    \"footer footer footer\";\n\n  grid-template-areas: \"header header header\" //\n    \"sidebar content content\" //\n    \"footer footer footer\";\n}\n\n// TODO: make these pretty\nselector {\n  prop:\n/* block */ value;\n\n  prop/* block */: value;\n\n  prop/* block */\n: value;\n\n  prop/* before */: // after\n    value;\n\n  prop/* before */: /* after*/ value;\n}");
}
#[test]
fn test_block_less_format_1_945b75f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// Animation definitions cannot cross the shadow boundary,\n// and thus need to be loaded directly into the atom-text-editor scope.\n/* Kikoo */\n/**\n * Kikoo\n */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Animation definitions cannot cross the shadow boundary,\n// and thus need to be loaded directly into the atom-text-editor scope.\n/* Kikoo */\n/**\n * Kikoo\n */");
}
#[test]
fn test_block_2_less_format_1_f658e26b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* Kikoo */\n/**\n * Kikoo\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* Kikoo */\n/**\n * Kikoo\n */");
}
#[test]
fn test_in_value_less_format_1_9139fb64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".real-world-example {\n  background: radial-gradient(\n      circle at left 0% bottom $position,\n      transparent,\n      transparent $diameter,\n      #fbfbfb calc(#{$diameter} + 1px) // Add 1px for edge-smoothing.\n    );\n}\n\n.simplification { \n  foo: (\n    calc() // not a comment anymore\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".real-world-example {\n  background: radial-gradient(\n    circle at left 0% bottom $position,\n    transparent,\n    transparent $diameter,\n    #fbfbfb calc(#{$diameter} + 1px) // Add 1px for edge-smoothing.\n  );\n}\n\n.simplification {\n  foo: (\n    calc() // not a comment anymore\n  );\n}");
}
#[test]
fn test_issue_8130_less_format_1_edc06c26() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@import \"../node_modules/foo/bar.less\";\n\n// @index(['./components/**/*.less', '!./**/_demo/**'], f => \\`@import '\\${f.path}\\${f.ext}';\\`)\n@import './components/Button/Button.less';\n@import './components/Form/Form.less';\n@import './components/Input/Input.less';\n// @endindex") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@import \"../node_modules/foo/bar.less\";\n\n// @index(['./components/**/*.less', '!./**/_demo/**'], f => \\`@import '\\${f.path}\\${f.ext}';\\`)\n@import \"./components/Button/Button.less\";\n@import \"./components/Form/Form.less\";\n@import \"./components/Input/Input.less\";\n// @endindex");
}
#[test]
fn test_mixed_less_format_1_a2bbf8f2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import 'b';\n// another comment\n//no-space before") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import \"b\";\n// another comment\n//no-space before");
}
#[test]
fn test_mixed_2_less_format_1_9d388255() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n@import \"a\";\n//*\n@import 'b';\n/* block */\n/*no-space block*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n@import \"a\";\n//*\n@import \"b\";\n/* block */\n/*no-space block*/");
}
#[test]
fn test_mixed_block_less_format_1_eb71963e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import 'b';\n/* block */\n/*no-space block*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import \"b\";\n/* block */\n/*no-space block*/");
}
#[test]
fn test_places_less_format_1_ea9ad1d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("div {\n  // a\n\n  margin-left: -@leftMargin; // b\n} // c\n\n// d\ndiv {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "div {\n  // a\n\n  margin-left: -@leftMargin; // b\n} // c\n\n// d\ndiv {\n}"
    );
}
#[test]
fn test_prettier_ignore_less_format_1_18980157() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// prettier-ignore\n@blue:  blue;\n@black: darkgray;\n\nfoo {\n  /* prettier-ignore */\n  thing:     foo;\n  -ms-thing: foo;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// prettier-ignore\n@blue:  blue;\n@black: darkgray;\n\nfoo {\n  /* prettier-ignore */\n  thing:     foo;\n  -ms-thing: foo;\n}");
}
#[test]
fn test_selectors_less_format_1_8c1e8782() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* custom properties set & @apply rule */\n:root {\n    /* comments 192 */ --centered /* comments 193 */ : /* comments 194 */ {\n        display: flex;\n        align-items: center;\n        justify-content: center;\n    };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* custom properties set & @apply rule */\n:root {\n  /* comments 192 */\n  --centered/* comments 193 */ : /* comments 194 */ {\n    display: flex;\n    align-items: center;\n    justify-content: center;\n  };\n}");
}
#[test]
fn test_trailing_star_slash_less_format_1_67c36f8a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("@media (max-width: 1) {}\na {\n  // element.style */\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@media (max-width: 1) {\n}\na {\n  // element.style */\n}"
    );
}
#[test]
fn test_value_lists_less_format_1_e885fbd4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@test-space-separated: #aaaaaa // Start with A\n  #bbbbbb // then some B\n  #cccccc; // and round it out with C\n\n@test-space-separated : #aaaaaa // Start with A\n  #bbbbbb // then some B\n  #cccccc; // and round it out with C\n\n@test-comma-separated: #aaaaaa, // Start with A\n  #bbbbbb, // then some B\n  #cccccc; // and round it out with C") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@test-space-separated: #aaaaaa // Start with A\n  #bbbbbb // then some B\n  #cccccc; // and round it out with C\n\n@test-space-separated: #aaaaaa // Start with A\n  #bbbbbb // then some B\n  #cccccc; // and round it out with C\n\n@test-comma-separated:\n  #aaaaaa,\n  // Start with A\n  #bbbbbb,\n  // then some B\n  #cccccc; // and round it out with C");
}
