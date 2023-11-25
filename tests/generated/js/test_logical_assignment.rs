#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_logical_assignment_js_format_1_0a95194a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a ||= b;\n\na &&= \"foo\";\nb ||= \"foo\";\nc ??= \"foo\";\n\nd &&= 42;\ne ||= 42;\nf ??= 42;\n\na.baz &&= result.baz;\nb.baz ||= result.baz;\nc.baz ??= result.baz;\n\na.foo[\"baz\"] &&= result.foo.baz;\nb.foo[\"baz\"] ||= result.foo.baz;\nc.foo[\"baz\"] ??= result.foo.baz;\n\na.foo.bar().baz &&= result.foo.bar().baz;\nb.foo.bar().baz ||= result.foo.bar().baz;\nb.baz ||= result.baz;\nc.baz ??= result.baz;\n\n(results ||= []).push(100);\n(results &&= []).push(100);\n(results ??= []).push(100);\n\nif ((thing &&= thing.original)) {\n}\nif ((thing &&= defaultValue)) {\n}\nif ((thing ||= defaultValue)) {\n}\nif ((thing ??= defaultValue)) {\n}\n\nf ||= (a) => a;\nf &&= (a) => a;\nf ??= (a) => a;\n\nf ||= (f.toString(), (a) => a);\nf &&= (f.toString(), (a) => a);\nf ??= (f.toString(), (a) => a);\n\n(results ||= results1 ||= []).push(100);\n(results &&= results1 &&= []).push(100);\n(results ??= results1 ??= []).push(100);\n\nobj[incr()] ||= incr();\noobj[\"obj\"][incr()] ||= incr();\nobj[incr()] &&= incr();\noobj[\"obj\"][incr()] &&= incr();\nobj[incr()] ??= incr();\noobj[\"obj\"][incr()] ??= incr();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a ||= b;\n\na &&= \"foo\";\nb ||= \"foo\";\nc ??= \"foo\";\n\nd &&= 42;\ne ||= 42;\nf ??= 42;\n\na.baz &&= result.baz;\nb.baz ||= result.baz;\nc.baz ??= result.baz;\n\na.foo[\"baz\"] &&= result.foo.baz;\nb.foo[\"baz\"] ||= result.foo.baz;\nc.foo[\"baz\"] ??= result.foo.baz;\n\na.foo.bar().baz &&= result.foo.bar().baz;\nb.foo.bar().baz ||= result.foo.bar().baz;\nb.baz ||= result.baz;\nc.baz ??= result.baz;\n\n(results ||= []).push(100);\n(results &&= []).push(100);\n(results ??= []).push(100);\n\nif ((thing &&= thing.original)) {\n}\nif ((thing &&= defaultValue)) {\n}\nif ((thing ||= defaultValue)) {\n}\nif ((thing ??= defaultValue)) {\n}\n\nf ||= (a) => a;\nf &&= (a) => a;\nf ??= (a) => a;\n\nf ||= (f.toString(), (a) => a);\nf &&= (f.toString(), (a) => a);\nf ??= (f.toString(), (a) => a);\n\n(results ||= results1 ||= []).push(100);\n(results &&= results1 &&= []).push(100);\n(results ??= results1 ??= []).push(100);\n\nobj[incr()] ||= incr();\noobj[\"obj\"][incr()] ||= incr();\nobj[incr()] &&= incr();\noobj[\"obj\"][incr()] &&= incr();\nobj[incr()] ??= incr();\noobj[\"obj\"][incr()] ??= incr();");
}
