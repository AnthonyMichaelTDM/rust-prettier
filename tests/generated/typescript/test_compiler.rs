#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_class_declaration_22_ts_format_1_e048cef4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("class C {\n    \"foo\"();\n    \"bar\"() { }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class C {\n  foo();\n  bar() {}\n}");
}
#[test]
fn test_any_is_assignable_to_object_ts_format_1_4ca91a37() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("interface P {\n    p: {};\n}\n\ninterface Q extends P { // Check assignability here. Any is assignable to {}\n    p: any;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface P {\n  p: {};\n}\n\ninterface Q extends P {\n  // Check assignability here. Any is assignable to {}\n  p: any;\n}");
}
#[test]
fn test_cast_of_await_ts_format_1_30e65afa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @target: es6\nasync function f() {\n    <number> await 0;\n    typeof await 0;\n    void await 0;\n    await void <string> typeof <number> void await 0;\n    await await 0;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @target: es6\nasync function f() {\n  <number>await 0;\n  typeof (await 0);\n  void (await 0);\n  await void (<string>typeof (<number>void (await 0)));\n  await await 0;\n}");
}
#[test]
fn test_cast_parentheses_ts_format_1_47807bfc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\u{feff}class a {\n    static b: any;\n}\n\nvar b = (<any>a);\nvar b = (<any>a).b;\nvar b = (<any>a.b).c;\nvar b = (<any>a.b()).c;\nvar b = (<any>new a);\nvar b = (<any>new a.b);\nvar b = (<any>new a).b") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\u{feff}class a {\n  static b: any;\n}\n\nvar b = <any>a;\nvar b = (<any>a).b;\nvar b = (<any>a.b).c;\nvar b = (<any>a.b()).c;\nvar b = <any>new a();\nvar b = <any>new a.b();\nvar b = (<any>new a()).b;");
}
#[test]
fn test_cast_test_ts_format_1_6000ec9f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\nvar x : any = 0;\nvar z = <number> x;\nvar y = x + z;\n\nvar a = <any>0;\nvar b = <boolean>true;\nvar s = <string>\"\";\n\nvar ar = <any[]>null;\n\nvar f = <(res : number) => void>null;\n\ndeclare class Point\n{\n    x: number;\n    y: number;\n    add(dx: number, dy: number): Point;\n    mult(p: Point): Point;\n    constructor(x: number, y: number);\n}\n\nvar p_cast = <Point> ({\n    x: 0,\n    y: 0,\n    add: function(dx, dy) {\n        return new Point(this.x + dx, this.y + dy);\n    },\n    mult: function(p) { return p; }\n})") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x: any = 0;\nvar z = <number>x;\nvar y = x + z;\n\nvar a = <any>0;\nvar b = <boolean>true;\nvar s = <string>\"\";\n\nvar ar = <any[]>null;\n\nvar f = <(res: number) => void>null;\n\ndeclare class Point {\n  x: number;\n  y: number;\n  add(dx: number, dy: number): Point;\n  mult(p: Point): Point;\n  constructor(x: number, y: number);\n}\n\nvar p_cast = <Point>{\n  x: 0,\n  y: 0,\n  add: function (dx, dy) {\n    return new Point(this.x + dx, this.y + dy);\n  },\n  mult: function (p) {\n    return p;\n  },\n};");
}
#[test]
fn test_check_infinite_expansion_termination_ts_format_1_996b7072() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Regression test for #1002\n// Before fix this code would cause infinite loop\n\ninterface IObservable<T> {\n    n: IObservable<T[]>; // Needed, must be T[]\n}\n\n// Needed\ninterface ISubject<T> extends IObservable<T> { }\n\ninterface Foo { x }\ninterface Bar { y }\n\nvar values: IObservable<Foo>;\nvar values2: ISubject<Bar>;\nvalues = values2;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Regression test for #1002\n// Before fix this code would cause infinite loop\n\ninterface IObservable<T> {\n  n: IObservable<T[]>; // Needed, must be T[]\n}\n\n// Needed\ninterface ISubject<T> extends IObservable<T> {}\n\ninterface Foo {\n  x;\n}\ninterface Bar {\n  y;\n}\n\nvar values: IObservable<Foo>;\nvar values2: ISubject<Bar>;\nvalues = values2;");
}
#[test]
fn test_comment_in_namespace_declaration_with_identifier_path_name_ts_format_1_a7bf0779() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("\u{feff}namespace hello.hi.world\n{\n    function foo() {}\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\u{feff}namespace hello.hi.world {\n  function foo() {}\n}"
    );
}
#[test]
fn test_comments_interface_ts_format_1_7c5d47a7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("interface i2 {\n    foo: (/**param help*/b: number) => string;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "interface i2 {\n  foo: (/**param help*/ b: number) => string;\n}"
    );
}
#[test]
fn test_contextual_signature_instantiation_2_ts_format_1_61fa81f1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// dot f g x = f(g(x))\nvar dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;\ndot = <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T): (r:U) => S => (x) => f(g(x));\nvar id: <T>(x:T) => T;\nvar r23 = dot(id)(id)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// dot f g x = f(g(x))\nvar dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;\ndot =\n  <T, S>(f: (_: T) => S) =>\n  <U>(g: (_: U) => T): ((r: U) => S) =>\n  (x) =>\n    f(g(x));\nvar id: <T>(x: T) => T;\nvar r23 = dot(id)(id);");
}
#[test]
fn test_declare_dotted_module_name_ts_format_1_b780bed0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @declaration: true\nmodule M {\n    module P.Q { } // This shouldnt be emitted\n}\n\nmodule M {\n    export module R.S { }  //This should be emitted\n}\n\nmodule T.U { // This needs to be emitted\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @declaration: true\nmodule M {\n  module P.Q {} // This shouldnt be emitted\n}\n\nmodule M {\n  export module R.S {} //This should be emitted\n}\n\nmodule T.U {\n  // This needs to be emitted\n}");
}
#[test]
fn test_decrement_and_increment_operators_ts_babel_ts_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_decrement_and_increment_operators_ts_format_1_587118d5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var x = 0;\n\n// errors\n1 ++;\n\n(1)++;\n(1)--;\n\n++(1);\n--(1);\n\n(1 + 2)++;\n(1 + 2)--;\n\n++(1 + 2);\n--(1 + 2);\n\n(x + x)++;\n(x + x)--;\n\n++(x + x);\n--(x + x);\n\n//OK\nx++;\nx--;\n\n++x;\n--x;\n\n(x)++;\n--(x);\n\n((x))++;\n((x))--;\n\nx[x++]++;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x = 0;\n\n// errors\n1++;\n\n1++;\n1--;\n\n++1;\n--1;\n\n(1 + 2)++;\n(1 + 2)--;\n\n++(1 + 2);\n--(1 + 2);\n\n(x + x)++;\n(x + x)--;\n\n++(x + x);\n--(x + x);\n\n//OK\nx++;\nx--;\n\n++x;\n--x;\n\nx++;\n--x;\n\nx++;\nx--;\n\nx[x++]++;");
}
#[test]
fn test_es_5_export_default_class_declaration_4_ts_format_1_392bca79() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @target: es5\n// @module: commonjs\n// @declaration: true\n\ndeclare module \"foo\" {\n    export var before: C;\n\n    export default class C {\n        method(): C;\n    }\n\n    export var after: C;\n\n    export var t: typeof C;\n}\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @target: es5\n// @module: commonjs\n// @declaration: true\n\ndeclare module \"foo\" {\n  export var before: C;\n\n  export default class C {\n    method(): C;\n  }\n\n  export var after: C;\n\n  export var t: typeof C;\n}");
}
#[test]
fn test_function_overloads_on_generic_arity_1_ts_format_1_bc0a2247() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// overloading on arity not allowed\ninterface C {\n   f<T>(): string;\n   f<T, U>(): string; \n \n   <T>(): string;\n   <T, U>(): string; \n \n  new <T>(): string;\n  new <T, U>(): string; \n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// overloading on arity not allowed\ninterface C {\n  f<T>(): string;\n  f<T, U>(): string;\n\n  <T>(): string;\n  <T, U>(): string;\n\n  new <T>(): string;\n  new <T, U>(): string;\n}");
}
#[test]
fn test_global_is_contextual_keyword_ts_format_1_94c77f14() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function a() {\n    let global = 1;\n}\nfunction b() {\n    class global {}\n}\n\nnamespace global {\n}\n\nfunction foo(global: number) {\n}\n\nlet obj = {\n    global: \"123\"\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function a() {\n  let global = 1;\n}\nfunction b() {\n  class global {}\n}\n\nnamespace global {}\n\nfunction foo(global: number) {}\n\nlet obj = {\n  global: \"123\",\n};");
}
#[test]
fn test_index_signature_with_initializer_ts_format_1_b0820cc3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// These used to be indexers, now they are computed properties\ninterface I {\n    [x = '']: string;\n}\n\nclass C {\n    [x = 0]: string\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// These used to be indexers, now they are computed properties\ninterface I {\n  [x = \"\"]: string;\n}\n\nclass C {\n  [x = 0]: string;\n}");
}
#[test]
fn test_mapped_type_with_combined_type_mappers_ts_format_1_c402ded1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Repro from #13351\n\ntype Meta<T, A> = {\n    [P in keyof T]: {\n        value: T[P];\n        also: A;\n        readonly children: Meta<T[P], A>;\n    };\n}\n\ninterface Input {\n    x: string;\n    y: number;\n}\n\ndeclare const output: Meta<Input, boolean>;\n\nconst shouldFail: { important: boolean } = output.x.children;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Repro from #13351\n\ntype Meta<T, A> = {\n  [P in keyof T]: {\n    value: T[P];\n    also: A;\n    readonly children: Meta<T[P], A>;\n  };\n};\n\ninterface Input {\n  x: string;\n  y: number;\n}\n\ndeclare const output: Meta<Input, boolean>;\n\nconst shouldFail: { important: boolean } = output.x.children;");
}
#[test]
fn test_modifiers_on_interface_index_signature_1_ts_format_1_48df6fcb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("interface I {\n  [a: string]: number;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "interface I {\n  [a: string]: number;\n}");
}
#[test]
fn test_privacy_glo_import_ts_format_1_ddd94c90() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("//@declaration: true\nmodule m1 {\n    export module m1_M1_public {\n        export class c1 {\n        }\n        export function f1() {\n            return new c1;\n        }\n        export var v1 = c1;\n        export var v2: c1;\n    }\n\n    module m1_M2_private {\n        export class c1 {\n        }\n        export function f1() {\n            return new c1;\n        }\n        export var v1 = c1;\n        export var v2: c1;\n    }\n\n    //export declare module \"m1_M3_public\" {\n    //    export function f1();\n    //    export class c1 {\n    //    }\n    //    export var v1: { new (): c1; };\n    //    export var v2: c1;\n    //}\n\n    //declare module \"m1_M4_private\" {\n    //    export function f1();\n    //    export class c1 {\n    //    }\n    //    export var v1: { new (): c1; };\n    //    export var v2: c1;\n    //}\n\n    import m1_im1_private = m1_M1_public;\n    export var m1_im1_private_v1_public = m1_im1_private.c1;\n    export var m1_im1_private_v2_public = new m1_im1_private.c1();\n    export var m1_im1_private_v3_public = m1_im1_private.f1;\n    export var m1_im1_private_v4_public = m1_im1_private.f1();\n    var m1_im1_private_v1_private = m1_im1_private.c1;\n    var m1_im1_private_v2_private = new m1_im1_private.c1();\n    var m1_im1_private_v3_private = m1_im1_private.f1;\n    var m1_im1_private_v4_private = m1_im1_private.f1();\n\n\n    import m1_im2_private = m1_M2_private;\n    export var m1_im2_private_v1_public = m1_im2_private.c1;\n    export var m1_im2_private_v2_public = new m1_im2_private.c1();\n    export var m1_im2_private_v3_public = m1_im2_private.f1;\n    export var m1_im2_private_v4_public = m1_im2_private.f1();\n    var m1_im2_private_v1_private = m1_im2_private.c1;\n    var m1_im2_private_v2_private = new m1_im2_private.c1();\n    var m1_im2_private_v3_private = m1_im2_private.f1;\n    var m1_im2_private_v4_private = m1_im2_private.f1();\n\n    //import m1_im3_private = require(\"m1_M3_public\");\n    //export var m1_im3_private_v1_public = m1_im3_private.c1;\n    //export var m1_im3_private_v2_public = new m1_im3_private.c1();\n    //export var m1_im3_private_v3_public = m1_im3_private.f1;\n    //export var m1_im3_private_v4_public = m1_im3_private.f1();\n    //var m1_im3_private_v1_private = m1_im3_private.c1;\n    //var m1_im3_private_v2_private = new m1_im3_private.c1();\n    //var m1_im3_private_v3_private = m1_im3_private.f1;\n    //var m1_im3_private_v4_private = m1_im3_private.f1();\n\n    //import m1_im4_private = require(\"m1_M4_private\");\n    //export var m1_im4_private_v1_public = m1_im4_private.c1;\n    //export var m1_im4_private_v2_public = new m1_im4_private.c1();\n    //export var m1_im4_private_v3_public = m1_im4_private.f1;\n    //export var m1_im4_private_v4_public = m1_im4_private.f1();\n    //var m1_im4_private_v1_private = m1_im4_private.c1;\n    //var m1_im4_private_v2_private = new m1_im4_private.c1();\n    //var m1_im4_private_v3_private = m1_im4_private.f1;\n    //var m1_im4_private_v4_private = m1_im4_private.f1();\n\n    export import m1_im1_public = m1_M1_public;\n    export import m1_im2_public = m1_M2_private;\n    //export import m1_im3_public = require(\"m1_M3_public\");\n    //export import m1_im4_public = require(\"m1_M4_private\");\n}\n\nmodule glo_M1_public {\n    export class c1 {\n    }\n    export function f1() {\n        return new c1;\n    }\n    export var v1 = c1;\n    export var v2: c1;\n}\n\ndeclare module \"glo_M2_public\" {\n    export function f1();\n    export class c1 {\n    }\n    export var v1: { new (): c1; };\n    export var v2: c1;\n}\n\ndeclare module \"use_glo_M1_public\" {\n    import use_glo_M1_public = glo_M1_public;\n    export var use_glo_M1_public_v1_public: { new (): use_glo_M1_public.c1; };\n    export var use_glo_M1_public_v2_public: typeof use_glo_M1_public;\n    export var use_glo_M1_public_v3_public: ()=> use_glo_M1_public.c1;\n    var use_glo_M1_public_v1_private: { new (): use_glo_M1_public.c1; };\n    var use_glo_M1_public_v2_private: typeof use_glo_M1_public;\n    var use_glo_M1_public_v3_private: () => use_glo_M1_public.c1;\n\n    import use_glo_M2_public = require(\"glo_M2_public\");\n    export var use_glo_M2_public_v1_public: { new (): use_glo_M2_public.c1; };\n    export var use_glo_M2_public_v2_public: typeof use_glo_M2_public;\n    export var use_glo_M2_public_v3_public: () => use_glo_M2_public.c1;\n    var use_glo_M2_public_v1_private: { new (): use_glo_M2_public.c1; };\n    var use_glo_M2_public_v2_private: typeof use_glo_M2_public;\n    var use_glo_M2_public_v3_private: () => use_glo_M2_public.c1;\n\n    module m2 {\n        //import errorImport = require(\"glo_M2_public\");\n        import nonerrorImport = glo_M1_public;\n\n        module m5 {\n            //import m5_errorImport = require(\"glo_M2_public\");\n            import m5_nonerrorImport = glo_M1_public;\n        }\n    }\n}\n\ndeclare module \"anotherParseError\" {\n    module m2 {\n        //declare module \"abc\" {\n        //}\n    }\n\n    module m2 {\n        //module \"abc2\" {\n        //}\n    }\n    //module \"abc3\" {\n    //}\n}\n\nmodule m2 {\n    //import m3 = require(\"use_glo_M1_public\");\n    module m4 {\n        var a = 10;\n        //import m2 = require(\"use_glo_M1_public\");\n    }\n\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "//@declaration: true\nmodule m1 {\n  export module m1_M1_public {\n    export class c1 {}\n    export function f1() {\n      return new c1();\n    }\n    export var v1 = c1;\n    export var v2: c1;\n  }\n\n  module m1_M2_private {\n    export class c1 {}\n    export function f1() {\n      return new c1();\n    }\n    export var v1 = c1;\n    export var v2: c1;\n  }\n\n  //export declare module \"m1_M3_public\" {\n  //    export function f1();\n  //    export class c1 {\n  //    }\n  //    export var v1: { new (): c1; };\n  //    export var v2: c1;\n  //}\n\n  //declare module \"m1_M4_private\" {\n  //    export function f1();\n  //    export class c1 {\n  //    }\n  //    export var v1: { new (): c1; };\n  //    export var v2: c1;\n  //}\n\n  import m1_im1_private = m1_M1_public;\n  export var m1_im1_private_v1_public = m1_im1_private.c1;\n  export var m1_im1_private_v2_public = new m1_im1_private.c1();\n  export var m1_im1_private_v3_public = m1_im1_private.f1;\n  export var m1_im1_private_v4_public = m1_im1_private.f1();\n  var m1_im1_private_v1_private = m1_im1_private.c1;\n  var m1_im1_private_v2_private = new m1_im1_private.c1();\n  var m1_im1_private_v3_private = m1_im1_private.f1;\n  var m1_im1_private_v4_private = m1_im1_private.f1();\n\n  import m1_im2_private = m1_M2_private;\n  export var m1_im2_private_v1_public = m1_im2_private.c1;\n  export var m1_im2_private_v2_public = new m1_im2_private.c1();\n  export var m1_im2_private_v3_public = m1_im2_private.f1;\n  export var m1_im2_private_v4_public = m1_im2_private.f1();\n  var m1_im2_private_v1_private = m1_im2_private.c1;\n  var m1_im2_private_v2_private = new m1_im2_private.c1();\n  var m1_im2_private_v3_private = m1_im2_private.f1;\n  var m1_im2_private_v4_private = m1_im2_private.f1();\n\n  //import m1_im3_private = require(\"m1_M3_public\");\n  //export var m1_im3_private_v1_public = m1_im3_private.c1;\n  //export var m1_im3_private_v2_public = new m1_im3_private.c1();\n  //export var m1_im3_private_v3_public = m1_im3_private.f1;\n  //export var m1_im3_private_v4_public = m1_im3_private.f1();\n  //var m1_im3_private_v1_private = m1_im3_private.c1;\n  //var m1_im3_private_v2_private = new m1_im3_private.c1();\n  //var m1_im3_private_v3_private = m1_im3_private.f1;\n  //var m1_im3_private_v4_private = m1_im3_private.f1();\n\n  //import m1_im4_private = require(\"m1_M4_private\");\n  //export var m1_im4_private_v1_public = m1_im4_private.c1;\n  //export var m1_im4_private_v2_public = new m1_im4_private.c1();\n  //export var m1_im4_private_v3_public = m1_im4_private.f1;\n  //export var m1_im4_private_v4_public = m1_im4_private.f1();\n  //var m1_im4_private_v1_private = m1_im4_private.c1;\n  //var m1_im4_private_v2_private = new m1_im4_private.c1();\n  //var m1_im4_private_v3_private = m1_im4_private.f1;\n  //var m1_im4_private_v4_private = m1_im4_private.f1();\n\n  export import m1_im1_public = m1_M1_public;\n  export import m1_im2_public = m1_M2_private;\n  //export import m1_im3_public = require(\"m1_M3_public\");\n  //export import m1_im4_public = require(\"m1_M4_private\");\n}\n\nmodule glo_M1_public {\n  export class c1 {}\n  export function f1() {\n    return new c1();\n  }\n  export var v1 = c1;\n  export var v2: c1;\n}\n\ndeclare module \"glo_M2_public\" {\n  export function f1();\n  export class c1 {}\n  export var v1: { new (): c1 };\n  export var v2: c1;\n}\n\ndeclare module \"use_glo_M1_public\" {\n  import use_glo_M1_public = glo_M1_public;\n  export var use_glo_M1_public_v1_public: { new (): use_glo_M1_public.c1 };\n  export var use_glo_M1_public_v2_public: typeof use_glo_M1_public;\n  export var use_glo_M1_public_v3_public: () => use_glo_M1_public.c1;\n  var use_glo_M1_public_v1_private: { new (): use_glo_M1_public.c1 };\n  var use_glo_M1_public_v2_private: typeof use_glo_M1_public;\n  var use_glo_M1_public_v3_private: () => use_glo_M1_public.c1;\n\n  import use_glo_M2_public = require(\"glo_M2_public\");\n  export var use_glo_M2_public_v1_public: { new (): use_glo_M2_public.c1 };\n  export var use_glo_M2_public_v2_public: typeof use_glo_M2_public;\n  export var use_glo_M2_public_v3_public: () => use_glo_M2_public.c1;\n  var use_glo_M2_public_v1_private: { new (): use_glo_M2_public.c1 };\n  var use_glo_M2_public_v2_private: typeof use_glo_M2_public;\n  var use_glo_M2_public_v3_private: () => use_glo_M2_public.c1;\n\n  module m2 {\n    //import errorImport = require(\"glo_M2_public\");\n    import nonerrorImport = glo_M1_public;\n\n    module m5 {\n      //import m5_errorImport = require(\"glo_M2_public\");\n      import m5_nonerrorImport = glo_M1_public;\n    }\n  }\n}\n\ndeclare module \"anotherParseError\" {\n  module m2 {\n    //declare module \"abc\" {\n    //}\n  }\n\n  module m2 {\n    //module \"abc2\" {\n    //}\n  }\n  //module \"abc3\" {\n  //}\n}\n\nmodule m2 {\n  //import m3 = require(\"use_glo_M1_public\");\n  module m4 {\n    var a = 10;\n    //import m2 = require(\"use_glo_M1_public\");\n  }\n}");
}
