#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attribute_js_format_1_f6d91c7d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div {...a}/>;\n\n<div {...(a || {})} />;\n\n<div {...(cond ? foo : bar)} />;\n\n<div {...a /* comment */}/>;\n\n<div {/* comment */...a}/>;\n\n<div {...a //comment\n}/>;\n\n<div {...a\n  //comment\n}/>;\n\n<div {\n  //comment\n  ...a\n}/>;\n\n<div {//comment\n  ...a // comment\n}/>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div {...a} />;\n\n<div {...(a || {})} />;\n\n<div {...(cond ? foo : bar)} />;\n\n<div {...a /* comment */} />;\n\n<div {/* comment */ ...a} />;\n\n<div\n  {\n    ...a //comment\n  }\n/>;\n\n<div\n  {\n    ...a\n    //comment\n  }\n/>;\n\n<div\n  {\n    //comment\n    ...a\n  }\n/>;\n\n<div\n  {\n    //comment\n    ...a // comment\n  }\n/>;");
}
#[test]
fn test_child_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_child_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_child_js_format_1_edc912b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>{...a}</div>;\n\n<div>{...a /* comment */}</div>;\n\n<div>{/* comment */...a}</div>;\n\n<div>{...a //comment\n}</div>;\n\n<div>{...a\n  //comment\n}</div>;\n\n<div>{\n  //comment\n  ...a\n}</div>;\n\n<div>{//comment\n  ...a // comment\n}</div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div>{...a}</div>;\n\n<div>{...a /* comment */}</div>;\n\n<div>{/* comment */ ...a}</div>;\n\n<div>\n  {\n    ...a //comment\n  }\n</div>;\n\n<div>\n  {\n    ...a\n    //comment\n  }\n</div>;\n\n<div>\n  {\n    //comment\n    ...a\n  }\n</div>;\n\n<div>\n  {\n    //comment\n    ...a // comment\n  }\n</div>;");
}
