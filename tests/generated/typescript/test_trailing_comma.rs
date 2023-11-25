use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_functions_tsx_trailing_commaall_format_1_e2d9da5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;");
}
#[test]
fn test_arrow_functions_tsx_trailing_commaes_5_format_1_e2d9da5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;");
}
#[test]
fn test_arrow_functions_tsx_trailing_commanone_format_1_e2d9da5e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const f1 = <T,>() => 1;\nconst f2 = <\n  Tttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttttt,\n>() => 1;");
}
#[test]
fn test_trailing_ts_trailing_commaall_format_1_74c56103() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class BaseSingleLevelProfileTargeting<\n\tT extends ValidSingleLevelProfileNode,\n> {\n}\n\nenum Enum {\n\tx = 1,\n\ty = 2,\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest,\n} = something;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class BaseSingleLevelProfileTargeting<\n  T extends ValidSingleLevelProfileNode,\n> {}\n\nenum Enum {\n  x = 1,\n  y = 2,\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest\n} = something;");
}
#[test]
fn test_trailing_ts_trailing_commaes_5_format_1_74c56103() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class BaseSingleLevelProfileTargeting<\n\tT extends ValidSingleLevelProfileNode,\n> {\n}\n\nenum Enum {\n\tx = 1,\n\ty = 2,\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest,\n} = something;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class BaseSingleLevelProfileTargeting<\n  T extends ValidSingleLevelProfileNode,\n> {}\n\nenum Enum {\n  x = 1,\n  y = 2,\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest\n} = something;");
}
#[test]
fn test_trailing_ts_trailing_commanone_format_1_74c56103() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class BaseSingleLevelProfileTargeting<\n\tT extends ValidSingleLevelProfileNode,\n> {\n}\n\nenum Enum {\n\tx = 1,\n\ty = 2,\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest,\n} = something;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class BaseSingleLevelProfileTargeting<\n  T extends ValidSingleLevelProfileNode\n> {}\n\nenum Enum {\n  x = 1,\n  y = 2\n}\n\nconst {\n  longKeySoThisWillGoOnMultipleLines,\n  longKeySoThisWillGoOnMultipleLines2,\n  longKeySoThisWillGoOnMultipleLines3,\n  ...rest\n} = something;");
}
#[test]
fn test_type_arguments_ts_trailing_commaall_format_1_e5d29759() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();");
}
#[test]
fn test_type_arguments_ts_trailing_commaes_5_format_1_e5d29759() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();");
}
#[test]
fn test_type_arguments_ts_trailing_commanone_format_1_e5d29759() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var bar: Bar<\n  AAAAAAA,\n  BBBBBBB,\n  CCCCCCC,\n  DDDDDDD,\n  EEEEEEE,\n  FFFFFFF,\n  GGGGGGG,\n  HHHHHHH\n>;\n\nconst baz = new Array<\n  FooFooFooFooFooFooFooFooFooFooFooFooFooFooFoo,\n  BarBarBarBarBarBarBarBarBarBarBarBarBarBarBar\n>();");
}
#[test]
fn test_type_parameters_vs_arguments_ts_trailing_commaall_format_1_59f2ef08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class FooClass<\n\tA,\n\tB,\n\tC,\n> {\n\ta: A;\n\tb: B;\n\tc: C;\n}\n\nconst instance = new FooClass<\n\tboolean,\n\tnumber,\n\tstring, // [ts] Trailing comma not allowed.\n\t>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class FooClass<A, B, C> {\n  a: A;\n  b: B;\n  c: C;\n}\n\nconst instance = new FooClass<\n  boolean,\n  number,\n  string // [ts] Trailing comma not allowed.\n>();");
}
#[test]
fn test_type_parameters_vs_arguments_ts_trailing_commaes_5_format_1_59f2ef08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class FooClass<\n\tA,\n\tB,\n\tC,\n> {\n\ta: A;\n\tb: B;\n\tc: C;\n}\n\nconst instance = new FooClass<\n\tboolean,\n\tnumber,\n\tstring, // [ts] Trailing comma not allowed.\n\t>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class FooClass<A, B, C> {\n  a: A;\n  b: B;\n  c: C;\n}\n\nconst instance = new FooClass<\n  boolean,\n  number,\n  string // [ts] Trailing comma not allowed.\n>();");
}
#[test]
fn test_type_parameters_vs_arguments_ts_trailing_commanone_format_1_59f2ef08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class FooClass<\n\tA,\n\tB,\n\tC,\n> {\n\ta: A;\n\tb: B;\n\tc: C;\n}\n\nconst instance = new FooClass<\n\tboolean,\n\tnumber,\n\tstring, // [ts] Trailing comma not allowed.\n\t>();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class FooClass<A, B, C> {\n  a: A;\n  b: B;\n  c: C;\n}\n\nconst instance = new FooClass<\n  boolean,\n  number,\n  string // [ts] Trailing comma not allowed.\n>();");
}
