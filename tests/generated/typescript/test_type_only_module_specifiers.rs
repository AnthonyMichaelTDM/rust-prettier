#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_ts_format_1_52c5dac9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { type A } from \"foo\";\nimport { type B, C } from \"foo\";\nimport { type D, type E } from \"foo\";\nimport { type E as F } from \"foo\";\n\nexport { type A } from \"foo\";\nexport { type B, C } from \"foo\";\nexport { type D, type E } from \"foo\";\nexport { type E as F } from \"foo\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { type A } from \"foo\";\nimport { type B, C } from \"foo\";\nimport { type D, type E } from \"foo\";\nimport { type E as F } from \"foo\";\n\nexport { type A } from \"foo\";\nexport { type B, C } from \"foo\";\nexport { type D, type E } from \"foo\";\nexport { type E as F } from \"foo\";");
}
