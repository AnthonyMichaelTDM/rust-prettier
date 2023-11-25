#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_constructor_js_format_1_99f51e53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A { x: number; }\n\nclass B {\n  x: number;\n  constructor() {\n    this.x; // OK\n  }\n}\n\nclass C extends A { }\n\nclass D extends A {\n  y: number;\n  constructor() {\n    this.y; // error (no super call)\n    this.x; // error (no super call)\n  }\n}\n\nclass E extends A {\n  y: number;\n  constructor() {\n    super();\n    this.y; // OK\n    this.x; // OK\n  }\n}\n\nfunction leak(f) {\n  f.y; // error\n  f.x; // error\n}\nclass F extends A {\n  y: number;\n  constructor() {\n    leak(this); // error (no super call yet)\n    super();\n    this.y;\n    this.x;\n  }\n}\n\nclass G extends A {\n  constructor(b) {\n    super.x; // error (no super call)\n  }\n}\n\nclass H extends A {\n  y: number;\n  constructor() {\n    if (Math.random() < 0.5)\n      super();\n    else\n      super();\n    this.y; // OK\n    this.x; // OK\n  }\n}\n\nclass I_ {\n  constructor(leaked_this) {\n    leaked_this.foo()\n  }\n  foo() { }\n}\nclass I extends I_ {\n  constructor() {\n    super(this); // error (no super call yet)\n  }\n}\n\nclass J__ { }\nclass J_ extends J__ {\n  constructor(closure_leaking_this) {\n    closure_leaking_this();\n    super();\n  }\n  foo() { }\n}\nclass J extends J_ {\n  constructor() {\n    super(() => this.foo()); // error (no super call yet)\n    // The reason for this error is that super constructor could call the\n    // closure and therefore access this before calling its own super\n    // constructor (as shown above). The only safe thing to do in the super\n    // constructor is to save the closure so that it can be called later, after\n    // initialization is done (as shown below).\n  }\n}\n\nclass K_ {\n  closure_leaking_this: () => void;\n  constructor(closure_leaking_this) {\n    this.closure_leaking_this = closure_leaking_this;\n  }\n  foo() { }\n}\nclass K extends K_ {\n  constructor() {\n    super(() => { if (_this) _this.foo() }); // OK\n    var _this = this;\n    this.closure_leaking_this();\n  }\n}\n\n// even though super() calls the parent's constructor(), it does not do the same\n// conversion on non-objects. so \\`new L_()\\` returns an instance of \\`L_\\` because\n// the constructor returns false (a non-object), but \\`super()\\` returns false,\n// which then similarly causes \\`new L()\\` to return an instance of \\`L\\`.\nclass L_ {\n  constructor() {\n    return false;\n  }\n}\nclass L extends L_ {\n  constructor() {\n    let x: boolean = super();\n    return x;\n  }\n}\n(new L_(): L_);\n(new L(): L);\n\n// similarly, the converse is true: if the parent's constructor returns an\n// object, then the child does too.\nclass M_ {\n  constructor() {\n    return {foo: 'foo'};\n  }\n}\nclass M extends M_ {\n  constructor() {\n    return super();\n  }\n}\n(new M_(): {foo: string});\n(new M(): {foo: string});\n\n// however! super() calls the parent constructor with the subclass as its \\`this\\`\n// value (essentially \\`super.constructor.call(this)\\`).\nclass N_ {\n  constructor(): this {\n    let x = this;\n    return x;\n  }\n}\nclass N extends N_ {\n  constructor() {\n    return super();\n  }\n}\n(new N_(): N_);\n(new N(): N);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  x: number;\n}\n\nclass B {\n  x: number;\n  constructor() {\n    this.x; // OK\n  }\n}\n\nclass C extends A {}\n\nclass D extends A {\n  y: number;\n  constructor() {\n    this.y; // error (no super call)\n    this.x; // error (no super call)\n  }\n}\n\nclass E extends A {\n  y: number;\n  constructor() {\n    super();\n    this.y; // OK\n    this.x; // OK\n  }\n}\n\nfunction leak(f) {\n  f.y; // error\n  f.x; // error\n}\nclass F extends A {\n  y: number;\n  constructor() {\n    leak(this); // error (no super call yet)\n    super();\n    this.y;\n    this.x;\n  }\n}\n\nclass G extends A {\n  constructor(b) {\n    super.x; // error (no super call)\n  }\n}\n\nclass H extends A {\n  y: number;\n  constructor() {\n    if (Math.random() < 0.5) super();\n    else super();\n    this.y; // OK\n    this.x; // OK\n  }\n}\n\nclass I_ {\n  constructor(leaked_this) {\n    leaked_this.foo();\n  }\n  foo() {}\n}\nclass I extends I_ {\n  constructor() {\n    super(this); // error (no super call yet)\n  }\n}\n\nclass J__ {}\nclass J_ extends J__ {\n  constructor(closure_leaking_this) {\n    closure_leaking_this();\n    super();\n  }\n  foo() {}\n}\nclass J extends J_ {\n  constructor() {\n    super(() => this.foo()); // error (no super call yet)\n    // The reason for this error is that super constructor could call the\n    // closure and therefore access this before calling its own super\n    // constructor (as shown above). The only safe thing to do in the super\n    // constructor is to save the closure so that it can be called later, after\n    // initialization is done (as shown below).\n  }\n}\n\nclass K_ {\n  closure_leaking_this: () => void;\n  constructor(closure_leaking_this) {\n    this.closure_leaking_this = closure_leaking_this;\n  }\n  foo() {}\n}\nclass K extends K_ {\n  constructor() {\n    super(() => {\n      if (_this) _this.foo();\n    }); // OK\n    var _this = this;\n    this.closure_leaking_this();\n  }\n}\n\n// even though super() calls the parent's constructor(), it does not do the same\n// conversion on non-objects. so \\`new L_()\\` returns an instance of \\`L_\\` because\n// the constructor returns false (a non-object), but \\`super()\\` returns false,\n// which then similarly causes \\`new L()\\` to return an instance of \\`L\\`.\nclass L_ {\n  constructor() {\n    return false;\n  }\n}\nclass L extends L_ {\n  constructor() {\n    let x: boolean = super();\n    return x;\n  }\n}\n(new L_(): L_);\n(new L(): L);\n\n// similarly, the converse is true: if the parent's constructor returns an\n// object, then the child does too.\nclass M_ {\n  constructor() {\n    return { foo: \"foo\" };\n  }\n}\nclass M extends M_ {\n  constructor() {\n    return super();\n  }\n}\n(new M_(): { foo: string });\n(new M(): { foo: string });\n\n// however! super() calls the parent constructor with the subclass as its \\`this\\`\n// value (essentially \\`super.constructor.call(this)\\`).\nclass N_ {\n  constructor(): this {\n    let x = this;\n    return x;\n  }\n}\nclass N extends N_ {\n  constructor() {\n    return super();\n  }\n}\n(new N_(): N_);\n(new N(): N);");
}
#[test]
fn test_import_js_format_1_d7dcb41c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class D {\n  foo(): number { return 0; }\n}\nmodule.exports = D;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class D {\n  foo(): number {\n    return 0;\n  }\n}\nmodule.exports = D;"
    );
}
#[test]
fn test_super_js_format_1_289bf2ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nclass A {\n  constructor(x:number) { }\n  static staticMethod(x:string): string { return x; }\n  f(x:string) { }\n}\n\nclass B extends A {\n  constructor(x:string,y:number) {\n    super(x);\n  }\n\n  static anotherStatic() {\n    (super.staticMethod('foo'): number); // error, string !~> number\n    (super.doesntExist()); // error, A doesn't have a doesntExist method\n  }\n\n  g() {\n    super.f(0);\n    return super.g;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  constructor(x: number) {}\n  static staticMethod(x: string): string {\n    return x;\n  }\n  f(x: string) {}\n}\n\nclass B extends A {\n  constructor(x: string, y: number) {\n    super(x);\n  }\n\n  static anotherStatic() {\n    (super.staticMethod(\"foo\"): number); // error, string !~> number\n    super.doesntExist(); // error, A doesn't have a doesntExist method\n  }\n\n  g() {\n    super.f(0);\n    return super.g;\n  }\n}");
}
#[test]
fn test_test_js_format_1_aaee8ef3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var D = require('./import');\nclass C extends D {\n  constructor() { return super(); }\n  foo() { return super.foo(); }\n}\nmodule.exports = C;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var D = require(\"./import\");\nclass C extends D {\n  constructor() {\n    return super();\n  }\n  foo() {\n    return super.foo();\n  }\n}\nmodule.exports = C;");
}
