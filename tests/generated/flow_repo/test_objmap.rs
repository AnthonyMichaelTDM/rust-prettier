#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_objmap_js_format_1_279b3133() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function promiseAllByKey<O>(o: O): Promise<$ObjMap<O,typeof $await>>;\ndeclare function keyMirror<O>(o: O): $ObjMapi<O, <K>(k:K) => K>;\n\nvar o = keyMirror({\n  FOO: null,\n  BAR: null,\n});\n\n(o.FOO : 'FOO'); // ok\n(o.FOO : 'BAR'); // error, 'FOO' incompatible with 'BAR'\n\npromiseAllByKey({\n  foo: Promise.resolve(0),\n  bar: 'bar',\n}).then(o => {\n  (o.foo: string); // error, number ~> string\n  (o.bar: 'bar'); // ok\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function promiseAllByKey<O>(o: O): Promise<$ObjMap<O, typeof $await>>;\ndeclare function keyMirror<O>(o: O): $ObjMapi<O, <K>(k: K) => K>;\n\nvar o = keyMirror({\n  FOO: null,\n  BAR: null,\n});\n\n(o.FOO: \"FOO\"); // ok\n(o.FOO: \"BAR\"); // error, 'FOO' incompatible with 'BAR'\n\npromiseAllByKey({\n  foo: Promise.resolve(0),\n  bar: \"bar\",\n}).then((o) => {\n  (o.foo: string); // error, number ~> string\n  (o.bar: \"bar\"); // ok\n});");
}
