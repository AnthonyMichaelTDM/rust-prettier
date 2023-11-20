#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_condition_js_format_1_6e274b50() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @providesModule Condition */\n\nfunction f(x:number) { }\nfunction g() { return (42 || \"hello\"); }\n\nvar x = g();\nif (typeof x === \"string\") {\n  x = 0;\n}\nf(x);\n\nclass A {}\nfunction h() { return (42 || new A()); }\n\nvar y = h();\nif (y instanceof A) {\n  y = 0;\n}\n//f(y);\n\nfunction bar() { return true; }\n\nclass C { qux() { } }\n\nfunction foo() {\n\n  var c = \"...\";\n  c = new C();\n  if (bar()) {\n    c.qux();\n  }\n\n}\n\nfunction goofy() {\n  var x = g();\n  if (typeof x == 'function') {\n    x();\n  } else { // if (typeof x == 'number') {\n    //f(x);\n  }\n}\n\nfunction goofy2() {\n  var o = {x : 0}\n  if (typeof o.x == 'function') {\n    o.x();\n  }\n  var y = o.x;\n  if (typeof y == 'function') {\n    y();\n  } else {\n    //f(y);\n  }\n}\n\nmodule.exports = true;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Condition */\n\nfunction f(x: number) {}\nfunction g() {\n  return 42 || \"hello\";\n}\n\nvar x = g();\nif (typeof x === \"string\") {\n  x = 0;\n}\nf(x);\n\nclass A {}\nfunction h() {\n  return 42 || new A();\n}\n\nvar y = h();\nif (y instanceof A) {\n  y = 0;\n}\n//f(y);\n\nfunction bar() {\n  return true;\n}\n\nclass C {\n  qux() {}\n}\n\nfunction foo() {\n  var c = \"...\";\n  c = new C();\n  if (bar()) {\n    c.qux();\n  }\n}\n\nfunction goofy() {\n  var x = g();\n  if (typeof x == \"function\") {\n    x();\n  } else {\n    // if (typeof x == 'number') {\n    //f(x);\n  }\n}\n\nfunction goofy2() {\n  var o = { x: 0 };\n  if (typeof o.x == \"function\") {\n    o.x();\n  }\n  var y = o.x;\n  if (typeof y == \"function\") {\n    y();\n  } else {\n    //f(y);\n  }\n}\n\nmodule.exports = true;");
}
#[test]
fn test_flow_sa_js_format_1_3dddf903() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule FlowSA */\n\nfunction check(x:string) { }\n\nfunction FlowSA() {\n  var x = 0;\n  x = \"...\";\n  check(x);\n}\n\nmodule.exports = FlowSA;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule FlowSA */\n\nfunction check(x: string) {}\n\nfunction FlowSA() {\n  var x = 0;\n  x = \"...\";\n  check(x);\n}\n\nmodule.exports = FlowSA;");
}
#[test]
fn test_sigma_js_format_1_22494cf5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule Sigma */\n\nclass A { a() {} }\n\nclass B extends A { b() {} }\n\nclass C extends B { c() {} }\n\nfunction bar(x:B) {\n  if (x instanceof A) {\n    x.a();\n    x.c(); // error\n  } else {\n    x++; // TODO no error? since unreachable (x: B implies x: A)\n  }\n}\n\nfunction foo(x:A) {\n  if (x instanceof C) {\n    x.a();\n    x.b();\n    x.c();\n    x.d(); // error\n  } else {\n    x.a();\n    x.c(); // error\n  }\n}\n\n\nclass D { d() {} }\n\nfunction baz(x:D) {\n  if (x instanceof A) {\n    // unreachable, TODO: this shouldn't throw\n  }\n}\n\nmodule.exports = \"sigma\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule Sigma */\n\nclass A {\n  a() {}\n}\n\nclass B extends A {\n  b() {}\n}\n\nclass C extends B {\n  c() {}\n}\n\nfunction bar(x: B) {\n  if (x instanceof A) {\n    x.a();\n    x.c(); // error\n  } else {\n    x++; // TODO no error? since unreachable (x: B implies x: A)\n  }\n}\n\nfunction foo(x: A) {\n  if (x instanceof C) {\n    x.a();\n    x.b();\n    x.c();\n    x.d(); // error\n  } else {\n    x.a();\n    x.c(); // error\n  }\n}\n\nclass D {\n  d() {}\n}\n\nfunction baz(x: D) {\n  if (x instanceof A) {\n    // unreachable, TODO: this shouldn't throw\n  }\n}\n\nmodule.exports = \"sigma\";");
}
#[test]
fn test_test_js_format_1_6989d70e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class BaseClass {\n  baseProp: string;\n}\n\nclass ChildClass extends BaseClass {\n  childProp: string;\n}\n\nfunction test(obj: BaseClass): string {\n  if (obj instanceof ChildClass) {\n    return obj.childProp_TYPO; // error (obj: ChildClass)\n  }\n  return obj.baseProp;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class BaseClass {\n  baseProp: string;\n}\n\nclass ChildClass extends BaseClass {\n  childProp: string;\n}\n\nfunction test(obj: BaseClass): string {\n  if (obj instanceof ChildClass) {\n    return obj.childProp_TYPO; // error (obj: ChildClass)\n  }\n  return obj.baseProp;\n}");
}
