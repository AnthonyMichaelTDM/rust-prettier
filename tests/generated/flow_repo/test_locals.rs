#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lex_js_format_1_c9c6f09b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function switch_scope(x: mixed) {\n  let a = \"\";\n  let b = \"\";\n  switch (x) {\n    case \"foo\":\n      let a;\n      a = 0; // doesn't add lower bound to outer a\n      b = 0;\n  }\n  (a : string); // OK\n  (b : string); // error: number ~> string\n}\n\nfunction try_scope_finally() {\n  let a;\n  let b;\n  try {\n    a = \"\";\n    b = \"\";\n  } finally {\n    let a;\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a : string); // ok\n  (b : string); // error: number ~> string\n}\n\nfunction for_scope() {\n  let a = \"\";\n  let b = \"\";\n  for (let a;;) {\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a : string);\n  (b : string); // error: number ~> string\n}\n\nfunction for_in_scope(o: Object) {\n  let a = 0;\n  let b = 0;\n  for (let a in o) {\n    a = \"\"; // doesn't add lower bound to outer a\n    b = \"\";\n  }\n  (a : number);\n  (b : number); // error: string ~> number\n}\n\nfunction for_of_scope(xs: number[]) {\n  let a = \"\";\n  let b = \"\";\n  for (let a of xs) {\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a : string);\n  (b : string); // error: number ~> string\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function switch_scope(x: mixed) {\n  let a = \"\";\n  let b = \"\";\n  switch (x) {\n    case \"foo\":\n      let a;\n      a = 0; // doesn't add lower bound to outer a\n      b = 0;\n  }\n  (a: string); // OK\n  (b: string); // error: number ~> string\n}\n\nfunction try_scope_finally() {\n  let a;\n  let b;\n  try {\n    a = \"\";\n    b = \"\";\n  } finally {\n    let a;\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a: string); // ok\n  (b: string); // error: number ~> string\n}\n\nfunction for_scope() {\n  let a = \"\";\n  let b = \"\";\n  for (let a; ; ) {\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a: string);\n  (b: string); // error: number ~> string\n}\n\nfunction for_in_scope(o: Object) {\n  let a = 0;\n  let b = 0;\n  for (let a in o) {\n    a = \"\"; // doesn't add lower bound to outer a\n    b = \"\";\n  }\n  (a: number);\n  (b: number); // error: string ~> number\n}\n\nfunction for_of_scope(xs: number[]) {\n  let a = \"\";\n  let b = \"\";\n  for (let a of xs) {\n    a = 0; // doesn't add lower bound to outer a\n    b = 0;\n  }\n  (a: string);\n  (b: string); // error: number ~> string\n}");
}
#[test]
fn test_locals_js_format_1_609cd7d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x:string = 0;\nvar x:number = 1;\n\n//declare var T: $Type<number | Array<T>>;\n\nfunction foo(p: bool) {}\n\nfunction sorry(really: bool) {\n    if (really) {\n        var x: number | string = 1337;\n    } else {\n        var x: bool = true;\n    }\n    foo(x);\n}\n\nfunction foo0(b: bool): number {\n  var x = 0;\n  if (b) {\n    var x = \"\"; // error: string ~> number\n  }\n  return x;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x: string = 0;\nvar x: number = 1;\n\n//declare var T: $Type<number | Array<T>>;\n\nfunction foo(p: boolean) {}\n\nfunction sorry(really: boolean) {\n  if (really) {\n    var x: number | string = 1337;\n  } else {\n    var x: boolean = true;\n  }\n  foo(x);\n}\n\nfunction foo0(b: boolean): number {\n  var x = 0;\n  if (b) {\n    var x = \"\"; // error: string ~> number\n  }\n  return x;\n}");
}
