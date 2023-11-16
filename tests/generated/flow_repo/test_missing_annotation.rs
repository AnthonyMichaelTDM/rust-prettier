#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_array_js_format_1_14210ab0() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\ntype Foo = {};\nvar f: Foo = {};\nexport var arr = [f];");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\ntype Foo = {};\nvar f: Foo = {};\nexport var arr = [f];"
    );
}
#[test]
fn test_async_return_js_format_1_06aac082() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nexport async function foo() {\n  return 123;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nexport async function foo() {\n  return 123;\n}"
    );
}
#[test]
fn test_infer_js_format_1_738bc51e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar Foo = {\n  a: function(arg) {  // missing arg annotation\n    return arg;\n  },\n\n  b: function(arg) {  // missing arg annotation\n    return {\n      bar: arg\n    };\n  },\n\n  c: function(arg: string) {  // no return annotation required\n    return {\n      bar: arg\n    };\n  },\n\n  d: function(arg: string): {\n    bar: string\n  } {\n    return {\n      bar: arg\n    };\n  },\n\n  // return type annotation may be omitted, but if so, input positions on\n  // observed return type (e.g. param types in a function type) must come\n  // from annotations\n  e: function(arg: string) {\n    return function(x) {  // missing param annotation\n      return x;\n    }\n  },\n\n  // ...if the return type is annotated explicitly, this is unnecessary\n  f: function(arg: string): (x:number) => number {\n    return function(x) {  // no error\n      return x;\n    }\n  }\n\n};\n\nvar Bar = {\n  a: Foo.a('Foo'),    // no annotation required\n\n  // object property types are inferred, so make sure that this doesn't cause\n  // us to also infer the parameter's type.\n  b: Foo.b('bar'),    // no annotation required\n\n  c: Foo.c('bar'),            // no annotation required\n\n  d: Foo.d('bar'),            // no annotation required\n};\n\nmodule.exports = Foo, Bar;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar Foo = {\n  a: function (arg) {\n    // missing arg annotation\n    return arg;\n  },\n\n  b: function (arg) {\n    // missing arg annotation\n    return {\n      bar: arg,\n    };\n  },\n\n  c: function (arg: string) {\n    // no return annotation required\n    return {\n      bar: arg,\n    };\n  },\n\n  d: function (arg: string): {\n    bar: string,\n  } {\n    return {\n      bar: arg,\n    };\n  },\n\n  // return type annotation may be omitted, but if so, input positions on\n  // observed return type (e.g. param types in a function type) must come\n  // from annotations\n  e: function (arg: string) {\n    return function (x) {\n      // missing param annotation\n      return x;\n    };\n  },\n\n  // ...if the return type is annotated explicitly, this is unnecessary\n  f: function (arg: string): (x: number) => number {\n    return function (x) {\n      // no error\n      return x;\n    };\n  },\n};\n\nvar Bar = {\n  a: Foo.a(\"Foo\"), // no annotation required\n\n  // object property types are inferred, so make sure that this doesn't cause\n  // us to also infer the parameter's type.\n  b: Foo.b(\"bar\"), // no annotation required\n\n  c: Foo.c(\"bar\"), // no annotation required\n\n  d: Foo.d(\"bar\"), // no annotation required\n};\n\n(module.exports = Foo), Bar;");
}
