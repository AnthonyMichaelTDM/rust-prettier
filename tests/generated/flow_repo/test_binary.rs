#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_in_js_format_1_3d3034ff() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  // objects on RHS\n  function() {\n    ('foo' in {});\n    ('foo' in { foo: null });\n    (0 in {});\n    (0 in { \"0\": null });\n  },\n\n  // arrays on RHS\n  function() {\n    ('foo' in []);\n    (0 in []);\n    ('length' in []);\n  },\n\n  // primitive classes on RHS\n  function() {\n    ('foo' in new String('bar'));\n    ('foo' in new Number(123));\n  },\n\n  // primitives on RHS\n  function() {\n    ('foo' in 123); // error\n    ('foo' in 'bar'); // error\n    ('foo' in void 0); // error\n    ('foo' in null); // error\n  },\n\n  // bogus stuff on LHS\n  function() {\n    (null in {}); // error\n    (void 0 in {}); // error\n    ({} in {}); // error\n    ([] in {}); // error\n    (false in []); // error\n  },\n\n  // in predicates\n  function() {\n    if ('foo' in 123) {} // error\n    if (!'foo' in {}) {} // error, !'foo' is a boolean\n    if (!('foo' in {})) {}\n  },\n\n  // annotations on RHS\n  function(x: Object, y: mixed) {\n    ('foo' in x); // ok\n    ('foo' in y); // error\n  },\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  // objects on RHS\n  function () {\n    \"foo\" in {};\n    \"foo\" in { foo: null };\n    0 in {};\n    0 in { \"0\": null };\n  },\n\n  // arrays on RHS\n  function () {\n    \"foo\" in [];\n    0 in [];\n    \"length\" in [];\n  },\n\n  // primitive classes on RHS\n  function () {\n    \"foo\" in new String(\"bar\");\n    \"foo\" in new Number(123);\n  },\n\n  // primitives on RHS\n  function () {\n    \"foo\" in 123; // error\n    \"foo\" in \"bar\"; // error\n    \"foo\" in void 0; // error\n    \"foo\" in null; // error\n  },\n\n  // bogus stuff on LHS\n  function () {\n    null in {}; // error\n    (void 0) in {}; // error\n    ({}) in {}; // error\n    [] in {}; // error\n    false in []; // error\n  },\n\n  // in predicates\n  function () {\n    if (\"foo\" in 123) {\n    } // error\n    if ((!\"foo\") in {}) {\n    } // error, !'foo' is a boolean\n    if (!(\"foo\" in {})) {\n    }\n  },\n\n  // annotations on RHS\n  function (x: Object, y: mixed) {\n    \"foo\" in x; // ok\n    \"foo\" in y; // error\n  },\n];");
}
