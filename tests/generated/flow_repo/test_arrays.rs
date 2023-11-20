#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrays_js_format_1_1cc13883() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Arrays */\n\nfunction foo(x:string) { }\n\nvar a = [];\na[0] = 1;\na[1] = \"...\";\n\nfoo(a[1]);\nvar y;\na.forEach(x => y=x);\n\n// for literals, composite element type is union of individuals\n// note: test both tuple and non-tuple inferred literals\nvar alittle: Array<?number> = [0, 1, 2, 3, null];\nvar abig: Array<?number> = [0, 1, 2, 3, 4, 5, 6, 8, null];\n\nvar abig2: Array<{x:number; y:number}> = [\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0},\n  {x:0, y:0, a:true},\n  {x:0, y:0, b:\"hey\"},\n  {x:0, y:0, c:1},\n  {x:0, y:0, c:\"hey\"}\n];\n\nmodule.exports = \"arrays\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Arrays */\n\nfunction foo(x: string) {}\n\nvar a = [];\na[0] = 1;\na[1] = \"...\";\n\nfoo(a[1]);\nvar y;\na.forEach((x) => (y = x));\n\n// for literals, composite element type is union of individuals\n// note: test both tuple and non-tuple inferred literals\nvar alittle: Array<?number> = [0, 1, 2, 3, null];\nvar abig: Array<?number> = [0, 1, 2, 3, 4, 5, 6, 8, null];\n\nvar abig2: Array<{ x: number, y: number }> = [\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0 },\n  { x: 0, y: 0, a: true },\n  { x: 0, y: 0, b: \"hey\" },\n  { x: 0, y: 0, c: 1 },\n  { x: 0, y: 0, c: \"hey\" },\n];\n\nmodule.exports = \"arrays\";");
}
#[test]
fn test_numeric_elem_js_format_1_036a22c5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var arr = [];\nvar day = new Date;\n\n// Date instances are numeric (see Flow_js.numeric) and thus can index into\n// arrays.\narr[day] = 0;\n(arr[day]: string); // error: number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var arr = [];\nvar day = new Date();\n\n// Date instances are numeric (see Flow_js.numeric) and thus can index into\n// arrays.\narr[day] = 0;\n(arr[day]: string); // error: number ~> string");
}
