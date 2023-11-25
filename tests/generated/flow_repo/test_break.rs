#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_js_format_1_0ba05e04() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(b) {\n    var x = b ? null: false;\n    var z;\n    while(b) {\n        if (x == null) { z = \"\"; break; }\n        var y:number = x; // error: boolean !~> number\n    }\n    var w:number = z; // 2 errors: ?string !~> number\n}\n\nfunction bar(b) {\n    var x = b ? null: false;\n    if (x == null) return;\n    switch (\"\") {\n    case 0:\n        var y:number = x; // error: boolean !~> number\n        x = \"\";\n    case 1:\n        var z:number = x; // 2 errors: (boolean | string) !~> number\n        break;\n    case 2:\n    }\n    var w:number = x; // 2 errors: (boolean | string) !~> number\n}\n\nfunction bar2(b) {\n    var x = b ? null: false;\n    if (x == null) return;\n    switch (\"\") {\n    case 0: {\n      let y:number = x; // error: boolean !~> number\n      x = \"\";\n    }\n    case 1: {\n      let z:number = x; // 2 errors: (boolean | string) !~> number\n      break;\n    }\n    case 2:\n    }\n    var w:number = x; // 2 errors: (boolean | string) !~> number\n}\n\nfunction qux(b) {\n    var z = 0;\n    while(b) {\n        var y:number = z;\n        if (b) { z = \"\"; continue; } // error: string !~> number\n        z = 0;\n    }\n    var w:number = z; // error: string !~> number\n}\n\n// same basic test as foo(), but with const. probes the\n// logic that still uses havoc to do env resets.\nfunction test_const() {\n  let st: string = 'abc';\n\n  for (let i = 1; i < 100; i++) {\n    const fooRes: ?string = \"HEY\";\n    if (!fooRes) {\n      break;\n    }\n\n    st = fooRes; // no error\n  }\n\n  return st;\n}") ? ;
    assert_eq ! (formatted , "function foo(b) {\n  var x = b ? null : false;\n  var z;\n  while (b) {\n    if (x == null) {\n      z = \"\";\n      break;\n    }\n    var y: number = x; // error: boolean !~> number\n  }\n  var w: number = z; // 2 errors: ?string !~> number\n}\n\nfunction bar(b) {\n  var x = b ? null : false;\n  if (x == null) return;\n  switch (\"\") {\n    case 0:\n      var y: number = x; // error: boolean !~> number\n      x = \"\";\n    case 1:\n      var z: number = x; // 2 errors: (boolean | string) !~> number\n      break;\n    case 2:\n  }\n  var w: number = x; // 2 errors: (boolean | string) !~> number\n}\n\nfunction bar2(b) {\n  var x = b ? null : false;\n  if (x == null) return;\n  switch (\"\") {\n    case 0: {\n      let y: number = x; // error: boolean !~> number\n      x = \"\";\n    }\n    case 1: {\n      let z: number = x; // 2 errors: (boolean | string) !~> number\n      break;\n    }\n    case 2:\n  }\n  var w: number = x; // 2 errors: (boolean | string) !~> number\n}\n\nfunction qux(b) {\n  var z = 0;\n  while (b) {\n    var y: number = z;\n    if (b) {\n      z = \"\";\n      continue;\n    } // error: string !~> number\n    z = 0;\n  }\n  var w: number = z; // error: string !~> number\n}\n\n// same basic test as foo(), but with const. probes the\n// logic that still uses havoc to do env resets.\nfunction test_const() {\n  let st: string = \"abc\";\n\n  for (let i = 1; i < 100; i++) {\n    const fooRes: ?string = \"HEY\";\n    if (!fooRes) {\n      break;\n    }\n\n    st = fooRes; // no error\n  }\n\n  return st;\n}");
    Ok(())
}
