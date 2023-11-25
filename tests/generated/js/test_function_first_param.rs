use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_function_expression_js_format_1_0c13f1e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//https://github.com/prettier/prettier/issues/3002\nbeep.boop().baz(\"foo\",\n{\n  some: {\n    thing: {\n      nested: true\n    }\n  }\n},\n{ another: { thing: true } },\n() => {});\n\n\n//https://github.com/prettier/prettier/issues/2984\ndb.collection('indexOptionDefault').createIndex({ a: 1 }, {\n  indexOptionDefaults: true,\n  w: 2,\n  wtimeout: 1000\n}, function(err) {\n  test.equal(null, err);\n  test.deepEqual({ w: 2, wtimeout: 1000 }, commandResult.writeConcern);\n\n  client.close();\n  done();\n})") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "//https://github.com/prettier/prettier/issues/3002\nbeep.boop().baz(\n  \"foo\",\n  {\n    some: {\n      thing: {\n        nested: true,\n      },\n    },\n  },\n  { another: { thing: true } },\n  () => {},\n);\n\n//https://github.com/prettier/prettier/issues/2984\ndb.collection(\"indexOptionDefault\").createIndex(\n  { a: 1 },\n  {\n    indexOptionDefaults: true,\n    w: 2,\n    wtimeout: 1000,\n  },\n  function (err) {\n    test.equal(null, err);\n    test.deepEqual({ w: 2, wtimeout: 1000 }, commandResult.writeConcern);\n\n    client.close();\n    done();\n  },\n);");
}
