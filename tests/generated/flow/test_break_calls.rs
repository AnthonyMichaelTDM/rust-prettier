use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_args_js_format_1_7c93dd95() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const response = something.$http.get<ThingamabobService.DetailsData>(\n  \\`api/foo.ashx/foo-details/\\${myId}\\`,\n  { cache: quux.httpCache, timeout }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const response = something.$http.get<ThingamabobService.DetailsData>(\n  \\`api/foo.ashx/foo-details/\\${myId}\\`,\n  { cache: quux.httpCache, timeout },\n);");
}
