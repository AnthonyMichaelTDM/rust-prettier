#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_object_is_js_format_1_d7bc71cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Object.is(1, 1);\nObject.is(1, 2);\nObject.is(1, {});\nObject.is(1, NaN);\nObject.is(0, 0);\nObject.is(0, -0);\nObject.is(NaN, NaN);\nObject.is({}, {});\n\nvar emptyObject = {};\nvar emptyArray = [];\nObject.is(emptyObject, emptyObject);\nObject.is(emptyArray, emptyArray);\nObject.is(emptyObject, emptyArray);\n\nvar squared = x => x * x;\nObject.is(squared, squared);\n\nvar a: boolean = Object.is('a', 'a');\nvar b: string = Object.is('a', 'a');\nvar c: boolean = Object.is('a');\nvar d: boolean = Object.is('a', 'b', 'c'); // Error - 'c' is unused") ? ;
    assert_eq ! (formatted , "Object.is(1, 1);\nObject.is(1, 2);\nObject.is(1, {});\nObject.is(1, NaN);\nObject.is(0, 0);\nObject.is(0, -0);\nObject.is(NaN, NaN);\nObject.is({}, {});\n\nvar emptyObject = {};\nvar emptyArray = [];\nObject.is(emptyObject, emptyObject);\nObject.is(emptyArray, emptyArray);\nObject.is(emptyObject, emptyArray);\n\nvar squared = (x) => x * x;\nObject.is(squared, squared);\n\nvar a: boolean = Object.is(\"a\", \"a\");\nvar b: string = Object.is(\"a\", \"a\");\nvar c: boolean = Object.is(\"a\");\nvar d: boolean = Object.is(\"a\", \"b\", \"c\"); // Error - 'c' is unused");
    Ok(())
}
