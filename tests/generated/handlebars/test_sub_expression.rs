use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_sub_expressions_hbs_single_quotetrue_format_1_6f920064() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div\n  {{mustache\n    (concat\n      (service)\n      (helper param hashPair=Value)\n      (largeNameHelper param param param param hashPair=value hashPair=value hashPair=Value)\n      hashPair=(helper param param param param param param hashPair=value hashPair=value hashPair=value)\n      hashPair=(does not need a line break due to being under 80 chars long)\n    )\n  }}\n></div>\n\n{{#block\n  (concat\n    (service)\n    (helper param hashPair=Value)\n    (largeNameHelper param param param param hashPair=value hashPair=value hashPair=Value)\n    hashPair=(helper param param param param param param hashPair=value hashPair=value hashPair=value)\n    hashPair=(does not need a line break due to being under 80 chars long)\n  )\n}}\n\n{{/block}}\n\n{{foobar-sub-component/foobar-foo \n  hook=\"stringLiteral\"\n  foo=\n    (t\n      (concat \"stringLiteral\" (get blockParam \"stringLiteral\") hash=hash hash=hash)\n      foo=(simple-helper (hash hashKey=blockParam.foo assignParam=blockParam.bar))\n    )\n}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  {{mustache\n    (concat\n      (service)\n      (helper param hashPair=Value)\n      (largeNameHelper\n        param param param param hashPair=value hashPair=value hashPair=Value\n      )\n      hashPair=(helper\n        param\n        param\n        param\n        param\n        param\n        param\n        hashPair=value\n        hashPair=value\n        hashPair=value\n      )\n      hashPair=(does not need a line break due to being under 80 chars long)\n    )\n  }}\n></div>\n\n{{#block\n  (concat\n    (service)\n    (helper param hashPair=Value)\n    (largeNameHelper\n      param param param param hashPair=value hashPair=value hashPair=Value\n    )\n    hashPair=(helper\n      param\n      param\n      param\n      param\n      param\n      param\n      hashPair=value\n      hashPair=value\n      hashPair=value\n    )\n    hashPair=(does not need a line break due to being under 80 chars long)\n  )\n}}{{/block}}\n\n{{foobar-sub-component/foobar-foo\n  hook='stringLiteral'\n  foo=(t\n    (concat\n      'stringLiteral' (get blockParam 'stringLiteral') hash=hash hash=hash\n    )\n    foo=(simple-helper (hash hashKey=blockParam.foo assignParam=blockParam.bar))\n  )\n}");
}
#[test]
fn test_sub_expressions_hbs_format_1_6f920064() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("hbs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div\n  {{mustache\n    (concat\n      (service)\n      (helper param hashPair=Value)\n      (largeNameHelper param param param param hashPair=value hashPair=value hashPair=Value)\n      hashPair=(helper param param param param param param hashPair=value hashPair=value hashPair=value)\n      hashPair=(does not need a line break due to being under 80 chars long)\n    )\n  }}\n></div>\n\n{{#block\n  (concat\n    (service)\n    (helper param hashPair=Value)\n    (largeNameHelper param param param param hashPair=value hashPair=value hashPair=Value)\n    hashPair=(helper param param param param param param hashPair=value hashPair=value hashPair=value)\n    hashPair=(does not need a line break due to being under 80 chars long)\n  )\n}}\n\n{{/block}}\n\n{{foobar-sub-component/foobar-foo \n  hook=\"stringLiteral\"\n  foo=\n    (t\n      (concat \"stringLiteral\" (get blockParam \"stringLiteral\") hash=hash hash=hash)\n      foo=(simple-helper (hash hashKey=blockParam.foo assignParam=blockParam.bar))\n    )\n}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  {{mustache\n    (concat\n      (service)\n      (helper param hashPair=Value)\n      (largeNameHelper\n        param param param param hashPair=value hashPair=value hashPair=Value\n      )\n      hashPair=(helper\n        param\n        param\n        param\n        param\n        param\n        param\n        hashPair=value\n        hashPair=value\n        hashPair=value\n      )\n      hashPair=(does not need a line break due to being under 80 chars long)\n    )\n  }}\n></div>\n\n{{#block\n  (concat\n    (service)\n    (helper param hashPair=Value)\n    (largeNameHelper\n      param param param param hashPair=value hashPair=value hashPair=Value\n    )\n    hashPair=(helper\n      param\n      param\n      param\n      param\n      param\n      param\n      hashPair=value\n      hashPair=value\n      hashPair=value\n    )\n    hashPair=(does not need a line break due to being under 80 chars long)\n  )\n}}{{/block}}\n\n{{foobar-sub-component/foobar-foo\n  hook=\"stringLiteral\"\n  foo=(t\n    (concat\n      \"stringLiteral\" (get blockParam \"stringLiteral\") hash=hash hash=hash\n    )\n    foo=(simple-helper (hash hashKey=blockParam.foo assignParam=blockParam.bar))\n  )\n}");
}
