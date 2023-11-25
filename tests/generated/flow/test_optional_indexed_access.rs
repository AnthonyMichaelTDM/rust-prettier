use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_indexed_access_js_trailing_commaall_format_1_ed546bcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type A = Obj?.['foo'];\ntype B = Obj?.['foo']['bar'];\ntype C = Obj['foo']?.['bar'];\ntype D = (Obj?.['foo'])['bar'];\ntype E = (T & S)?.['bar'];\ntype F = (T | S)?.['bar'];\ntype G = (?T)?.['bar'];\ntype H = (typeof x)?.['bar'];\ntype I = (string => void)?.['bar'];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type A = Obj?.[\"foo\"];\ntype B = Obj?.[\"foo\"][\"bar\"];\ntype C = Obj[\"foo\"]?.[\"bar\"];\ntype D = (Obj?.[\"foo\"])[\"bar\"];\ntype E = (T & S)?.[\"bar\"];\ntype F = (T | S)?.[\"bar\"];\ntype G = (?T)?.[\"bar\"];\ntype H = (typeof x)?.[\"bar\"];\ntype I = ((string) => void)?.[\"bar\"];");
}
