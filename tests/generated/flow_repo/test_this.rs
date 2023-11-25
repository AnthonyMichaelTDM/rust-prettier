use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_this_js_format_1_833b153b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @providesModule This */\n\nfunction F() { this.x = 0; }\nF.prototype.m = function() { this.y = 0; }\n\nfunction foo(p:string) { }\n\nvar o1 = new F(); // sets o1.x to 0\no1.x = \"\";\nfoo(o1.x); // ok by definite assignment\n\nvar o2 = new F();\no1.y = 0;\no2.y = \"\";\nfoo(o2.y); // setting o1.y to 0 has no effect on o2.y\n\nvar o3 = new F();\no3.m(); // sets o3.y to 0\no3.y = \"\";\nfoo(o3.y); // ok by definite assignment\nfoo(o2.y); // setting o3.y to 0 has no effect on o2.y\n\n/*\n * this bindings:\n */\n\n/* standard functions may rebind this */\nfunction f1() : number {\n  return this.x\n}\n\nvar f1_1 = f1.bind({x: 0})();            // ok\nvar f1_2 : string = f1.bind({x: 0})();   // error, number -> string\nvar f1_3 = f1.bind({x: \"\"})();           // error, string -> number\n// TODO make this error blame the call site, rather than the function body\nvar f1_4 = f1();                         // error, (global object).x\n\n/* arrow functions bind this at point of definition */\n/* top level arrow functions bind this to global object */\nvar a1 = () => {\n  return this.x\n}\n\nvar ax = a1();                          // error, (this:mixed).x\n\n/* nested arrows bind enclosing this (which may itself rebind) */\nfunction f2() : number {\n    var a2 = () => { return this.x };\n    return a2()\n}\n\nvar f2_1 = f2.bind({x: 0})();            // ok\nvar f2_2 : string = f2.bind({x: 0})();   // error, number -> string\nvar f2_3 = f2.bind({x: \"\"})();           // error, string -> number\n// TODO make this error blame the call site, rather than the function body\nvar f2_4 = f2();                         // error, (global object).x\n\n(this: void);\n\nmodule.exports = true;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule This */\n\nfunction F() {\n  this.x = 0;\n}\nF.prototype.m = function () {\n  this.y = 0;\n};\n\nfunction foo(p: string) {}\n\nvar o1 = new F(); // sets o1.x to 0\no1.x = \"\";\nfoo(o1.x); // ok by definite assignment\n\nvar o2 = new F();\no1.y = 0;\no2.y = \"\";\nfoo(o2.y); // setting o1.y to 0 has no effect on o2.y\n\nvar o3 = new F();\no3.m(); // sets o3.y to 0\no3.y = \"\";\nfoo(o3.y); // ok by definite assignment\nfoo(o2.y); // setting o3.y to 0 has no effect on o2.y\n\n/*\n * this bindings:\n */\n\n/* standard functions may rebind this */\nfunction f1(): number {\n  return this.x;\n}\n\nvar f1_1 = f1.bind({ x: 0 })(); // ok\nvar f1_2: string = f1.bind({ x: 0 })(); // error, number -> string\nvar f1_3 = f1.bind({ x: \"\" })(); // error, string -> number\n// TODO make this error blame the call site, rather than the function body\nvar f1_4 = f1(); // error, (global object).x\n\n/* arrow functions bind this at point of definition */\n/* top level arrow functions bind this to global object */\nvar a1 = () => {\n  return this.x;\n};\n\nvar ax = a1(); // error, (this:mixed).x\n\n/* nested arrows bind enclosing this (which may itself rebind) */\nfunction f2(): number {\n  var a2 = () => {\n    return this.x;\n  };\n  return a2();\n}\n\nvar f2_1 = f2.bind({ x: 0 })(); // ok\nvar f2_2: string = f2.bind({ x: 0 })(); // error, number -> string\nvar f2_3 = f2.bind({ x: \"\" })(); // error, string -> number\n// TODO make this error blame the call site, rather than the function body\nvar f2_4 = f2(); // error, (global object).x\n\n(this: void);\n\nmodule.exports = true;");
}
#[test]
fn test_arrows_js_format_1_10346c31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  foo() {\n    return () => { return this.bar(); }; // OK, since this: C\n  }\n  bar() { return this; } // return type is C\n}\nvar c = new C;\nvar f = c.foo();\nvar i = f(); // OK\n(i: C); // OK\n\nclass D extends C { }\nvar d = new D;\nvar g = d.foo();\nvar j = g(); // OK\n(j: D); // error, since return type of bar is C, not the type of \\`this\\`\n\nclass E {\n  foo(x: number) { }\n}\nclass F extends E {\n  foo() { // OK to override with generalization\n    (() => {\n      super.foo(\"\"); // find super method, error due to incorrect arg\n    })();\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  foo() {\n    return () => {\n      return this.bar();\n    }; // OK, since this: C\n  }\n  bar() {\n    return this;\n  } // return type is C\n}\nvar c = new C();\nvar f = c.foo();\nvar i = f(); // OK\n(i: C); // OK\n\nclass D extends C {}\nvar d = new D();\nvar g = d.foo();\nvar j = g(); // OK\n(j: D); // error, since return type of bar is C, not the type of \\`this\\`\n\nclass E {\n  foo(x: number) {}\n}\nclass F extends E {\n  foo() {\n    // OK to override with generalization\n    (() => {\n      super.foo(\"\"); // find super method, error due to incorrect arg\n    })();\n  }\n}");
}
