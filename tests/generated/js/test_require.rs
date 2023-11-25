use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_require_js_format_1_f2a4a650() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const { one, two, three, four, five, six, seven, eight, nine, ten } = require('./my-utils');\nconst { one1, two1, three1, four1, five1, six1, seven1, eight1, nine1, ten1, eleven1 } = require('./my-utils');\n\nconst MyReallyExtrememlyLongModuleName = require('MyReallyExtrememlyLongModuleName');\n\nconst plugin = require(\n  global.STANDALONE\n    ? path.join(__dirname, \"../standalone.js\")\n    : path.join(__dirname, \"..\")\n);\n\nconst plugin2 = require(\n  path.join(\n    __dirname,\n    global.STANDALONE ? \"../standalone.js\" : '..'\n  )\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const {\n  one,\n  two,\n  three,\n  four,\n  five,\n  six,\n  seven,\n  eight,\n  nine,\n  ten,\n} = require(\"./my-utils\");\nconst {\n  one1,\n  two1,\n  three1,\n  four1,\n  five1,\n  six1,\n  seven1,\n  eight1,\n  nine1,\n  ten1,\n  eleven1,\n} = require(\"./my-utils\");\n\nconst MyReallyExtrememlyLongModuleName = require(\"MyReallyExtrememlyLongModuleName\");\n\nconst plugin = require(\n  global.STANDALONE\n    ? path.join(__dirname, \"../standalone.js\")\n    : path.join(__dirname, \"..\"),\n);\n\nconst plugin2 = require(\n  path.join(__dirname, global.STANDALONE ? \"../standalone.js\" : \"..\"),\n);");
}
