use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_0340c3f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Test resolution precedence in node:\n * checked module > lib def > unchecked module\n *\n * @flow\n */\n\n// node_modules/buffer/index.js is unchecked,\n// so we shouldn't pick up its boolean redefinition of INSPECT_MAX_BYTES\n//\nvar buffer = require(\"buffer\");\nvar b: boolean = buffer.INSPECT_MAX_BYTES; // error, number ~/> boolean\n\n// node_modules/crypto/index.js is checked,\n// so we should pick up its boolean redefinition of DEFAULT_ENCODING\n//\nvar crypto = require(\"crypto\");\nvar b: boolean = crypto.DEFAULT_ENCODING; // no error, we've overridden\n\n// names that are explicit paths shouldn't fall back to lib defs\n//\nvar buffer2 = require(\"./buffer\");\nvar x2: string = buffer2.INSPECT_MAX_BYTES; // error, module not found\n\nvar buffer3 = require(\"./buffer.js\");\nvar x3: string = buffer3.INSPECT_MAX_BYTES; // error, module not found") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Test resolution precedence in node:\n * checked module > lib def > unchecked module\n *\n * @flow\n */\n\n// node_modules/buffer/index.js is unchecked,\n// so we shouldn't pick up its boolean redefinition of INSPECT_MAX_BYTES\n//\nvar buffer = require(\"buffer\");\nvar b: boolean = buffer.INSPECT_MAX_BYTES; // error, number ~/> boolean\n\n// node_modules/crypto/index.js is checked,\n// so we should pick up its boolean redefinition of DEFAULT_ENCODING\n//\nvar crypto = require(\"crypto\");\nvar b: boolean = crypto.DEFAULT_ENCODING; // no error, we've overridden\n\n// names that are explicit paths shouldn't fall back to lib defs\n//\nvar buffer2 = require(\"./buffer\");\nvar x2: string = buffer2.INSPECT_MAX_BYTES; // error, module not found\n\nvar buffer3 = require(\"./buffer.js\");\nvar x3: string = buffer3.INSPECT_MAX_BYTES; // error, module not found");
}
