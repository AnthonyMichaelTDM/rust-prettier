#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_multiple_js_format_1_bb9abe69() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var assert = require('assert'),\n  lookup = require('../lookup');\n\nconst eloBar     = require(\"elo-bar\")\n  , foo        = require(\"foo\")\n  , otherThing = require(\"other-thing\");\n\nvar a, b, c;\n\nlet superSuperSuperLong1, superSuperSuperLong2, superSuperSuperLong3, superSuperSuperLong4;\n\nfor (var i = 0, len = arr.length; i < len; i++) {}\n\nvar templateTagsMapping = {\n    '%{itemIndex}': 'index',\n    '%{itemContentMetaTextViews}': 'views'\n  },\n  separator = '<span class=\"item__content__meta__separator\">•</span>',\n  templateTagsList = $.map(templateTagsMapping, function(value, key) {\n    return key;\n  }),\n  data;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var assert = require(\"assert\"),\n  lookup = require(\"../lookup\");\n\nconst eloBar = require(\"elo-bar\"),\n  foo = require(\"foo\"),\n  otherThing = require(\"other-thing\");\n\nvar a, b, c;\n\nlet superSuperSuperLong1,\n  superSuperSuperLong2,\n  superSuperSuperLong3,\n  superSuperSuperLong4;\n\nfor (var i = 0, len = arr.length; i < len; i++) {}\n\nvar templateTagsMapping = {\n    \"%{itemIndex}\": \"index\",\n    \"%{itemContentMetaTextViews}\": \"views\",\n  },\n  separator = '<span class=\"item__content__meta__separator\">•</span>',\n  templateTagsList = $.map(templateTagsMapping, function (value, key) {\n    return key;\n  }),\n  data;");
}
#[test]
fn test_string_js_format_1_6312cd28() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("elements[0].innerHTML = '<div></div><div></div><div></div><div></div><div></div><div></div>';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "elements[0].innerHTML =\n  \"<div></div><div></div><div></div><div></div><div></div><div></div>\";");
}
