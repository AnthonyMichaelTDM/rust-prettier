#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_in_object_type_js_format_1_18e67eb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const example1 = (): ({ p: string => string }) => (0: any);\nconst example2 = (): ({ p: { p: string => string } }) => (0: any);\nconst example3 = (): ({ p: { p: { p: string => string } } }) => (0: any);\nconst example4 = (): ({ p: { p: ?{ p: string => string } } }) => (0: any);\nconst example5 = (): ({ p: { p: { p: string => string } | string } }) =>\n  (0: any);\nconst example6 = (): ({ p: { p: { p: string => string } & string } }) =>\n  (0: any);\nconst example7 = (): ({ p: { p: { p: [(string) => string, string] } } }) =>\n  (0: any);\nfunction example8(): { p: string => string } {\n  return (0: any);\n}\nfunction example9(): { p: { p: string => string } } {\n  return (0: any);\n}\nfunction example10(): { p: { p: { p: string => string } } } {\n  return (0: any);\n}\nconst example11 = (): ({ p: string => string } & string) => (0: any);\nconst example12 = (): ({ p: string => string } | string) => (0: any);\nconst example13 = (): ([{ p: string => string }, string]) => (0: any);\nconst example14 = (): ({ p: string => string }[]) => (0: any);\nconst example15 = (): ({ p: { p: { p: (string => string) & string } } }) =>\n  (0: any);\nconst example16 = (): ({ p: { p: { p: (string => string) | string } } }) =>\n  (0: any);\nconst example17 = (): (?{ p: string => string }) => (0: any);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const example1 = (): ({ p: (string) => string }) => (0: any);\nconst example2 = (): ({ p: { p: (string) => string } }) => (0: any);\nconst example3 = (): ({ p: { p: { p: (string) => string } } }) => (0: any);\nconst example4 = (): ({ p: { p: ?{ p: (string) => string } } }) => (0: any);\nconst example5 = (): ({ p: { p: { p: (string) => string } | string } }) =>\n  (0: any);\nconst example6 = (): ({ p: { p: { p: (string) => string } & string } }) =>\n  (0: any);\nconst example7 = (): ({ p: { p: { p: [(string) => string, string] } } }) =>\n  (0: any);\nfunction example8(): { p: (string) => string } {\n  return (0: any);\n}\nfunction example9(): { p: { p: (string) => string } } {\n  return (0: any);\n}\nfunction example10(): { p: { p: { p: (string) => string } } } {\n  return (0: any);\n}\nconst example11 = (): ({ p: (string) => string } & string) => (0: any);\nconst example12 = (): ({ p: (string) => string } | string) => (0: any);\nconst example13 = (): ([{ p: (string) => string }, string]) => (0: any);\nconst example14 = (): ({ p: (string) => string }[]) => (0: any);\nconst example15 = (): ({ p: { p: { p: ((string) => string) & string } } }) =>\n  (0: any);\nconst example16 = (): ({ p: { p: { p: ((string) => string) | string } } }) =>\n  (0: any);\nconst example17 = (): (?{ p: (string) => string }) => (0: any);");
}
#[test]
fn test_issue_1249_js_format_1_5d7f1c46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Bar = ( number | string ) => number;\ntype X = (?(number, number) => number) => void;\ntype X = ?((number, number) => number) => void;\ntype X = ?(number, number) => number => void;\ntype X = 1234 => void;\ntype X = 'abc' => void;\ntype X = true => void;\ntype X = false => void;\ntype X = boolean => void;\ntype X = number => void;\ntype X = void => void;\ntype X = null => void;\ntype X = any => void;\ntype X = empty => void;\ntype X = mixed => void;\ntype X = string => void;\ntype X = abc => void;\ntype X = a | b => void;\ntype X = (a | b) => void;\ntype X = a & b => void;\ntype X = (a & b) => void;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Bar = (number | string) => number;\ntype X = (?(number, number) => number) => void;\ntype X = ?((number, number) => number) => void;\ntype X = ?(number, number) => (number) => void;\ntype X = (1234) => void;\ntype X = (\"abc\") => void;\ntype X = (true) => void;\ntype X = (false) => void;\ntype X = (boolean) => void;\ntype X = (number) => void;\ntype X = (void) => void;\ntype X = (null) => void;\ntype X = (any) => void;\ntype X = (empty) => void;\ntype X = (mixed) => void;\ntype X = (string) => void;\ntype X = (abc) => void;\ntype X = a | ((b) => void);\ntype X = (a | b) => void;\ntype X = a & ((b) => void);\ntype X = (a & b) => void;");
}
#[test]
fn test_parens_js_format_1_5f8fd6b0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f1 = (): (string => string) => {};\nconst f2 = (): ?(y => {a: b => c}) => (0: any);\nconst f3 = (): (a | string => string) => {};\nconst f4 = (): (a & string => string) => {};\nfunction f5(): string => string {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f1 = (): ((string) => string) => {};\nconst f2 = (): (?(y) => { a: (b) => c }) => (0: any);\nconst f3 = (): a | ((string) => string) => {};\nconst f4 = (): a & ((string) => string) => {};\nfunction f5(): (string) => string {}");
}
