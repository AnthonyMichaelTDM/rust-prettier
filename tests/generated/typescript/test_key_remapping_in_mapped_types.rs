#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_key_remapping_ts_format_1_fc9808f4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("type MappedTypeWithNewKeys<T> = {\n  [K in keyof T as NewKeyType]: T[K]\n};\n  \ntype RemoveKindField<T> = {\n  [K in keyof T as Exclude<K, \"kind\">]: T[K]\n};\n  \ntype PickByValueType<T, U> = {\n  [K in keyof T as T[K] extends U ? K : never]: T[K]\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type MappedTypeWithNewKeys<T> = {\n  [K in keyof T as NewKeyType]: T[K];\n};\n\ntype RemoveKindField<T> = {\n  [K in keyof T as Exclude<K, \"kind\">]: T[K];\n};\n\ntype PickByValueType<T, U> = {\n  [K in keyof T as T[K] extends U ? K : never]: T[K];\n};");
}
