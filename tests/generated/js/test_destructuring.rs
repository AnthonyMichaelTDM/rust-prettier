#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_destructuring_js_format_1_351f740c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const [one, two = null, three = null] = arr;\na = ([s=1,]) => 1\nconst { children, ...props } = this.props\n\nconst { user: { firstName, lastName } } = this.props;\n\nconst {\n  name: { first, last },\n  organisation: { address: { street: orgStreetAddress, postcode: orgPostcode } }\n} = user;\n\nfunction f({ data: { name } }) {}\n\nconst UserComponent = function({\n  name: { first, last },\n  organisation: { address: { street: orgStreetAddress, postcode: orgPostcode } },\n}) {\n  return\n};\n\nconst { a, b, c, d: { e } } = someObject;\n\ntry {\n  // code\n} catch ({ data: { message }}) {\n  // code\n}\n\ntry {\n  // code\n} catch ({ data: { message: { errors }}}) {\n  // code\n}\n\nconst obj = {\n  func(id, { blog: { title } }) {\n    return id + title;\n  },\n};\n\nclass A {\n  func(id, { blog: { title } }) {\n    return id + title;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const [one, two = null, three = null] = arr;\na = ([s = 1]) => 1;\nconst { children, ...props } = this.props;\n\nconst {\n  user: { firstName, lastName },\n} = this.props;\n\nconst {\n  name: { first, last },\n  organisation: {\n    address: { street: orgStreetAddress, postcode: orgPostcode },\n  },\n} = user;\n\nfunction f({ data: { name } }) {}\n\nconst UserComponent = function ({\n  name: { first, last },\n  organisation: {\n    address: { street: orgStreetAddress, postcode: orgPostcode },\n  },\n}) {\n  return;\n};\n\nconst {\n  a,\n  b,\n  c,\n  d: { e },\n} = someObject;\n\ntry {\n  // code\n} catch ({ data: { message } }) {\n  // code\n}\n\ntry {\n  // code\n} catch ({\n  data: {\n    message: { errors },\n  },\n}) {\n  // code\n}\n\nconst obj = {\n  func(id, { blog: { title } }) {\n    return id + title;\n  },\n};\n\nclass A {\n  func(id, { blog: { title } }) {\n    return id + title;\n  }\n}");
}
#[test]
fn test_issue_5988_js_format_1_e89a8a16() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const { foo, bar: bazAndSomething, quxIsLong } = someBigFunctionName(\"foo\")(\"bar\");",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const {\n  foo,\n  bar: bazAndSomething,\n  quxIsLong,\n} = someBigFunctionName(\"foo\")(\"bar\");");
}
