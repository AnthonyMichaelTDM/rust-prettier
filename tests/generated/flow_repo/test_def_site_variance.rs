#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_7af4b09f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Variance<+Out,-In> {\n  foo(x: Out): Out { return x; }\n  bar(y: In): In { return y; }\n}\n\nclass A { }\nclass B extends A { }\n\nfunction subtyping(\n  v1: Variance<A,B>,\n  v2: Variance<B,A>\n) {\n  (v1: Variance<B,A>); // error on both targs (A ~/~> B)\n  (v2: Variance<A,B>); // OK for both targs (B ~> A)\n}\n\nclass PropVariance<+Out,-In> {\n  inv1: Out; // error\n  inv2: In; // error\n  -co1: Out; // error\n  -co2: In; // ok\n  +con1: Out; // ok\n  +con2: In; // error\n\n  inv_dict1: {[k:string]: Out}; // error\n  inv_dict2: {[k:string]: In}; // error\n  co_dict1: {+[k:string]: Out}; // ok\n  co_dict2: {+[k:string]: In}; // error\n  con_dict1: {-[k:string]: Out}; // error\n  con_dict2: {-[k:string]: In}; // ok\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Variance<+Out, -In> {\n  foo(x: Out): Out {\n    return x;\n  }\n  bar(y: In): In {\n    return y;\n  }\n}\n\nclass A {}\nclass B extends A {}\n\nfunction subtyping(v1: Variance<A, B>, v2: Variance<B, A>) {\n  (v1: Variance<B, A>); // error on both targs (A ~/~> B)\n  (v2: Variance<A, B>); // OK for both targs (B ~> A)\n}\n\nclass PropVariance<+Out, -In> {\n  inv1: Out; // error\n  inv2: In; // error\n  -co1: Out; // error\n  -co2: In; // ok\n  +con1: Out; // ok\n  +con2: In; // error\n\n  inv_dict1: { [k: string]: Out }; // error\n  inv_dict2: { [k: string]: In }; // error\n  co_dict1: { +[k: string]: Out }; // ok\n  co_dict2: { +[k: string]: In }; // error\n  con_dict1: { -[k: string]: Out }; // error\n  con_dict2: { -[k: string]: In }; // ok\n}");
}
