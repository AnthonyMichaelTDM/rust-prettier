#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_generic_component_tsx_format_1_8be6b91f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const c1 = <MyComponent<number> data={12} />\n\nconst c2 = <MyComponent<number> />\n\nconst c3 = <MyComponent<number> attr=\"value\" />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const c1 = <MyComponent<number> data={12} />;\n\nconst c2 = <MyComponent<number> />;\n\nconst c3 = <MyComponent<number> attr=\"value\" />;");
}
#[test]
fn test_keyword_tsx_format_1_1f24a857() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<try />;\n<object />");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<try />;\n<object />;");
}
#[test]
fn test_member_expression_tsx_format_1_f5df4608() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(<a />).method();\n(<a />).property;\n(<a />)[\"computed\"];\n(<a />)[\"computed\"]();\n(\n  <div>\n    <a>foo</a>\n  </div>\n).method();\n(\n  <div>\n    <a>foo</a>\n  </div>\n).property;\n(\n  <div>\n    <a>foo</a>\n  </div>\n)[\"computed\"];\n(\n  <div>\n    <a>foo</a>\n  </div>\n)[\"computed\"]();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(<a />).method();\n(<a />).property;\n(<a />)[\"computed\"];\n(<a />)[\"computed\"]();\n(\n  <div>\n    <a>foo</a>\n  </div>\n).method();\n(\n  <div>\n    <a>foo</a>\n  </div>\n).property;\n(\n  <div>\n    <a>foo</a>\n  </div>\n)[\"computed\"];\n(\n  <div>\n    <a>foo</a>\n  </div>\n)[\"computed\"]();");
}
#[test]
fn test_not_react_ts_format_1_45f4b7ec() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/// <reference types=\"node\" />\n\ntype ul = any;\nconst somethingElse = 1;\nconst thing = <ul>somethingElse;\nconst div = \"<div></div>\";\nconst h1 = \\`<h1>Hi</h1>\\`;\nconst footer = '<footer></footer>';") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/// <reference types=\"node\" />\n\ntype ul = any;\nconst somethingElse = 1;\nconst thing = <ul>somethingElse;\nconst div = \"<div></div>\";\nconst h1 = \\`<h1>Hi</h1>\\`;\nconst footer = \"<footer></footer>\";");
}
#[test]
fn test_react_tsx_format_1_75a18212() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const MyCoolList = ({ things }) =>\n    <ul>\n        {things.map(MyCoolThing)}\n    </ul>;\n\nconst MyCoolThing = ({ thingo }) => <li>{thingo}</li>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const MyCoolList = ({ things }) => <ul>{things.map(MyCoolThing)}</ul>;\n\nconst MyCoolThing = ({ thingo }) => <li>{thingo}</li>;");
}
#[test]
fn test_this_tsx_format_1_9a0444ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<this.state.Component />;\n<this.Component />;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<this.state.Component />;\n<this.Component />;");
}
#[test]
fn test_type_parameters_tsx_format_1_39feef19() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const functionName1 = <T,>(arg) => false;\nconst functionName2 = <T extends any>(arg) => false;\nconst functionName3 = <T, S>(arg) => false;\n\nfunction functionName4<T>() {\n  return false;\n}\n\nfunctionName5<T>();\n\ninterface Interface1<T> {\n  one: \"one\";\n}\n\ninterface Interface2 {\n  two: Two<T>;\n}\n\ntype Type1<T> = \"type1\";\n\ntype Type2 = Two<T>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const functionName1 = <T,>(arg) => false;\nconst functionName2 = <T extends any>(arg) => false;\nconst functionName3 = <T, S>(arg) => false;\n\nfunction functionName4<T>() {\n  return false;\n}\n\nfunctionName5<T>();\n\ninterface Interface1<T> {\n  one: \"one\";\n}\n\ninterface Interface2 {\n  two: Two<T>;\n}\n\ntype Type1<T> = \"type1\";\n\ntype Type2 = Two<T>;");
}
#[test]
fn test_url_tsx_format_1_4ac89751() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const link = <a href=\"example.com\">http://example.com</a>;\n\nconst first = <div>http://example.com</div>;\n\n const second = <>http://example.com</>;\n\n const third = <div><br />http://example.com</div>;\n\n const fourth = <div><span></span>http://example.com</div>;\n\n const fifth = <div>{}http://example.com</div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const link = <a href=\"example.com\">http://example.com</a>;\n\nconst first = <div>http://example.com</div>;\n\nconst second = <>http://example.com</>;\n\nconst third = (\n  <div>\n    <br />\n    http://example.com\n  </div>\n);\n\nconst fourth = (\n  <div>\n    <span></span>http://example.com\n  </div>\n);\n\nconst fifth = <div>{}http://example.com</div>;");
}
