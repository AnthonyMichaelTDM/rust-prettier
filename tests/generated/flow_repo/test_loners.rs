use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_loners_js_format_1_f0feb7f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o = { x: 5, y: \"jello\" };\nvar z = o.z;\nvar export_o: { x: number; } = o;\n\nfunction f(u,v?):number { return u; }\nvar export_f: (u: number) => number = f;\n\n//exports = export_o;\nmodule.exports = export_f;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var o = { x: 5, y: \"jello\" };\nvar z = o.z;\nvar export_o: { x: number } = o;\n\nfunction f(u, v?): number {\n  return u;\n}\nvar export_f: (u: number) => number = f;\n\n//exports = export_o;\nmodule.exports = export_f;");
}
