use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_relational_operators_js_format_1_58d73351() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(<div />) < 5;\n<div /> > 5;\n(<div></div>) < 5;\n<div></div> > 5;\n\n<div /> <= 5;\n<div /> >= 5;\n<div></div> <= 5;\n<div></div> >= 5;\n\n(<div />) < <div />;\n<div /> > <div />;\n(<div></div>) < <div></div>;\n<div></div> > <div></div>;\n\n<div /> <= <div />;\n<div /> >= <div />;\n<div></div> <= <div></div>;\n<div></div> >= <div></div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(<div />) < 5;\n<div /> > 5;\n(<div></div>) < 5;\n<div></div> > 5;\n\n<div /> <= 5;\n<div /> >= 5;\n<div></div> <= 5;\n<div></div> >= 5;\n\n(<div />) < <div />;\n<div /> > <div />;\n(<div></div>) < <div></div>;\n<div></div> > <div></div>;\n\n<div /> <= <div />;\n<div /> >= <div />;\n<div></div> <= <div></div>;\n<div></div> >= <div></div>;");
}
