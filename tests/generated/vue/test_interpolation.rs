#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_bitwise_or_operator_vue_format_1_248278a7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n    {{\n      fn(\n        bitwise | or | operator | a_long_long_long_long_long_long_long_long_long_long_variable\n      )\n      | filter1\n      | filter2\n      | filter3\n      | filter4\n    }}\n  </div>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div>\n    {{\n      fn(\n        bitwise |\n          or |\n          operator |\n          a_long_long_long_long_long_long_long_long_long_long_variable,\n      )\n        | filter1\n        | filter2\n        | filter3\n        | filter4\n    }}\n  </div>\n</template>");
}
#[test]
fn test_parenthesized_vue_format_1_c9fc6a16() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template>\n<span>{{(a||          b)}} {{z&&(a&&b)}}</span>\n</template>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template>\n  <span>{{ a || b }} {{ z && a && b }}</span>\n</template>"
    );
}
#[test]
fn test_template_vue_format_1_427e4f67() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <root>\n<div class=\"red\">\n    We are going to add this simple card here\n  </div>\n  <div class=\"red\">\n    What is going on {{ prop1 }} and {{ prop2 }}\n  </div>\n  </root>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <root>\n    <div class=\"red\">We are going to add this simple card here</div>\n    <div class=\"red\">What is going on {{ prop1 }} and {{ prop2 }}</div>\n  </root>\n</template>");
}
