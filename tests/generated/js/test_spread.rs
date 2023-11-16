#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_spread_js_format_1_a91d227d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const foo = { ...(a || b) };\nconst foo2 = { ...a || b };\nconst foo3 = { ...(a ? b : c) };\n\nasync () => ({ ...(await foo) });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = { ...(a || b) };\nconst foo2 = { ...(a || b) };\nconst foo3 = { ...(a ? b : c) };\n\nasync () => ({ ...(await foo) });");
}
