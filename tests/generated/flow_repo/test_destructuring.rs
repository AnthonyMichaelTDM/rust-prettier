#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_rest_js_format_1_3a412e87() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let xs = [0, \"\", true];\nlet [a, ...ys] = xs;\nlet [b, ...zs] = ys;\nlet c = zs[0]; // retain tuple info\nlet d = zs[1]; // run off the end\n\n(a: void); // error: number ~> void\n(b: void); // error: string ~> void\n(c: void); // error: boolean ~> void\n(d: void); // error: number|string|boolean ~> void\n\nlet [...e] = 0;") ? ;
    assert_eq ! (formatted , "let xs = [0, \"\", true];\nlet [a, ...ys] = xs;\nlet [b, ...zs] = ys;\nlet c = zs[0]; // retain tuple info\nlet d = zs[1]; // run off the end\n\n(a: void); // error: number ~> void\n(b: void); // error: string ~> void\n(c: void); // error: boolean ~> void\n(d: void); // error: number|string|boolean ~> void\n\nlet [...e] = 0;");
    Ok(())
}
#[test]
fn test_computed_js_format_1_52f26732() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var { [\"key\"]: val1 } = { key: \"val\" };\n(val1: void); // error: string ~> void\n\nvar key: string = \"key\";\nvar { [key]: val2 } = { key: \"val\" };\n(val2: void); // ok (gasp!) by existing StrT -> ElemT rule\n\nvar { [\"key\"]: val3, ...spread } = { key: \"val\" };\n(spread.key: void); // error (gasp!) in general we don't know if a computed prop should be excluded from spread") ? ;
    assert_eq ! (formatted , "var { [\"key\"]: val1 } = { key: \"val\" };\n(val1: void); // error: string ~> void\n\nvar key: string = \"key\";\nvar { [key]: val2 } = { key: \"val\" };\n(val2: void); // ok (gasp!) by existing StrT -> ElemT rule\n\nvar { [\"key\"]: val3, ...spread } = { key: \"val\" };\n(spread.key: void); // error (gasp!) in general we don't know if a computed prop should be excluded from spread");
    Ok(())
}
#[test]
fn test_defaults_js_format_1_fe5ce215() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction obj_prop_fun({p:{q=0}={q:true}}={p:{q:\"\"}}) {\n  // errors:\n  // * number  ~> void, from default on _.p.q\n  // * boolean ~> void, from default on _.p\n  // * string  ~> void, from default on _\n  // * null    ~> void, from call below\n  (q:void);\n}\nobj_prop_fun(); // ok\nobj_prop_fun({}); // ok\nobj_prop_fun({p:{}}); // ok\nobj_prop_fun({p:{q:null}}); // ok, provides add'l lower bound\n\nfunction obj_prop_var(o={p:{q:\"\"}}) {\n  var {p:{q=0}={q:true}} = o;\n  // errors:\n  // * number  ~> void, from default on o.p.q\n  // * boolean ~> void, from default on o.p\n  // * string  ~> void, from default on o\n  // * null    ~> void, from call below\n  (q:void);\n}\nobj_prop_var(); // ok\nobj_prop_var({}); // ok\nobj_prop_var({p:{}}); // ok\nobj_prop_var({p:{q:null}}); // ok, provides add'l lower bound\n\nfunction obj_rest({p:{q,...o}={q:0,r:0}}={p:{q:0,r:\"\"}}) {\n  // errors:\n  // * number  ~> void, from default on _.p\n  // * string  ~> void, from default on _\n  // * null    ~> void, from call below\n  (o.r:void);\n}\nobj_rest(); // ok\nobj_rest({}); // ok\nobj_rest({p:{}}); // ok\nobj_rest({p:{q:0,r:null}});\n\nfunction obj_prop_annot({\n  p = true // error: boolean ~> string\n}: {\n  p: string\n} = {\n  p: 0 // error: number ~> string\n}) {\n  (p:void); // error: string ~> void\n}\n\nvar {\n  p = true // error: boolean ~> string\n}: {\n  p: string\n} = {\n  p: 0 // error: number ~> string\n};\n(p:void); // error: string ~> void\n\nfunction obj_prop_err({x:{y}}=null) {} // error: property `x` cannot be accessed on null\nfunction obj_rest_err({...o}=0) {} // error: expected object instead of number\nfunction arr_elem_err([x]=null) {} // error: element 0 cannot be accessed on null\nfunction arr_rest_err([...a]=null) {} // error: expected array instead of null\n\nfunction gen<T>(x:T,{p=x}:{p:T}):T {\n  return p;\n}\n\n// Default values in destructuring unwrap optional types\nobj_prop_fun(({} : {p?:{q?:null}})); // ok\nobj_prop_var(({} : {p?:{q?:null}})); // ok\n\n// union-like upper bounds preserved through destructuring\nfunction obj_prop_opt({p}:{p?:string}={p:0}) {}\nfunction obj_prop_maybe({p}:{p:?string}={p:0}) {}\nfunction obj_prop_union({p}:{p:number|string}={p:true}) {}\n\n// TODO: union-of-objects upper bounds preserved through destructuring\nfunction obj_prop_union2({p}:{p:number}|{p:string}={p:true}) {}\n\nfunction default_expr_scope({a, b = a}) {}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction obj_prop_fun({ p: { q = 0 } = { q: true } } = { p: { q: \"\" } }) {\n  // errors:\n  // * number  ~> void, from default on _.p.q\n  // * boolean ~> void, from default on _.p\n  // * string  ~> void, from default on _\n  // * null    ~> void, from call below\n  (q: void);\n}\nobj_prop_fun(); // ok\nobj_prop_fun({}); // ok\nobj_prop_fun({ p: {} }); // ok\nobj_prop_fun({ p: { q: null } }); // ok, provides add'l lower bound\n\nfunction obj_prop_var(o = { p: { q: \"\" } }) {\n  var { p: { q = 0 } = { q: true } } = o;\n  // errors:\n  // * number  ~> void, from default on o.p.q\n  // * boolean ~> void, from default on o.p\n  // * string  ~> void, from default on o\n  // * null    ~> void, from call below\n  (q: void);\n}\nobj_prop_var(); // ok\nobj_prop_var({}); // ok\nobj_prop_var({ p: {} }); // ok\nobj_prop_var({ p: { q: null } }); // ok, provides add'l lower bound\n\nfunction obj_rest(\n  { p: { q, ...o } = { q: 0, r: 0 } } = { p: { q: 0, r: \"\" } },\n) {\n  // errors:\n  // * number  ~> void, from default on _.p\n  // * string  ~> void, from default on _\n  // * null    ~> void, from call below\n  (o.r: void);\n}\nobj_rest(); // ok\nobj_rest({}); // ok\nobj_rest({ p: {} }); // ok\nobj_rest({ p: { q: 0, r: null } });\n\nfunction obj_prop_annot(\n  {\n    p = true, // error: boolean ~> string\n  }: {\n    p: string,\n  } = {\n    p: 0, // error: number ~> string\n  },\n) {\n  (p: void); // error: string ~> void\n}\n\nvar {\n  p = true, // error: boolean ~> string\n}: {\n  p: string,\n} = {\n  p: 0, // error: number ~> string\n};\n(p: void); // error: string ~> void\n\nfunction obj_prop_err({ x: { y } } = null) {} // error: property `x` cannot be accessed on null\nfunction obj_rest_err({ ...o } = 0) {} // error: expected object instead of number\nfunction arr_elem_err([x] = null) {} // error: element 0 cannot be accessed on null\nfunction arr_rest_err([...a] = null) {} // error: expected array instead of null\n\nfunction gen<T>(x: T, { p = x }: { p: T }): T {\n  return p;\n}\n\n// Default values in destructuring unwrap optional types\nobj_prop_fun(({}: { p?: { q?: null } })); // ok\nobj_prop_var(({}: { p?: { q?: null } })); // ok\n\n// union-like upper bounds preserved through destructuring\nfunction obj_prop_opt({ p }: { p?: string } = { p: 0 }) {}\nfunction obj_prop_maybe({ p }: { p: ?string } = { p: 0 }) {}\nfunction obj_prop_union({ p }: { p: number | string } = { p: true }) {}\n\n// TODO: union-of-objects upper bounds preserved through destructuring\nfunction obj_prop_union2({ p }: { p: number } | { p: string } = { p: true }) {}\n\nfunction default_expr_scope({ a, b = a }) {}");
    Ok(())
}
#[test]
fn test_destructuring_js_format_1_e71f84c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var a:string;\ndeclare var b:string;\ndeclare var c:string;\n[{a1:a, b},c] = [{a1:0, b:1},2];\n\nvar {m} = {m:0};\n({m} = {m:m});\n\nvar obj;\n({n: obj.x} = {n:3});\n[obj.x] = ['foo'];\n\nfunction foo({p, z:[r]}) {\n    a = p;\n    b = z;\n    c = r;\n}\nfoo({p:0, z:[1,2]});\n\n[a,,b,...c] = [0,1,true,3];\n\nfunction bar({x, ...z}) {\n    var o:{x: string; y: number;} = z;\n}\nbar({x:\"\",y:0});\n\nvar spread = {y:\"\"};\nvar extend: {x:number; y:string; z: boolean} = {x:0, ...spread};\n\nfunction qux(_: {a:number}) { }\nqux({a:\"\"});\nfunction corge({b}: {b:string}) { }\ncorge({b:0});\n\nvar {n}:{n: number} = {n: \"\"}\n\nfunction test() {\n  var {foo} = {bar: 123}; // error on foo\n  var {bar, baz} = {bar: 123} // error on baz\n}\n\nfunction test() {\n  var x = {foo: 'abc', bar: 123};\n  var {foo, ...rest} = x;\n  (x.baz: string); // error, baz doesn't exist\n  (rest.baz: string); // no error, rest is unsealed\n}\n\nmodule.exports = corge;\n\nclass Base {\n  baseprop1: number;\n  baseprop2: number;\n}\n\nclass Child extends Base {\n  childprop1: number;\n  childprop2: number;\n}\n\nvar {baseprop1, childprop1, ...others} = new Child();\n\nvar bp1: number = baseprop1;\nvar bp1_err: string = baseprop1; // Error: number ~> string\nvar bp2: number = others.baseprop2;\nvar bp2_err: string = others.baseprop2; // Error: number ~> string\n\nvar cp1: number = childprop1;\nvar cp1_err: string = childprop1; // Error: number ~> string\nvar cp2: number = others.childprop1;\nvar cp2_err: string = others.childprop2; // Error: number ~> string") ? ;
    assert_eq ! (formatted , "declare var a: string;\ndeclare var b: string;\ndeclare var c: string;\n[{ a1: a, b }, c] = [{ a1: 0, b: 1 }, 2];\n\nvar { m } = { m: 0 };\n({ m } = { m: m });\n\nvar obj;\n({ n: obj.x } = { n: 3 });\n[obj.x] = [\"foo\"];\n\nfunction foo({ p, z: [r] }) {\n  a = p;\n  b = z;\n  c = r;\n}\nfoo({ p: 0, z: [1, 2] });\n\n[a, , b, ...c] = [0, 1, true, 3];\n\nfunction bar({ x, ...z }) {\n  var o: { x: string, y: number } = z;\n}\nbar({ x: \"\", y: 0 });\n\nvar spread = { y: \"\" };\nvar extend: { x: number, y: string, z: boolean } = { x: 0, ...spread };\n\nfunction qux(_: { a: number }) {}\nqux({ a: \"\" });\nfunction corge({ b }: { b: string }) {}\ncorge({ b: 0 });\n\nvar { n }: { n: number } = { n: \"\" };\n\nfunction test() {\n  var { foo } = { bar: 123 }; // error on foo\n  var { bar, baz } = { bar: 123 }; // error on baz\n}\n\nfunction test() {\n  var x = { foo: \"abc\", bar: 123 };\n  var { foo, ...rest } = x;\n  (x.baz: string); // error, baz doesn't exist\n  (rest.baz: string); // no error, rest is unsealed\n}\n\nmodule.exports = corge;\n\nclass Base {\n  baseprop1: number;\n  baseprop2: number;\n}\n\nclass Child extends Base {\n  childprop1: number;\n  childprop2: number;\n}\n\nvar { baseprop1, childprop1, ...others } = new Child();\n\nvar bp1: number = baseprop1;\nvar bp1_err: string = baseprop1; // Error: number ~> string\nvar bp2: number = others.baseprop2;\nvar bp2_err: string = others.baseprop2; // Error: number ~> string\n\nvar cp1: number = childprop1;\nvar cp1_err: string = childprop1; // Error: number ~> string\nvar cp2: number = others.childprop1;\nvar cp2_err: string = others.childprop2; // Error: number ~> string");
    Ok(())
}
#[test]
fn test_eager_js_format_1_ce235397() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("var x;\n({x} = null); // error, property `x` can not be accessed on `null`")?;
    assert_eq!(
        formatted,
        "var x;\n({ x } = null); // error, property `x` can not be accessed on `null`"
    );
    Ok(())
}
#[test]
fn test_poly_js_format_1_15ebfa05() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction obj_pattern<X>({ prop } : { prop: X }) {} // prop: X\ntype Prop<X> = { prop: X };\nfunction obj_pattern2<X>({ prop } : Prop<X>) {} // prop: X\n\nfunction arr_pattern<X>([ elem ] : X[]) {} // elem: X\ntype Elem<X> = X[];\nfunction arr_pattern2<X>([ elem ] : Elem<X>) {} // elem: X\n\nfunction tup_pattern<X>([ proj ] : [X]) {} // proj: X\ntype Proj<X> = [X];\nfunction tup_pattern2<X>([ proj ] : Proj<X>) {} // proj: X\n\nfunction rest_pattern<X>(...r: X[]) {} // r: X[]\n\nfunction obj_rest_pattern<X>({ _, ...o } : { _: any, x: X }) { // o: { x: X }\n  o.x;\n}\ntype ObjRest<X> = { _: any, x: X };\nfunction obj_rest_pattern<X>({ _, ...o } : ObjRest<X>) { // o: { x: X }\n  o.x;\n}\n\nfunction arr_rest_pattern<X>([ _, ...a ] : [ any, X ]) { // a: [X]\n  a[0];\n}\ntype ArrRest<X> = [ any, X ];\nfunction arr_rest_pattern<X>([ _, ...a ] : ArrRest<X>) { // a: [X]\n  a[0];\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nfunction obj_pattern<X>({ prop }: { prop: X }) {} // prop: X\ntype Prop<X> = { prop: X };\nfunction obj_pattern2<X>({ prop }: Prop<X>) {} // prop: X\n\nfunction arr_pattern<X>([elem]: X[]) {} // elem: X\ntype Elem<X> = X[];\nfunction arr_pattern2<X>([elem]: Elem<X>) {} // elem: X\n\nfunction tup_pattern<X>([proj]: [X]) {} // proj: X\ntype Proj<X> = [X];\nfunction tup_pattern2<X>([proj]: Proj<X>) {} // proj: X\n\nfunction rest_pattern<X>(...r: X[]) {} // r: X[]\n\nfunction obj_rest_pattern<X>({ _, ...o }: { _: any, x: X }) {\n  // o: { x: X }\n  o.x;\n}\ntype ObjRest<X> = { _: any, x: X };\nfunction obj_rest_pattern<X>({ _, ...o }: ObjRest<X>) {\n  // o: { x: X }\n  o.x;\n}\n\nfunction arr_rest_pattern<X>([_, ...a]: [any, X]) {\n  // a: [X]\n  a[0];\n}\ntype ArrRest<X> = [any, X];\nfunction arr_rest_pattern<X>([_, ...a]: ArrRest<X>) {\n  // a: [X]\n  a[0];\n}");
    Ok(())
}
#[test]
fn test_rec_js_format_1_6065e12f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Make sure that destructuring doesn't cause infinite loops when combined with\n// funny doses of repositioning\n\nlet foo = (i: number) => [i];\n\nconst bar = (i: number) => {\n  [i] = foo(i);\n  return [i];\n};\n\nfoo = (i: number) => {\n  return bar(i);\n};\n\n// Also make sure that the following doesn't loop\n\ndeclare var o: empty;\nvar { x: o } = o;\n\n// this also must not loop\n\ndeclare var _x:  {};\n\nlet x = _x;\n\nfunction baz () {\n    const {...y} = x;\n\n    x = y;\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\n// Make sure that destructuring doesn't cause infinite loops when combined with\n// funny doses of repositioning\n\nlet foo = (i: number) => [i];\n\nconst bar = (i: number) => {\n  [i] = foo(i);\n  return [i];\n};\n\nfoo = (i: number) => {\n  return bar(i);\n};\n\n// Also make sure that the following doesn't loop\n\ndeclare var o: empty;\nvar { x: o } = o;\n\n// this also must not loop\n\ndeclare var _x: {};\n\nlet x = _x;\n\nfunction baz() {\n  const { ...y } = x;\n\n  x = y;\n}");
    Ok(())
}
#[test]
fn test_refinement_non_termination_js_format_1_0bb91a44() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction _([argArray]: Array<Value>) {\n  if (argArray instanceof NullValue || argArray instanceof UndefinedValue) {\n  }\n};\n\nclass Value { }\nclass NullValue extends Value { }\nclass UndefinedValue extends Value { }") ? ;
    assert_eq ! (formatted , "// @flow\n\nfunction _([argArray]: Array<Value>) {\n  if (argArray instanceof NullValue || argArray instanceof UndefinedValue) {\n  }\n}\n\nclass Value {}\nclass NullValue extends Value {}\nclass UndefinedValue extends Value {}");
    Ok(())
}
#[test]
fn test_string_lit_js_format_1_71654ef5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var { \"key\": val } = { key: \"val\" };\n(val: void); // error: string ~> void\n\nvar { \"with-dash\": with_dash } = { \"with-dash\": \"motivating example\" };\n(with_dash: \"motivating example\"); // ok") ? ;
    assert_eq ! (formatted , "var { key: val } = { key: \"val\" };\n(val: void); // error: string ~> void\n\nvar { \"with-dash\": with_dash } = { \"with-dash\": \"motivating example\" };\n(with_dash: \"motivating example\"); // ok");
    Ok(())
}
#[test]
fn test_unannotated_js_format_1_b0dbd25f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// @flow\n\nvar { x } = {\n  x: { foo: \"foo\" }\n};\n\nfunction bar() {\n  x.bar\n}",
    )?;
    assert_eq!(
        formatted,
        "// @flow\n\nvar { x } = {\n  x: { foo: \"foo\" },\n};\n\nfunction bar() {\n  x.bar;\n}"
    );
    Ok(())
}
