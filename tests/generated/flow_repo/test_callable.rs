#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_js_format_1_080f7895() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type F = {\n  (x: string): number;\n  p?: string;\n}\n\nfunction f(x) {\n  return x.length;\n}\n\n(f: F);") ? ;
    assert_eq ! (formatted , "type F = {\n  (x: string): number,\n  p?: string,\n};\n\nfunction f(x) {\n  return x.length;\n}\n\n(f: F);");
    Ok(())
}
#[test]
fn test_primitives_js_format_1_88101847() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x = Boolean(4);\nfunction foo(fn:(value:any)=>boolean) { }\nfoo(Boolean);\n\nvar dict: { [k: string]: any } = {};\ndict(); // error, callable signature not found\n\ninterface ICall {\n  (x: string): void;\n}\ndeclare var icall: ICall;\nicall(0); // error, number ~> string\nicall.call(null, 0); // error, number ~> string\n\ntype Callable = {\n  (x: string): void;\n}\n\ndeclare var callable: Callable;\ncallable(0); // error, number ~> string\ncallable.call(null, 0); // error, number ~> string") ? ;
    assert_eq ! (formatted , "var x = Boolean(4);\nfunction foo(fn: (value: any) => boolean) {}\nfoo(Boolean);\n\nvar dict: { [k: string]: any } = {};\ndict(); // error, callable signature not found\n\ninterface ICall {\n  (x: string): void;\n}\ndeclare var icall: ICall;\nicall(0); // error, number ~> string\nicall.call(null, 0); // error, number ~> string\n\ntype Callable = {\n  (x: string): void,\n};\n\ndeclare var callable: Callable;\ncallable(0); // error, number ~> string\ncallable.call(null, 0); // error, number ~> string");
    Ok(())
}
