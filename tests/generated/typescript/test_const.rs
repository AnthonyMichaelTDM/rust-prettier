#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_initializer_ambient_context_ts_format_1_92df2442() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("declare module N {\n  enum E {\n    ok = 0\n  }\n\n  export const string = \"2\";\n  export const number = 1.;\n  export const bigint = 0n;\n  export const negative_bigint = -0n;\n  export const negative_number = -1;\n  export const template = \\`-2\\`;\n  export const False = false;\n  export const True = true;\n  export const E_ok = E.ok;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare module N {\n  enum E {\n    ok = 0,\n  }\n\n  export const string = \"2\";\n  export const number = 1;\n  export const bigint = 0n;\n  export const negative_bigint = -0n;\n  export const negative_number = -1;\n  export const template = \\`-2\\`;\n  export const False = false;\n  export const True = true;\n  export const E_ok = E.ok;\n}");
}
