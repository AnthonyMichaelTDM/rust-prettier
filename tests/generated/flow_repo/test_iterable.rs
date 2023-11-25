#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_d6ca3f0e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(([1, 2]: Array<number>): Iterable<number>);\n([1,2,\"hi\"]: Iterable<number | string>);\n([1,2,3]: Iterable<*>);\n\n([\"hi\"]: Iterable<number>); // Error string ~> number\n([\"hi\", 1]: Iterable<string>); // Error number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(([1, 2]: Array<number>): Iterable<number>);\n([1, 2, \"hi\"]: Iterable<number | string>);\n([1, 2, 3]: Iterable<*>);\n\n([\"hi\"]: Iterable<number>); // Error string ~> number\n([\"hi\", 1]: Iterable<string>); // Error number ~> string");
}
#[test]
fn test_caching_bug_js_format_1_5fac34f3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n/**\n * I've hit a bug with the caching in flow_js.ml. Avik is removing that caching\n * so it should be fixed soon. The basic idea is I flow something like\n *\n * Array<any | any> ~> Iterable<string>\n *\n * then Flow won't notice when I try to flow\n *\n * Array<string | number> ~> Iterable<string>\n *\n * We shouldn't hit the cache because the union types are different, but we do\n * anyway. I've fixed this temporarily by bumping the \"meaningful\" param to\n * Hashtbl.hash_param\n */\n\nfunction fill_the_cache(x: Array<any | any>): Iterable<string> { return x; }\n\n// Error: number ~> string\nfunction miss_the_cache(x: Array<string | number>): Iterable<string> { return x; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n/**\n * I've hit a bug with the caching in flow_js.ml. Avik is removing that caching\n * so it should be fixed soon. The basic idea is I flow something like\n *\n * Array<any | any> ~> Iterable<string>\n *\n * then Flow won't notice when I try to flow\n *\n * Array<string | number> ~> Iterable<string>\n *\n * We shouldn't hit the cache because the union types are different, but we do\n * anyway. I've fixed this temporarily by bumping the \"meaningful\" param to\n * Hashtbl.hash_param\n */\n\nfunction fill_the_cache(x: Array<any | any>): Iterable<string> {\n  return x;\n}\n\n// Error: number ~> string\nfunction miss_the_cache(x: Array<string | number>): Iterable<string> {\n  return x;\n}");
}
#[test]
fn test_iter_js_format_1_850e8b2e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(strs: Iterable<string>): void {\n  for (var s: string of strs) {\n    console.log(s);\n  }\n}\n\nvar m: Map<string, number> = new Map();\n\nfoo(m.keys());") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(strs: Iterable<string>): void {\n  for (var s: string of strs) {\n    console.log(s);\n  }\n}\n\nvar m: Map<string, number> = new Map();\n\nfoo(m.keys());");
}
#[test]
fn test_iterator_result_js_format_1_0b769ef4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction makeIterator(coin_flip: () => boolean ): Iterator<string> {\n  return {\n    \"@@iterator\"() { return makeIterator(coin_flip); },\n    next(): IteratorResult<string, void> {\n      var done = coin_flip();\n      if (!done) {\n        return { done, value: \"still going...\" };\n      } else {\n        return { done };\n      }\n    }\n  }\n}\n\nfunction makeIterator(coin_flip: () => boolean ): Iterator<string> {\n  return {\n    \"@@iterator\"() { return makeIterator(coin_flip); },\n    next(): IteratorResult<string, void> {\n      var done = coin_flip();\n      if (done) { // Whoops, made a mistake and forgot to negate done\n        return { done, value: \"still going...\" }; // Error string ~> void\n      } else {\n        return { done }; // Error void ~> string\n      }\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction makeIterator(coin_flip: () => boolean): Iterator<string> {\n  return {\n    \"@@iterator\"() {\n      return makeIterator(coin_flip);\n    },\n    next(): IteratorResult<string, void> {\n      var done = coin_flip();\n      if (!done) {\n        return { done, value: \"still going...\" };\n      } else {\n        return { done };\n      }\n    },\n  };\n}\n\nfunction makeIterator(coin_flip: () => boolean): Iterator<string> {\n  return {\n    \"@@iterator\"() {\n      return makeIterator(coin_flip);\n    },\n    next(): IteratorResult<string, void> {\n      var done = coin_flip();\n      if (done) {\n        // Whoops, made a mistake and forgot to negate done\n        return { done, value: \"still going...\" }; // Error string ~> void\n      } else {\n        return { done }; // Error void ~> string\n      }\n    },\n  };\n}");
}
#[test]
fn test_map_js_format_1_ab10a8b7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction mapTest1(map: Map<string, number>): Iterable<[string, number]> {\n  return map;\n}\nfunction mapTest2<K, V>(map: Map<K, V>): Iterable<[K, V]> {\n  return map;\n};\nfunction mapTest3(map: Map<string, number>): Iterable<*> {\n  return map;\n}\n// Error - Map is an Iterable<[K, V]>\nfunction mapTest4(map: Map<number, string>): Iterable<string> {\n  return map;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction mapTest1(map: Map<string, number>): Iterable<[string, number]> {\n  return map;\n}\nfunction mapTest2<K, V>(map: Map<K, V>): Iterable<[K, V]> {\n  return map;\n}\nfunction mapTest3(map: Map<string, number>): Iterable<*> {\n  return map;\n}\n// Error - Map is an Iterable<[K, V]>\nfunction mapTest4(map: Map<number, string>): Iterable<string> {\n  return map;\n}");
}
#[test]
fn test_set_js_format_1_688e0770() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction setTest1(set: Set<string>): Iterable<string> {\n  return set;\n}\nfunction setTest2<T>(set: Set<T>): Iterable<T> {\n  return set;\n};\nfunction setTest3(set: Set<string>): Iterable<*> {\n  return set;\n}\n// Error string ~> number\nfunction setTest4(set: Set<string>): Iterable<number> {\n  return set;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction setTest1(set: Set<string>): Iterable<string> {\n  return set;\n}\nfunction setTest2<T>(set: Set<T>): Iterable<T> {\n  return set;\n}\nfunction setTest3(set: Set<string>): Iterable<*> {\n  return set;\n}\n// Error string ~> number\nfunction setTest4(set: Set<string>): Iterable<number> {\n  return set;\n}");
}
#[test]
fn test_string_js_format_1_6af84612() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(\"hi\": Iterable<string>);\n(\"hi\": Iterable<*>);\n(\"hi\": Iterable<number>); // Error - string is a Iterable<string>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(\"hi\": Iterable<string>);\n(\"hi\": Iterable<*>);\n(\"hi\": Iterable<number>); // Error - string is a Iterable<string>");
}
#[test]
fn test_variance_js_format_1_4ab0af30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n(([]: Array<string>): Iterable<?string>); // ok, Iterable<+T>\n\n(([]: Array<string>).values(): Iterable<?string>); // ok, Iterator<+T>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n(([]: Array<string>): Iterable<?string>); // ok, Iterable<+T>\n\n(([]: Array<string>).values(): Iterable<?string>); // ok, Iterator<+T>");
}
