#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_filter_js_format_1_ab7b2024() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Filter the contents of an array\n\ndeclare function my_filter<T, P: $Pred<1>>(v: Array<T>, cb: P): Array<$Refine<T,P,1>>;\n\ndeclare var arr: Array<mixed>;\nconst barr = my_filter(arr, is_string);\n(barr: Array<string>);\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Filter the contents of an array\n\ndeclare function my_filter<T, P: $Pred<1>>(\n  v: Array<T>,\n  cb: P,\n): Array<$Refine<T, P, 1>>;\n\ndeclare var arr: Array<mixed>;\nconst barr = my_filter(arr, is_string);\n(barr: Array<string>);\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}");
}
#[test]
fn test_filter_union_js_format_1_6a127e64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Filter the contents of an array\n\n\ndeclare function my_filter<T, P: $Pred<1>>(v: Array<T>, cb: P): Array<$Refine<T,P,1>>;\n\ntype A = { kind: 'A', u: number }\ntype B = { kind: 'B', v: string }\ntype C = { kind: 'C', y: boolean }\ntype D = { kind: 'D', x: boolean }\ntype E = { kind: 'E', y: boolean }\n\ndeclare var ab: Array<A|B|C>;\n\n(my_filter(ab, (x): %checks => x.kind === 'A'): Array<A>);    // OK\n(my_filter(ab, (x): %checks => x.kind !== 'A'): Array<B|C>);  // OK") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Filter the contents of an array\n\ndeclare function my_filter<T, P: $Pred<1>>(\n  v: Array<T>,\n  cb: P,\n): Array<$Refine<T, P, 1>>;\n\ntype A = { kind: \"A\", u: number };\ntype B = { kind: \"B\", v: string };\ntype C = { kind: \"C\", y: boolean };\ntype D = { kind: \"D\", x: boolean };\ntype E = { kind: \"E\", y: boolean };\n\ndeclare var ab: Array<A | B | C>;\n\n(my_filter(ab, (x): %checks => x.kind === \"A\"): Array<A>); // OK\n(my_filter(ab, (x): %checks => x.kind !== \"A\"): Array<B | C>); // OK");
}
#[test]
fn test_refine_js_format_1_8fcf1530() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n/*\n  $Pred<N> is an \"abstract predicate type\", i.e. denotes a (function) type that\n  refines N variables. So if \\`cb\\` is a function, then it should be refining\n  exactly N argument. It is abstract in that we do not need to specify:\n  (a) which variables are going to be refined (just the number), or (b) what\n  exactly the refinement (predicate) is going to be.\n\n  $Refine<T,P,k> is a refinement type, that refines type T with the k-th\n  argument that gets refined by an abstract preficate type P.\n*/\ndeclare function refine<T, P: $Pred<1>>(v: T, cb: P): $Refine<T,P,1>;\n// function refine(v, cb)\n// { if (cb(v)) { return v; } else { throw new Error(); } }\n\n/*\n  Use case\n*/\ndeclare var a: mixed;\nvar b = refine(a, is_string);\n(b: string);\n\ndeclare function refine_fst<T, P: $Pred<2>>(v: T, w: T, cb: P): $Refine<T,P,1>;\n// function refine_fst(v, w, cb)\n// { if (cb(v, w)) { return v; } else { throw new Error(); } }\n\ndeclare var c: mixed;\ndeclare var d: mixed;\n\nvar e = refine2(c, d, is_string_and_number);\n(e: string);\n\n\ndeclare function refine2<T, P: $Pred<2>>(v: T, w: T, cb: P): $Refine<T,P,1>;\n\n// function refine_fst(v, w, cb)\n// { if (cb(v, w)) { return w; } else { throw new Error(); } }\n\nfunction is_string(x): boolean %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n/*\n  $Pred<N> is an \"abstract predicate type\", i.e. denotes a (function) type that\n  refines N variables. So if \\`cb\\` is a function, then it should be refining\n  exactly N argument. It is abstract in that we do not need to specify:\n  (a) which variables are going to be refined (just the number), or (b) what\n  exactly the refinement (predicate) is going to be.\n\n  $Refine<T,P,k> is a refinement type, that refines type T with the k-th\n  argument that gets refined by an abstract preficate type P.\n*/\ndeclare function refine<T, P: $Pred<1>>(v: T, cb: P): $Refine<T, P, 1>;\n// function refine(v, cb)\n// { if (cb(v)) { return v; } else { throw new Error(); } }\n\n/*\n  Use case\n*/\ndeclare var a: mixed;\nvar b = refine(a, is_string);\n(b: string);\n\ndeclare function refine_fst<T, P: $Pred<2>>(\n  v: T,\n  w: T,\n  cb: P,\n): $Refine<T, P, 1>;\n// function refine_fst(v, w, cb)\n// { if (cb(v, w)) { return v; } else { throw new Error(); } }\n\ndeclare var c: mixed;\ndeclare var d: mixed;\n\nvar e = refine2(c, d, is_string_and_number);\n(e: string);\n\ndeclare function refine2<T, P: $Pred<2>>(v: T, w: T, cb: P): $Refine<T, P, 1>;\n\n// function refine_fst(v, w, cb)\n// { if (cb(v, w)) { return w; } else { throw new Error(); } }\n\nfunction is_string(x): boolean %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}");
}
#[test]
fn test_sanity_filter_js_format_1_e83f1c92() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare function my_filter<T, P: $Pred<1>>(v: Array<T>, cb: P): Array<$Refine<T,P,1>>;\n\n// Sanity check A: filtering the wrong type\ndeclare var a: Array<mixed>;\nconst b = my_filter(a, is_string);\n(b: Array<number>);\n\n\n// Sanity check B: Passing non-predicate function to filter\ndeclare var c: Array<mixed>;\nconst d = my_filter(c, is_string_regular);\n(d: Array<string>);\n\nfunction is_string(x): boolean %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_regular(x): boolean {\n  return typeof x === \"string\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare function my_filter<T, P: $Pred<1>>(\n  v: Array<T>,\n  cb: P,\n): Array<$Refine<T, P, 1>>;\n\n// Sanity check A: filtering the wrong type\ndeclare var a: Array<mixed>;\nconst b = my_filter(a, is_string);\n(b: Array<number>);\n\n// Sanity check B: Passing non-predicate function to filter\ndeclare var c: Array<mixed>;\nconst d = my_filter(c, is_string_regular);\n(d: Array<string>);\n\nfunction is_string(x): boolean %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_regular(x): boolean {\n  return typeof x === \"string\";\n}");
}
#[test]
fn test_sanity_filter_union_js_format_1_71f7d250() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Filter the contents of an array\n\n\ndeclare function my_filter<T, P: $Pred<1>>(v: Array<T>, cb: P): Array<$Refine<T,P,1>>;\n\ntype A = { kind: 'A', u: number }\ntype B = { kind: 'B', v: string }\ntype C = { kind: 'C', y: boolean }\ntype D = { kind: 'D', x: boolean }\ntype E = { kind: 'E', y: boolean }\n\ndeclare var ab: Array<A|B|C>;\n\n(my_filter(ab, (x): %checks => x.kind === 'A'): Array<B>);    // ERROR\n(my_filter(ab, (x): %checks => x.kind !== 'A'): Array<A|C>);  // ERROR") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Filter the contents of an array\n\ndeclare function my_filter<T, P: $Pred<1>>(\n  v: Array<T>,\n  cb: P,\n): Array<$Refine<T, P, 1>>;\n\ntype A = { kind: \"A\", u: number };\ntype B = { kind: \"B\", v: string };\ntype C = { kind: \"C\", y: boolean };\ntype D = { kind: \"D\", x: boolean };\ntype E = { kind: \"E\", y: boolean };\n\ndeclare var ab: Array<A | B | C>;\n\n(my_filter(ab, (x): %checks => x.kind === \"A\"): Array<B>); // ERROR\n(my_filter(ab, (x): %checks => x.kind !== \"A\"): Array<A | C>); // ERROR");
}
#[test]
fn test_sanity_refine_js_format_1_14d2b0ac() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// Sanity check A: the refinment position index is outside of the allowed range\ndeclare function refine<T, P: $Pred<1>>(v: T, cb: P): $Refine<T,P,2>;\n\ndeclare var a: mixed;\nvar b = refine(a, is_string);   // ERROR: index out of bounds\n(b: string);\n\n\n// Sanity check B: refine2 expects a function that accepts 3 arguments but\n// it is called with a function that takes 2\ndeclare var c: mixed;\ndeclare var d: mixed;\ndeclare var e: mixed;\n\ndeclare function refine3<T, P: $Pred<3>>(u: T, v: T, w: T, cb: P): $Refine<T,P,1>;\n\nvar e = refine3(c, d, e, is_string_and_number);\n(e: string);\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}\n\n\n// Sanity check C: expecting a predicate function but passed a non-predicate one\nvar e = refine(a, is_string_regular);   // ERROR: is_string_regular is not a\n                                        // predicate function\n(e: number);\n\n////////////////////////////////////////////////////////////////////////////////\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_regular(x)  {\n  return typeof x === \"string\";\n}\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// Sanity check A: the refinment position index is outside of the allowed range\ndeclare function refine<T, P: $Pred<1>>(v: T, cb: P): $Refine<T, P, 2>;\n\ndeclare var a: mixed;\nvar b = refine(a, is_string); // ERROR: index out of bounds\n(b: string);\n\n// Sanity check B: refine2 expects a function that accepts 3 arguments but\n// it is called with a function that takes 2\ndeclare var c: mixed;\ndeclare var d: mixed;\ndeclare var e: mixed;\n\ndeclare function refine3<T, P: $Pred<3>>(\n  u: T,\n  v: T,\n  w: T,\n  cb: P,\n): $Refine<T, P, 1>;\n\nvar e = refine3(c, d, e, is_string_and_number);\n(e: string);\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}\n\n// Sanity check C: expecting a predicate function but passed a non-predicate one\nvar e = refine(a, is_string_regular); // ERROR: is_string_regular is not a\n// predicate function\n(e: number);\n\n////////////////////////////////////////////////////////////////////////////////\n\nfunction is_string(x): %checks {\n  return typeof x === \"string\";\n}\n\nfunction is_string_regular(x) {\n  return typeof x === \"string\";\n}\n\nfunction is_string_and_number(x, y): %checks {\n  return typeof x === \"string\" && typeof y === \"number\";\n}");
}
