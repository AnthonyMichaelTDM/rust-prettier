#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_call_js_format_1_56f6fa2b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (true)\n  if (true)\n    if (true)\n      if (true)\n        if (true)\n          longVariableName1 = // @ts-ignore\n          (variable01 + veryLongVariableNameNumber2).method();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (true)\n  if (true)\n    if (true)\n      if (true)\n        if (true)\n          longVariableName1 = // @ts-ignore\n            (variable01 + veryLongVariableNameNumber2).method();");
}
#[test]
fn test_call_2_js_format_1_636e2f14() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const kochabCooieGameOnOboleUnweave = // ???\n      rhubarbRhubarb(annularCooeedSplicesWalksWayWay);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const kochabCooieGameOnOboleUnweave = // ???\n  rhubarbRhubarb(annularCooeedSplicesWalksWayWay);");
}
#[test]
fn test_function_js_format_1_34384db3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("f1 = (\n  a =\n  //comment\n  b\n) => {};\n  \nf2 = (\n  a = //comment\n  b\n) => {};\n  \nf3 = (\n  a =\n  b //comment\n) => {};\n  \nf4 = // Comment\n  () => {};\n\nf5 =\n  \n  // Comment\n  \n () => {}\n  \nf6 = /* comment */\n  \n  // Comment\n  \n  () => {}\n  \nlet f1 = (\n  a =\n  //comment\n  b\n) => {};\n  \nlet f2 = (\n  a = //comment\n  b\n) => {};\n  \nlet f3 = (\n  a =\n  b //comment\n) => {};\n  \nlet f4 = // Comment\n  () => {};\n  \nlet f5 =\n  \n  // Comment\n  \n  () => {}\n  \nlet f6 = /* comment */\n  \n  // Comment\n  \n  () => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "f1 = (\n  //comment\n  a = b,\n) => {};\n\nf2 = (\n  a = b, //comment\n) => {};\n\nf3 = (\n  a = b, //comment\n) => {};\n\nf4 = // Comment\n  () => {};\n\nf5 =\n  // Comment\n\n  () => {};\n\nf6 =\n  /* comment */\n\n  // Comment\n\n  () => {};\n\nlet f1 = (\n  //comment\n  a = b,\n) => {};\n\nlet f2 = (\n  a = b, //comment\n) => {};\n\nlet f3 = (\n  a = b, //comment\n) => {};\n\nlet f4 = // Comment\n  () => {};\n\nlet f5 =\n  // Comment\n\n  () => {};\n\nlet f6 =\n  /* comment */\n\n  // Comment\n\n  () => {};");
}
#[test]
fn test_identifier_js_format_1_6880df30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const kochabCooieGameOnOboleUnweave = // ???\n      annularCooeedSplicesWalksWayWay;\n\nconst bifornCringerMoshedPerplexSawder = // !!!\n  glimseGlyphsHazardNoopsTieTie +\n  averredBathersBoxroomBuggyNurl -\n  anodyneCondosMalateOverateRetinol;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const kochabCooieGameOnOboleUnweave = // ???\n  annularCooeedSplicesWalksWayWay;\n\nconst bifornCringerMoshedPerplexSawder = // !!!\n  glimseGlyphsHazardNoopsTieTie +\n  averredBathersBoxroomBuggyNurl -\n  anodyneCondosMalateOverateRetinol;");
}
#[test]
fn test_number_js_format_1_b2f227a1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fnNumber =\n  // Comment\n  3;\n\nfnNumber =\n\n  // Comment\n\n  3;\n\nfnNumber =\n  // Comment0\n  // Comment1\n  3;\n\nfnNumber = /* comment */\n  3;\n\nfnNumber = /* comments0 */\n  /* comments1 */\n  3;\n\nfnNumber =\n  // Comment\n  3;\n\nvar fnNumber =\n\n  // Comment\n\n  3;\n\nvar fnNumber =\n  // Comment0\n  // Comment1\n  3;\n\nvar fnNumber = /* comment */\n  3;\n\nvar fnNumber = /* comments0 */\n  /* comments1 */\n  3;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "fnNumber =\n  // Comment\n  3;\n\nfnNumber =\n  // Comment\n\n  3;\n\nfnNumber =\n  // Comment0\n  // Comment1\n  3;\n\nfnNumber = /* comment */ 3;\n\nfnNumber =\n  /* comments0 */\n  /* comments1 */\n  3;\n\nfnNumber =\n  // Comment\n  3;\n\nvar fnNumber =\n  // Comment\n\n  3;\n\nvar fnNumber =\n  // Comment0\n  // Comment1\n  3;\n\nvar fnNumber = /* comment */ 3;\n\nvar fnNumber =\n  /* comments0 */\n  /* comments1 */\n  3;");
}
#[test]
fn test_string_js_format_1_6a60884d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("fnString =\n  // Comment\n  'some' + 'long' + 'string';\n\nfnString =\n  // Comment\n\n  'some' + 'long' + 'string';\n\nfnString =\n\n  // Comment\n\n  'some' + 'long' + 'string';\n\nfnString =\n  /* comment */\n  'some' + 'long' + 'string';\n\nfnString =\n  /**\n   * multi-line\n   */\n  'some' + 'long' + 'string';\n\nfnString =\n  /* inline */ 'some' + 'long' + 'string' + 'some' + 'long' + 'string' + 'some' + 'long' + 'string' + 'some' + 'long' + 'string';\n\nfnString = // Comment0\n  // Comment1\n  'some' + 'long' + 'string';\n\nfnString = // Comment\n  'some' + 'long' + 'string';\n\nfnString =\n  // Comment\n  'some' + 'long' + 'string';\n\nvar fnString =\n  // Comment\n\n  'some' + 'long' + 'string';\n\nvar fnString =\n\n  // Comment\n\n  'some' + 'long' + 'string';\n\nvar fnString =\n  /* comment */\n  'some' + 'long' + 'string';\n\nvar fnString =\n  /**\n   * multi-line\n   */\n  'some' + 'long' + 'string';\n\nvar fnString =\n  /* inline */ 'some' + 'long' + 'string' + 'some' + 'long' + 'string' + 'some' + 'long' + 'string' + 'some' + 'long' + 'string';\n\nvar fnString = // Comment0\n  // Comment1\n  'some' + 'long' + 'string';\n\nvar fnString = // Comment\n  'some' + 'long' + 'string';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "fnString =\n  // Comment\n  \"some\" + \"long\" + \"string\";\n\nfnString =\n  // Comment\n\n  \"some\" + \"long\" + \"string\";\n\nfnString =\n  // Comment\n\n  \"some\" + \"long\" + \"string\";\n\nfnString =\n  /* comment */\n  \"some\" + \"long\" + \"string\";\n\nfnString =\n  /**\n   * multi-line\n   */\n  \"some\" + \"long\" + \"string\";\n\nfnString =\n  /* inline */ \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\";\n\nfnString = // Comment0\n  // Comment1\n  \"some\" + \"long\" + \"string\";\n\nfnString = \"some\" + \"long\" + \"string\"; // Comment\n\nfnString =\n  // Comment\n  \"some\" + \"long\" + \"string\";\n\nvar fnString =\n  // Comment\n\n  \"some\" + \"long\" + \"string\";\n\nvar fnString =\n  // Comment\n\n  \"some\" + \"long\" + \"string\";\n\nvar fnString =\n  /* comment */\n  \"some\" + \"long\" + \"string\";\n\nvar fnString =\n  /**\n   * multi-line\n   */\n  \"some\" + \"long\" + \"string\";\n\nvar fnString =\n  /* inline */ \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\" +\n  \"some\" +\n  \"long\" +\n  \"string\";\n\nvar fnString = // Comment0\n  // Comment1\n  \"some\" + \"long\" + \"string\";\n\nvar fnString = \"some\" + \"long\" + \"string\"; // Comment");
}
