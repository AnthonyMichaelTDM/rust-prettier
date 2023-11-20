#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_crlf_scss_format_1_85c8b3b5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$nice-blue: #5B83AD;\n$light-blue: $nice-blue + #111;\n\n// Comment 1\n/*\n * Comment 2\n */\n#header {\n    // Comment 3\n    /*\n     * Comment 4\n     */\n    color: @light-blue;\n}\n\n@media only screen and (max-width: 600px) {\n    // Comment 1\n    /*\n     * Comment 5\n     */\n    body {\n        background-color: lightblue; // Comment 6\n        color: red; /* Comment 7 */\n    }\n}\n\n@mixin create-rules($padding) {\n    //\n    // Comment 8\n    //\n    .abc {\n        padding: $padding;\n    }\n    /**\n     * Comment 9\n     */\n    .def {\n        padding: $padding;\n    }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$nice-blue: #5b83ad;\n$light-blue: $nice-blue + #111;\n\n// Comment 1\n/*\n * Comment 2\n */\n#header {\n  // Comment 3\n  /*\n     * Comment 4\n     */\n  color: @light-blue;\n}\n\n@media only screen and (max-width: 600px) {\n  // Comment 1\n  /*\n     * Comment 5\n     */\n  body {\n    background-color: lightblue; // Comment 6\n    color: red; /* Comment 7 */\n  }\n}\n\n@mixin create-rules($padding) {\n  //\n  // Comment 8\n  //\n  .abc {\n    padding: $padding;\n  }\n  /**\n     * Comment 9\n     */\n  .def {\n    padding: $padding;\n  }\n}");
}
#[test]
fn test_at_rule_scss_format_1_574c4f4b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("@at-root .foo\n// .bar\n{\n\n}\n\n@at-root\n// .bar\n.foo\n{\n\n");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@at-root .foo\n// .bar\n{\n}\n\n@at-root // .bar\n.foo {\n}"
    );
}
#[test]
fn test_between_decl_scss_format_1_c0c24f69() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("selector {\nprop: // comment\nvalue;\n\nprop: /* block */ value;\n\nprop\n: value;\n}\n\n// #5603\n.grid {\n        grid-template-areas: //\n          \"header header header\" //\n          \"sidebar content content\" //\n          \"footer footer footer\";\n\n        grid-template-areas:\n          \"header header header\" //\n          \"sidebar content content\" //\n          \"footer footer footer\";\n }\n\n// #8052\n$font-family-rich:\n  // custom\n  'Noto Sans TC', 'Noto Sans SC', 'Noto Sans JP',\n  // Safari for OS X and iOS (San Francisco)\n  -apple-system, BlinkMacSystemFont,\n  // fallback\n  Roboto, 'Helvetica Neue', Helvetica, Arial, sans-serif,\n  // emoji\n  'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol' !default;\n\n// #7109\n.test {\n    background:\n        /////// foo\n        // bar\n\n        radial-gradient(circle farthest-corner at 5% 10%, #000000, transparent 50%);\n}\n\n// TODO: make these pretty\nselector {\nprop:\n/* block */\nvalue;\n\nprop\n// inline\n:\nvalue;\n\nprop/* block */:\nvalue;\n\nprop\n/* block */\n:\nvalue;\n\nprop/* before */: // after\nvalue;\n\n\nprop/* before */: /* after*/\nvalue;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "selector {\n  prop: // comment\n    value;\n\n  prop: /* block */ value;\n\n  prop: value;\n}\n\n// #5603\n.grid {\n  grid-template-areas: //\n    \"header header header\" //\n    \"sidebar content content\" //\n    \"footer footer footer\";\n\n  grid-template-areas: \"header header header\" //\n    \"sidebar content content\" //\n    \"footer footer footer\";\n}\n\n// #8052\n$font-family-rich:\n  // custom\n  \"Noto Sans TC\",\n  \"Noto Sans SC\",\n  \"Noto Sans JP\",\n  // Safari for OS X and iOS (San Francisco)\n  -apple-system,\n  BlinkMacSystemFont,\n  // fallback\n  Roboto,\n  \"Helvetica Neue\",\n  Helvetica,\n  Arial,\n  sans-serif,\n  // emoji\n  \"Apple Color Emoji\",\n  \"Segoe UI Emoji\",\n  \"Segoe UI Symbol\" !default;\n\n// #7109\n.test {\n  background:\n        /////// foo\n        // bar\n    radial-gradient(circle farthest-corner at 5% 10%, #000000, transparent 50%);\n}\n\n// TODO: make these pretty\nselector {\n  prop:\n/* block */ value;\n\n  prop // inline\n: value;\n\n  prop/* block */: value;\n\n  prop/* block */\n: value;\n\n  prop/* before */: // after\n    value;\n\n  prop/* before */: /* after*/ value;\n}");
}
#[test]
fn test_bug_scss_format_1_6093d627() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@font-face  {\n  src: url(if(\n    $bootstrap-sass-asset-helper,\n    twbs-font-path('#{$icon-font-path}#{$icon-font-name}.eot'),\n    '#{$icon-font-path}#{$icon-font-name}.eot'\n  ));\n}\n\n// Catchall baseclass\n/* Catchall baseclass */\n.glyphicon {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@font-face {\n  src: url(if(\n    $bootstrap-sass-asset-helper,\n    twbs-font-path(\"#{$icon-font-path}#{$icon-font-name}.eot\"),\n    \"#{$icon-font-path}#{$icon-font-name}.eot\"\n  ));\n}\n\n// Catchall baseclass\n/* Catchall baseclass */\n.glyphicon {\n}");
}
#[test]
fn test_custom_properties_scss_format_1_8c1e8782() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* custom properties set & @apply rule */\n:root {\n    /* comments 192 */ --centered /* comments 193 */ : /* comments 194 */ {\n        display: flex;\n        align-items: center;\n        justify-content: center;\n    };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* custom properties set & @apply rule */\n:root {\n  /* comments 192 */\n  --centered/* comments 193 */ : /* comments 194 */ {\n    display: flex;\n    align-items: center;\n    justify-content: center;\n  };\n}");
}
#[test]
fn test_if_eslit_at_rule_decloration_scss_format_1_28304628() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@function _normalize-include($section) {\n// Check if $section is in the $include list.\n  @if index($_normalize-include, $section) {\n    @return true;\n  }\n// If $include is set to (all), make sure $section is not in $exclude.\n  @else if not index($_normalize-exclude, $section) and index($_normalize-include, all) {\n    @return true;\n  }\n  @return false;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@function _normalize-include($section) {\n  // Check if $section is in the $include list.\n  @if index($_normalize-include, $section) {\n    @return true;\n  }\n  // If $include is set to (all), make sure $section is not in $exclude.\n  @else if not\n    index($_normalize-exclude, $section) and\n    index($_normalize-include, all)\n  {\n    @return true;\n  }\n  @return false;\n}");
}
#[test]
fn test_in_value_scss_format_1_9139fb64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".real-world-example {\n  background: radial-gradient(\n      circle at left 0% bottom $position,\n      transparent,\n      transparent $diameter,\n      #fbfbfb calc(#{$diameter} + 1px) // Add 1px for edge-smoothing.\n    );\n}\n\n.simplification { \n  foo: (\n    calc() // not a comment anymore\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".real-world-example {\n  background: radial-gradient(\n    circle at left 0% bottom $position,\n    transparent,\n    transparent $diameter,\n    #fbfbfb calc(#{$diameter} + 1px) // Add 1px for edge-smoothing.\n  );\n}\n\n.simplification {\n  foo: (\n    calc() // not a comment anymore\n  );\n}");
}
#[test]
fn test_lists_scss_format_1_2d21ed44() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("$my-list:\n  'foo', // Foo\n  'bar'; // Bar\n\n$my-list2:\n  a // a\n  b\n  c;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "$my-list:\n  \"foo\",\n  // Foo\n  \"bar\"; // Bar\n\n$my-list2: a // a\n  b c;"
    );
}
#[test]
fn test_maps_scss_format_1_c043009c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("$my-map: (\n  'foo': 1, // Foo\n  'bar': 2, // Bar\n);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "$my-map: (\n  \"foo\": 1,\n  // Foo\n  \"bar\": 2,\n  // Bar\n);"
    );
}
#[test]
fn test_mixed_scss_format_1_a2bbf8f2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import 'b';\n// another comment\n//no-space before") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import \"b\";\n// another comment\n//no-space before");
}
#[test]
fn test_mixed_2_scss_format_1_9d388255() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n@import \"a\";\n//*\n@import 'b';\n/* block */\n/*no-space block*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n@import \"a\";\n//*\n@import \"b\";\n/* block */\n/*no-space block*/");
}
#[test]
fn test_mixed_block_scss_format_1_eb71963e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import 'b';\n/* block */\n/*no-space block*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* DO NOT ADD EXTRA CODE TO THIS FILE */\n\n@import \"a\";\n\n// '/*' <-- this breaks formatting\n\n@import \"b\";\n/* block */\n/*no-space block*/");
}
#[test]
fn test_prettier_ignore_scss_format_1_ef5e979b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\nfoo {\n  /* prettier-ignore */\n  thing:     foo;\n  -ms-thing: foo;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "foo {\n  /* prettier-ignore */\n  thing:     foo;\n  -ms-thing: foo;\n}"
    );
}
#[test]
fn test_selectors_scss_format_1_1b4452e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".powerPathNavigator .helm button.pressedButton, // comment 1\n.powerPathNavigator .helm button:active:not(.disabledButton),\n.powerPathNavigator table.powerPathInfo th:active,\n.powerPathNavigator table.powerPathInfo th:active + th:last-child {\n}\n\n// comment 2\n.powerPathNavigator .helm button.pressedButton,\n.powerPathNavigator .helm button:active:not(.disabledButton) {\n}\n\n.foo,\n// comment 3\n.bar {\n  display: block;\n}\n\n.field\n  {\n  &[data-field-id=\"1\"], // comment 4\n  &[data-field-id=\"2\"], // comment 5\n  &[data-field-id=\"3\"], // comment 6\n  {\n    background: green;\n  }\n}\n\n// comment 7\n.field\n// comment 8\n{ // comment 9\n  // comment 10\n  &[data-field-id=\"1\"], // comment 11\n  // comment 12\n  &[data-field-id=\"2\"] // comment 13\n  // comment 14\n  , // comment 15\n  // comment 16\n  &[data-field-id=\"3\"], // comment 17\n  // comment 18\n  { // comment 19\n    // comment 20\n    background: green;\n    // comment 21\n  } // comment 22\n  // comment 23\n}\n// comment 24\n\n.foo\n// comment 25\n.bar\n// comment 26\n{}\n\n.foo\n// comment 27\n+\n// comment 28\n.bar\n// comment 29\n{}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".powerPathNavigator .helm button.pressedButton, // comment 1\n.powerPathNavigator .helm button:active:not(.disabledButton),\n.powerPathNavigator table.powerPathInfo th:active,\n.powerPathNavigator table.powerPathInfo th:active + th:last-child {\n}\n\n// comment 2\n.powerPathNavigator .helm button.pressedButton,\n.powerPathNavigator .helm button:active:not(.disabledButton) {\n}\n\n.foo,\n// comment 3\n.bar {\n  display: block;\n}\n\n.field {\n  &[data-field-id=\"1\"], // comment 4\n  &[data-field-id=\"2\"], // comment 5\n  &[data-field-id=\"3\"], // comment 6\n  {\n    background: green;\n  }\n}\n\n// comment 7\n.field\n// comment 8\n{\n  // comment 9\n  // comment 10\n  &[data-field-id=\"1\"], // comment 11\n  // comment 12\n  &[data-field-id=\"2\"] // comment 13\n  // comment 14\n  , // comment 15\n  // comment 16\n  &[data-field-id=\"3\"], // comment 17\n  // comment 18\n  {\n    // comment 19\n    // comment 20\n    background: green;\n    // comment 21\n  } // comment 22\n  // comment 23\n}\n// comment 24\n\n.foo\n// comment 25\n.bar\n// comment 26\n{\n}\n\n.foo\n// comment 27\n+\n// comment 28\n.bar\n// comment 29\n{\n}");
}
#[test]
fn test_trailing_star_slash_scss_format_1_67c36f8a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
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
fn test_variable_declaration_scss_format_1_8d9748d2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$var: /* comment 1 */ all /* comment 2 */ !default /* comment 3 */ ; /* comment 4 */\n\n@mixin text-color {\n  /* comment 5 */\n  /* comment 6 */ $text-color /* comment 7 */ : /* comment 8 */ red /* comment 9 */ !default /* comment 10 */ ; /* comment 11 */\n  /* comment 12 */\n  color: $text-color;\n}\n\n.error {\n  /* comment 13 */\n  /* comment 14 */ $text-color /* comment 15 */ : /* comment 16 */ green /* comment 17 */ !global /* comment 18 */ ; /* comment 19 */\n  /* comment 20 */\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$var: /* comment 1 */ all /* comment 2 */ !default /* comment 3 */; /* comment 4 */\n\n@mixin text-color {\n  /* comment 5 */\n  /* comment 6 */\n  $text-color/* comment 7 */ : /* comment 8 */ red /* comment 9 */ !default /* comment 10 */; /* comment 11 */\n  /* comment 12 */\n  color: $text-color;\n}\n\n.error {\n  /* comment 13 */\n  /* comment 14 */\n  $text-color/* comment 15 */ : /* comment 16 */ green /* comment 17 */ !global /* comment 18 */; /* comment 19 */\n  /* comment 20 */\n}");
}
