use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_compatibility_js_format_1_91a46407() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let tests = [\n  function(x: { x: { foo: string } }, y: { x: { bar: number } }) {\n    x = y; // 2 errors: \\`foo\\` not found in y.x; \\`bar\\` not found in x.x\n  },\n\n  function(x: { foo: string }, y: { foo: number }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function(x: { x: { foo: string } }, y: { x: { foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function(x: { +foo: string }, y: { +foo: number }) {\n    x = y; // 1 error: number !~> string\n  },\n\n  function(x: { x: { +foo: string } }, y: { x: { +foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function(x: { -foo: string }, y: { -foo: number }) {\n    x = y; // 1 error: string !~> number\n  },\n\n  function(x: { x: { -foo: string } }, y: { x: { -foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let tests = [\n  function (x: { x: { foo: string } }, y: { x: { bar: number } }) {\n    x = y; // 2 errors: \\`foo\\` not found in y.x; \\`bar\\` not found in x.x\n  },\n\n  function (x: { foo: string }, y: { foo: number }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function (x: { x: { foo: string } }, y: { x: { foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function (x: { +foo: string }, y: { +foo: number }) {\n    x = y; // 1 error: number !~> string\n  },\n\n  function (x: { x: { +foo: string } }, y: { x: { +foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n\n  function (x: { -foo: string }, y: { -foo: number }) {\n    x = y; // 1 error: string !~> number\n  },\n\n  function (x: { x: { -foo: string } }, y: { x: { -foo: number } }) {\n    x = y; // 2 errors: string !~> number; number !~> string\n  },\n];");
}
#[test]
fn test_conversion_js_format_1_c66de3ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(Object({foo: 'bar'}): {foo: string});\n(Object(\"123\"): String);\n(Object(123): Number);\n(Object(true): Boolean);\n(Object(null): {});\n(Object(undefined): {});\n(Object(void(0)): {});\n(Object(undefined): Number); // error\n\nvar x = Object(null);\nx.foo = \"bar\";\n\nvar y = Object(\"123\");\n(y.charAt(0): string);\n\nvar z = Object(123); // error (next line makes this not match any signatures)\n(z.charAt(0): string);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(Object({ foo: \"bar\" }): { foo: string });\n(Object(\"123\"): String);\n(Object(123): Number);\n(Object(true): Boolean);\n(Object(null): {});\n(Object(undefined): {});\n(Object(void 0): {});\n(Object(undefined): Number); // error\n\nvar x = Object(null);\nx.foo = \"bar\";\n\nvar y = Object(\"123\");\n(y.charAt(0): string);\n\nvar z = Object(123); // error (next line makes this not match any signatures)\n(z.charAt(0): string);");
}
#[test]
fn test_objects_js_format_1_92f360d5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar x : {'123': string, bar: string} = {'123': 'val', bar: 'bar'};\n(x.foo : string);     // error, key doesn't exist\n(x['foo'] : string);  // error, key doesn't exist\n(x[123] : boolean);   // TODO: use the number's value to error here\n(x.bar: boolean);     // error, string !~> boolean\n(x['123'] : boolean); // error, string !~> boolean\nx['123'] = false;     // error, boolean !~> string\nx[123] = false;       // TODO: use the number's value to error here\nx['foo'+'bar'] = 'derp'; // ok since we can't tell\n(x[\\`foo\\`]: string);   // error, key doesn't exist\n\nvar y : {foo: string} = {foo: 'bar'};\ny['foo'] = 123; // error, number !~> string\ny['bar'] = 'abc'; // error, property not found\n\n(y['hasOwnProperty']: string); // error, prototype method is not a string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar x: { \"123\": string, bar: string } = { \"123\": \"val\", bar: \"bar\" };\n(x.foo: string); // error, key doesn't exist\n(x[\"foo\"]: string); // error, key doesn't exist\n(x[123]: boolean); // TODO: use the number's value to error here\n(x.bar: boolean); // error, string !~> boolean\n(x[\"123\"]: boolean); // error, string !~> boolean\nx[\"123\"] = false; // error, boolean !~> string\nx[123] = false; // TODO: use the number's value to error here\nx[\"foo\" + \"bar\"] = \"derp\"; // ok since we can't tell\n(x[\\`foo\\`]: string); // error, key doesn't exist\n\nvar y: { foo: string } = { foo: \"bar\" };\ny[\"foo\"] = 123; // error, number !~> string\ny[\"bar\"] = \"abc\"; // error, property not found\n\n(y[\"hasOwnProperty\"]: string); // error, prototype method is not a string");
}
#[test]
fn test_unaliased_assign_js_format_1_d9f78bcd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * test handling of unaliased value assignment.\n *\n * An unaliased object rvalue may be assigned to a supertype lvalue,\n * because later widening mutations on the rvalue can't break assumptions\n * made by other lvalues.\n *\n * However, upon assignment the rvalue must take on the type of the\n * lvalue, to avoid both false positives and false negatives\n * (unsoundness), as shown below.\n *\n * @flow\n */\n\nvar glob: { x: string } = { x: \"hey\" };\n\nfunction assign_then_alias() {\n  var obj: { x: string | number };\n  obj = { x: \"hey\" };\n  glob = obj;    // error: subsequent assignment might make glob.x a number\n}\n\nfunction assign_then_widen() {\n  var obj: { x: string | number };\n  obj = { x: \"hey\" };\n  obj.x = 10;  // ok, by lvalue's given type\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * test handling of unaliased value assignment.\n *\n * An unaliased object rvalue may be assigned to a supertype lvalue,\n * because later widening mutations on the rvalue can't break assumptions\n * made by other lvalues.\n *\n * However, upon assignment the rvalue must take on the type of the\n * lvalue, to avoid both false positives and false negatives\n * (unsoundness), as shown below.\n *\n * @flow\n */\n\nvar glob: { x: string } = { x: \"hey\" };\n\nfunction assign_then_alias() {\n  var obj: { x: string | number };\n  obj = { x: \"hey\" };\n  glob = obj; // error: subsequent assignment might make glob.x a number\n}\n\nfunction assign_then_widen() {\n  var obj: { x: string | number };\n  obj = { x: \"hey\" };\n  obj.x = 10; // ok, by lvalue's given type\n}");
}
