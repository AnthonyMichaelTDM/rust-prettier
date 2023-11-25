#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_32d32507() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var ColorId = {\n  RED: 'R',\n  GREEN: 'G',\n  BLUE: 'B',\n};\n\nvar ColorNumber = {\n  RED: 'ff0000',\n  GREEN: '00ff00',\n  BLUE: '0000ff',\n};\n\nvar ColorIdToNumber = {\n  [ColorId.RED]: ColorNumber.RED,\n  [ColorId.GREEN]: ColorNumber.GREEN,\n  [ColorId.BLUE]: ColorNumber.BLUE,\n};\n\n(ColorIdToNumber[ColorId.RED]: 'ffffff'); // oops\n\nColorIdToNumber.XXX; // oops\n\nmodule.exports = { ColorId, ColorNumber };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var ColorId = {\n  RED: \"R\",\n  GREEN: \"G\",\n  BLUE: \"B\",\n};\n\nvar ColorNumber = {\n  RED: \"ff0000\",\n  GREEN: \"00ff00\",\n  BLUE: \"0000ff\",\n};\n\nvar ColorIdToNumber = {\n  [ColorId.RED]: ColorNumber.RED,\n  [ColorId.GREEN]: ColorNumber.GREEN,\n  [ColorId.BLUE]: ColorNumber.BLUE,\n};\n\n(ColorIdToNumber[ColorId.RED]: \"ffffff\"); // oops\n\nColorIdToNumber.XXX; // oops\n\nmodule.exports = { ColorId, ColorNumber };");
}
#[test]
fn test_test_2_js_format_1_c1071b0c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var { ColorId, ColorNumber } = require('./test');\nvar ColorIdToNumber = {\n  [ColorId.RED]: ColorNumber.RED,\n  [ColorId.GREEN]: ColorNumber.GREEN,\n  [ColorId.BLUE]: ColorNumber.BLUE,\n};\n\n(ColorIdToNumber[ColorId.GREEN]: 'ffffff'); // oops\n\nmodule.exports = ColorIdToNumber;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var { ColorId, ColorNumber } = require(\"./test\");\nvar ColorIdToNumber = {\n  [ColorId.RED]: ColorNumber.RED,\n  [ColorId.GREEN]: ColorNumber.GREEN,\n  [ColorId.BLUE]: ColorNumber.BLUE,\n};\n\n(ColorIdToNumber[ColorId.GREEN]: \"ffffff\"); // oops\n\nmodule.exports = ColorIdToNumber;");
}
#[test]
fn test_test_3_js_format_1_5a7caf19() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var { ColorId } = require('./test');\nvar ColorIdToNumber = require('./test2');\n\n(ColorIdToNumber[ColorId.BLUE]: 'ffffff'); // oops") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var { ColorId } = require(\"./test\");\nvar ColorIdToNumber = require(\"./test2\");\n\n(ColorIdToNumber[ColorId.BLUE]: \"ffffff\"); // oops");
}
#[test]
fn test_test_4_js_format_1_6f3b07fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("module.exports = 'hello';");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "module.exports = \"hello\";");
}
#[test]
fn test_test_5_js_format_1_d820ad01() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var hello = require('./test4');\nvar dummy = require('./test');\nmodule.exports = {\n  ...dummy,\n  [hello]: 'world',\n  ...dummy,\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var hello = require(\"./test4\");\nvar dummy = require(\"./test\");\nmodule.exports = {\n  ...dummy,\n  [hello]: \"world\",\n  ...dummy,\n};");
}
#[test]
fn test_test_6_js_format_1_328729b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var o = require('./test5');\n(o.hello: 'nothing'); // oops");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "var o = require(\"./test5\");\n(o.hello: \"nothing\"); // oops"
    );
}
#[test]
fn test_test_7_js_format_1_799f65dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var obj = {x: 0, m() { return this.x }}\nvar x: string = obj['m'](); // error, number ~> string\n\nvar arr = [function() { return this.length }];\nvar y: string = arr[0](); // error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var obj = {\n  x: 0,\n  m() {\n    return this.x;\n  },\n};\nvar x: string = obj[\"m\"](); // error, number ~> string\n\nvar arr = [\n  function () {\n    return this.length;\n  },\n];\nvar y: string = arr[0](); // error: number ~> string");
}
