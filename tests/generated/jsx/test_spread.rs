#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attribute_js_format_1_f6d91c7d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div {...a}/>;\n\n<div {...(a || {})} />;\n\n<div {...(cond ? foo : bar)} />;\n\n<div {...a /* comment */}/>;\n\n<div {/* comment */...a}/>;\n\n<div {...a //comment\n}/>;\n\n<div {...a\n  //comment\n}/>;\n\n<div {\n  //comment\n  ...a\n}/>;\n\n<div {//comment\n  ...a // comment\n}/>;") ? ;
    assert_eq ! (formatted , "<div {...a} />;\n\n<div {...(a || {})} />;\n\n<div {...(cond ? foo : bar)} />;\n\n<div {...a /* comment */} />;\n\n<div {/* comment */ ...a} />;\n\n<div\n  {\n    ...a //comment\n  }\n/>;\n\n<div\n  {\n    ...a\n    //comment\n  }\n/>;\n\n<div\n  {\n    //comment\n    ...a\n  }\n/>;\n\n<div\n  {\n    //comment\n    ...a // comment\n  }\n/>;");
    Ok(())
}
#[test]
fn test_child_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_child_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_child_js_format_1_edc912b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>{...a}</div>;\n\n<div>{...a /* comment */}</div>;\n\n<div>{/* comment */...a}</div>;\n\n<div>{...a //comment\n}</div>;\n\n<div>{...a\n  //comment\n}</div>;\n\n<div>{\n  //comment\n  ...a\n}</div>;\n\n<div>{//comment\n  ...a // comment\n}</div>;") ? ;
    assert_eq ! (formatted , "<div>{...a}</div>;\n\n<div>{...a /* comment */}</div>;\n\n<div>{/* comment */ ...a}</div>;\n\n<div>\n  {\n    ...a //comment\n  }\n</div>;\n\n<div>\n  {\n    ...a\n    //comment\n  }\n</div>;\n\n<div>\n  {\n    //comment\n    ...a\n  }\n</div>;\n\n<div>\n  {\n    //comment\n    ...a // comment\n  }\n</div>;");
    Ok(())
}
