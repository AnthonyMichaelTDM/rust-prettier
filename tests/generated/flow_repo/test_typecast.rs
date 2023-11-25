#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_typecast_js_format_1_4d156e99() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n */\nvar tests = [\n  () => {\n    // erroneous typcasts raise errors...\n    var n = (\"hey\" : number);\n    // ...but 'any' does dynamic downcasts, if you must\n    var x: number = (\"hey\": any);\n    var y = ((\"hey\": any): number);\n  },\n\n  () => {\n    // typecasts in sequences\n    // (parens always required around typecasts)\n    var s: string = ((0: number), (\"hey\": string));\n  },\n\n  () => {\n    // TODO pending array element inference issues\n    // control case:\n    // var a : Array<?number> = [0, 1, 2, 3, 4, 5, 6, 7, null];\n    // typecast case:\n    // var b = [(0 : ?number), 1, 2, 3, 4, 5, 6, 7, null];\n  }\n];") ? ;
    assert_eq ! (formatted , "/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n */\nvar tests = [\n  () => {\n    // erroneous typcasts raise errors...\n    var n = (\"hey\": number);\n    // ...but 'any' does dynamic downcasts, if you must\n    var x: number = (\"hey\": any);\n    var y = ((\"hey\": any): number);\n  },\n\n  () => {\n    // typecasts in sequences\n    // (parens always required around typecasts)\n    var s: string = ((0: number), (\"hey\": string));\n  },\n\n  () => {\n    // TODO pending array element inference issues\n    // control case:\n    // var a : Array<?number> = [0, 1, 2, 3, 4, 5, 6, 7, null];\n    // typecast case:\n    // var b = [(0 : ?number), 1, 2, 3, 4, 5, 6, 7, null];\n  },\n];");
    Ok(())
}
