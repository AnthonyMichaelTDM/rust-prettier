#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_in_type_annotations_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_in_type_annotations_js_format_1_6fb41a20() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type F = (x: mixed) /* C1 */ => /* C2 */ x /* C3 */ is /* C4 */ number;\n\nfunction f(x: any) /* C1 */ : /* C2 */ x /* C3 */ is /* C4 */ number /* C5 */ { return true; }\n\nconst arrow = (x: any): /* C1 */ x is (number /* C2 */ => /* C3 */ x /* C4 */ is /* C5 */ string /* C6 */ => /* C7 */ x /* C8 */ is /* C9 */ boolean) => true;") ? ;
    assert_eq ! (formatted , "type F = (x: mixed /* C1 */) => /* C2 */ x /* C3 */ is /* C4 */ number;\n\nfunction f(x: any) /* C1 */ : /* C2 */ x /* C3 */ is /* C4 */ number /* C5 */ {\n  return true;\n}\n\nconst arrow = (\n  x: any,\n): /* C1 */ x is ((\n  number /* C2 */,\n) => /* C3 */ x /* C4 */ is /* C5 */ (\n  string /* C6 */,\n) => /* C7 */ x /* C8 */ is /* C9 */ boolean) => true;");
    Ok(())
}
#[test]
fn test_passing_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_passing_js_format_1_3cc21390() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype F = (x: mixed) => x is A;\n\nfunction f(any): x is A { return true; }\n\nconst arrow0 = (x: any): x is A => true;\nconst arrow1 = (x: any): x is (y: A) => B => true;\nconst arrow3 = (x: any): x is (y: A) => y is B => true;\n\nconst needs_parens_1 = (x: any): x is (A => B) => true;\nconst needs_parens_2 = (x: any): x is ((A) => B) => true;\nconst needs_parens_3 = (x: any): x is (A => x is B) => true;\nconst needs_parens_4 = (x: any): x is (A => x is B => x is C) => true;\n\nconst needs_parens_5 = (x: any): x is (y: A) => (B => C) => true;\nconst needs_parens_6 = (x: any): x is (y: A) => y is (B => C) => true;\n\nclass C {\n  m(): this is D {}\n  f = (): this is D => {}\n}\n\nfunction asserts_1(x: any): asserts x {}\nfunction asserts_2(x: any): asserts x is A {}") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype F = (x: mixed) => x is A;\n\nfunction f(any): x is A {\n  return true;\n}\n\nconst arrow0 = (x: any): x is A => true;\nconst arrow1 = (x: any): x is ((y: A) => B) => true;\nconst arrow3 = (x: any): x is ((y: A) => y is B) => true;\n\nconst needs_parens_1 = (x: any): x is ((A) => B) => true;\nconst needs_parens_2 = (x: any): x is ((A) => B) => true;\nconst needs_parens_3 = (x: any): x is ((A) => x is B) => true;\nconst needs_parens_4 = (x: any): x is ((A) => x is (B) => x is C) => true;\n\nconst needs_parens_5 = (x: any): x is ((y: A) => (B) => C) => true;\nconst needs_parens_6 = (x: any): x is ((y: A) => y is (B) => C) => true;\n\nclass C {\n  m(): this is D {}\n  f = (): this is D => {};\n}\n\nfunction asserts_1(x: any): asserts x {}\nfunction asserts_2(x: any): asserts x is A {}");
    Ok(())
}
