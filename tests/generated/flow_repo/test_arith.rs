#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arith_js_format_1_074ac7ed() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Arith */\n\nfunction num(x:number) { }\n\nfunction str(x:string) { }\n\nfunction foo() {\n  var x = 0;\n  var y = \"...\";\n  var z = {};\n  num(x+x);\n  num(x+y); // error\n  str(x+y);\n  str(x+x); // error\n  str(z+y); // error\n}\n\n// test MaybeT(NumT)\nfunction bar0(x: ?number, y: number) {\n  num(x + y);\n}\nfunction bar1(x: number, y: ?number) {\n  num(x + y);\n}\n\n// test OptionalT(NumT)\nfunction bar2(x?: number, y: number) {\n  num(x + y);\n}\nfunction bar3(x: number, y?: number) {\n  num(x + y);\n}\n\n// test OptionalT(MaybeT(NumT))\nfunction bar4(x?: ?number, y: number) {\n  num(x + y);\n}\nfunction bar5(x: number, y?: ?number) {\n  num(x + y);\n}\n\nnum(null + null); // === 0\nnum(undefined + undefined); // === NaN\n\nnum(null + 1); // === 1\nnum(1 + null); // === 1\nnum(undefined + 1); // === NaN\nnum(1 + undefined); // === NaN\n\nnum(null + true); // === 1\nnum(true + null); // === 1\nnum(undefined + true); // === NaN\nnum(true + undefined); // === NaN\n\nstr(\"foo\" + true); // error\nstr(true + \"foo\"); // error\nstr(\"foo\" + null); // error\nstr(null + \"foo\"); // error\nstr(\"foo\" + undefined); // error\nstr(undefined + \"foo\"); // error\n\nlet tests = [\n  function(x: mixed, y: mixed) {\n    (x + y); // error\n    (x + 0); // error\n    (0 + x); // error\n    (x + \"\"); // error\n    (\"\" + x); // error\n    (x + {}); // error\n    ({} + x); // error\n  },\n\n  // when one side is a string or number and the other is invalid, we\n  // assume you are expecting a string or number (respectively), rather than\n  // erroring twice saying number !~> string and obj !~> string.\n  function() {\n    ((1 + {}): number); // error: object !~> number\n    (({} + 1): number); // error: object !~> number\n    ((\"1\" + {}): string); // error: object !~> string\n    (({} + \"1\"): string); // error: object !~> string\n  },\n\n  function(x: any, y: number, z: string) {\n    (x + y: string); // ok\n    (y + x: string); // ok\n\n    (x + z: empty); // error, string ~> empty\n    (z + x: empty); // error, string ~> empty\n  },\n];") ? ;
    assert_eq ! (formatted , "/* @providesModule Arith */\n\nfunction num(x: number) {}\n\nfunction str(x: string) {}\n\nfunction foo() {\n  var x = 0;\n  var y = \"...\";\n  var z = {};\n  num(x + x);\n  num(x + y); // error\n  str(x + y);\n  str(x + x); // error\n  str(z + y); // error\n}\n\n// test MaybeT(NumT)\nfunction bar0(x: ?number, y: number) {\n  num(x + y);\n}\nfunction bar1(x: number, y: ?number) {\n  num(x + y);\n}\n\n// test OptionalT(NumT)\nfunction bar2(x?: number, y: number) {\n  num(x + y);\n}\nfunction bar3(x: number, y?: number) {\n  num(x + y);\n}\n\n// test OptionalT(MaybeT(NumT))\nfunction bar4(x?: ?number, y: number) {\n  num(x + y);\n}\nfunction bar5(x: number, y?: ?number) {\n  num(x + y);\n}\n\nnum(null + null); // === 0\nnum(undefined + undefined); // === NaN\n\nnum(null + 1); // === 1\nnum(1 + null); // === 1\nnum(undefined + 1); // === NaN\nnum(1 + undefined); // === NaN\n\nnum(null + true); // === 1\nnum(true + null); // === 1\nnum(undefined + true); // === NaN\nnum(true + undefined); // === NaN\n\nstr(\"foo\" + true); // error\nstr(true + \"foo\"); // error\nstr(\"foo\" + null); // error\nstr(null + \"foo\"); // error\nstr(\"foo\" + undefined); // error\nstr(undefined + \"foo\"); // error\n\nlet tests = [\n  function (x: mixed, y: mixed) {\n    x + y; // error\n    x + 0; // error\n    0 + x; // error\n    x + \"\"; // error\n    \"\" + x; // error\n    x + {}; // error\n    ({}) + x; // error\n  },\n\n  // when one side is a string or number and the other is invalid, we\n  // assume you are expecting a string or number (respectively), rather than\n  // erroring twice saying number !~> string and obj !~> string.\n  function () {\n    (1 + {}: number); // error: object !~> number\n    ({} + 1: number); // error: object !~> number\n    (\"1\" + {}: string); // error: object !~> string\n    ({} + \"1\": string); // error: object !~> string\n  },\n\n  function (x: any, y: number, z: string) {\n    (x + y: string); // ok\n    (y + x: string); // ok\n\n    (x + z: empty); // error, string ~> empty\n    (z + x: empty); // error, string ~> empty\n  },\n];");
    Ok(())
}
#[test]
fn test_exponent_js_format_1_ef596f0e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nlet x: number = 2 ** 3;\nx **= 4;\n\nlet y: string = \"123\";\ny **= 2; // error\n\n1 + 2 ** 3 + 4;\n2 ** 2;\n(-2) ** 2;") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nlet x: number = 2 ** 3;\nx **= 4;\n\nlet y: string = \"123\";\ny **= 2; // error\n\n1 + 2 ** 3 + 4;\n2 ** 2;\n(-2) ** 2;");
    Ok(())
}
#[test]
fn test_generic_js_format_1_6806714c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction f<A>(a: A): A { return a + a; } // error\nfunction f<A,B>(a: A, b: B): A {return a + b; } // error\nfunction f<A,B>(a: A, b: B): A {return b + a; } // error\nfunction f<A,B>(a: A, b: B): B {return a + b; } // error\nfunction f<A,B>(a: A, b: B): B {return b + a; } // error") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction f<A>(a: A): A {\n  return a + a;\n} // error\nfunction f<A, B>(a: A, b: B): A {\n  return a + b;\n} // error\nfunction f<A, B>(a: A, b: B): A {\n  return b + a;\n} // error\nfunction f<A, B>(a: A, b: B): B {\n  return a + b;\n} // error\nfunction f<A, B>(a: A, b: B): B {\n  return b + a;\n} // error");
    Ok(())
}
#[test]
fn test_mult_js_format_1_ed8f4ae9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction num(x:number) { }\n\nnum(null * 1);\nnum(1 * null);\n\nlet x: number = 2 * 3;\nx *= 4;\n\nlet y: string = \"123\";\ny *= 2; // error") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction num(x: number) {}\n\nnum(null * 1);\nnum(1 * null);\n\nlet x: number = 2 * 3;\nx *= 4;\n\nlet y: string = \"123\";\ny *= 2; // error");
    Ok(())
}
#[test]
fn test_relational_js_format_1_96ea3136() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(1 < 2);\n(1 < \"foo\"); // error\n(\"foo\" < 1); // error\n(\"foo\" < \"bar\");\n(1 < {foo: 1}); // error\n({foo: 1} < 1); // error\n({foo: 1} < {foo: 1}); // error\n(\"foo\" < {foo: 1}); // error\n({foo: 1} < \"foo\"); // error\n\nvar x = (null : ?number);\n(1 < x); // 2 errors: null !~> number; undefined !~> number\n(x < 1); // 2 errors: null !~> number; undefined !~> number\n\n(null < null); // error\n(undefined < null); // error\n(null < undefined); // error\n(undefined < undefined); // error\n(NaN < 1);\n(1 < NaN);\n(NaN < NaN);\n\nlet tests = [\n  function(x: any, y: number, z: string) {\n    (x > y);\n    (x > z);\n  },\n];") ? ;
    assert_eq ! (formatted , "/* @flow */\n\n1 < 2;\n1 < \"foo\"; // error\n\"foo\" < 1; // error\n\"foo\" < \"bar\";\n1 < { foo: 1 }; // error\n({ foo: 1 }) < 1; // error\n({ foo: 1 }) < { foo: 1 }; // error\n\"foo\" < { foo: 1 }; // error\n({ foo: 1 }) < \"foo\"; // error\n\nvar x = (null: ?number);\n1 < x; // 2 errors: null !~> number; undefined !~> number\nx < 1; // 2 errors: null !~> number; undefined !~> number\n\nnull < null; // error\nundefined < null; // error\nnull < undefined; // error\nundefined < undefined; // error\nNaN < 1;\n1 < NaN;\nNaN < NaN;\n\nlet tests = [\n  function (x: any, y: number, z: string) {\n    x > y;\n    x > z;\n  },\n];");
    Ok(())
}
