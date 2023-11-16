#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_basic_ts_format_1_4473b56c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type X3<T> = T extends [infer U extends number] ? MustBeNumber<U> : never;\ntype X4<T> = T extends [infer U extends number, infer U extends number] ? MustBeNumber<U> : never;\ntype X5<T> = T extends [infer U extends number, infer U] ? MustBeNumber<U> : never;\ntype X6<T> = T extends [infer U, infer U extends number] ? MustBeNumber<U> : never;\ntype X7<T> = T extends [infer U extends string, infer U extends number] ? U : never;\ntype X8<U, T> = T extends infer U extends number ? U : T;\ntype X9<U, T> = T extends (infer U extends number ? U : T) ? U : T;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type X3<T> = T extends [infer U extends number] ? MustBeNumber<U> : never;\ntype X4<T> = T extends [infer U extends number, infer U extends number]\n  ? MustBeNumber<U>\n  : never;\ntype X5<T> = T extends [infer U extends number, infer U]\n  ? MustBeNumber<U>\n  : never;\ntype X6<T> = T extends [infer U, infer U extends number]\n  ? MustBeNumber<U>\n  : never;\ntype X7<T> = T extends [infer U extends string, infer U extends number]\n  ? U\n  : never;\ntype X8<U, T> = T extends infer U extends number ? U : T;\ntype X9<U, T> = T extends (infer U extends number ? U : T) ? U : T;");
}
