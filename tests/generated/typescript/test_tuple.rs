#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dangling_comments_ts_trailing_commaall_format_1_9a1de994() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo1 = [/* comment */];\n\ntype Foo2 = [\n    // comment\n];\n\ntype Foo3 = [\n    // comment1\n    // comment2\n];\n\ntype Foo4 = [\n    // comment1\n\n    // comment2\n];\n\ntype Foo5 = [\n    /* comment1 */\n];\n\ntype Foo6 = [\n    /* comment1 */\n\n    /* comment2 */\n];\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo1 = [\n  /* comment */\n];\n\ntype Foo2 = [\n  // comment\n];\n\ntype Foo3 = [\n  // comment1\n  // comment2\n];\n\ntype Foo4 = [\n  // comment1\n  // comment2\n];\n\ntype Foo5 = [\n  /* comment1 */\n];\n\ntype Foo6 = [\n  /* comment1 */\n  /* comment2 */\n];");
}
#[test]
fn test_dangling_comments_ts_trailing_commaes_5_format_1_9a1de994() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo1 = [/* comment */];\n\ntype Foo2 = [\n    // comment\n];\n\ntype Foo3 = [\n    // comment1\n    // comment2\n];\n\ntype Foo4 = [\n    // comment1\n\n    // comment2\n];\n\ntype Foo5 = [\n    /* comment1 */\n];\n\ntype Foo6 = [\n    /* comment1 */\n\n    /* comment2 */\n];\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo1 = [\n  /* comment */\n];\n\ntype Foo2 = [\n  // comment\n];\n\ntype Foo3 = [\n  // comment1\n  // comment2\n];\n\ntype Foo4 = [\n  // comment1\n  // comment2\n];\n\ntype Foo5 = [\n  /* comment1 */\n];\n\ntype Foo6 = [\n  /* comment1 */\n  /* comment2 */\n];");
}
#[test]
fn test_dangling_comments_ts_trailing_commanone_format_1_9a1de994() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo1 = [/* comment */];\n\ntype Foo2 = [\n    // comment\n];\n\ntype Foo3 = [\n    // comment1\n    // comment2\n];\n\ntype Foo4 = [\n    // comment1\n\n    // comment2\n];\n\ntype Foo5 = [\n    /* comment1 */\n];\n\ntype Foo6 = [\n    /* comment1 */\n\n    /* comment2 */\n];\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo1 = [\n  /* comment */\n];\n\ntype Foo2 = [\n  // comment\n];\n\ntype Foo3 = [\n  // comment1\n  // comment2\n];\n\ntype Foo4 = [\n  // comment1\n  // comment2\n];\n\ntype Foo5 = [\n  /* comment1 */\n];\n\ntype Foo6 = [\n  /* comment1 */\n  /* comment2 */\n];");
}
#[test]
fn test_trailing_comma_ts_trailing_commaall_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string;\n      to: string;\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
#[test]
fn test_trailing_comma_ts_trailing_commaes_5_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string;\n      to: string;\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
#[test]
fn test_trailing_comma_ts_trailing_commanone_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string;\n      to: string;\n    } // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
#[test]
fn test_trailing_comma_for_empty_tuples_ts_trailing_commaall_format_1_6e94d97e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong = []\n\ntype Foo = Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends [] ? Foo3 : Foo4") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong =\n  [];\n\ntype Foo =\n  Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends []\n    ? Foo3\n    : Foo4;");
}
#[test]
fn test_trailing_comma_for_empty_tuples_ts_trailing_commaes_5_format_1_6e94d97e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong = []\n\ntype Foo = Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends [] ? Foo3 : Foo4") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong =\n  [];\n\ntype Foo =\n  Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends []\n    ? Foo3\n    : Foo4;");
}
#[test]
fn test_trailing_comma_for_empty_tuples_ts_trailing_commanone_format_1_6e94d97e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong = []\n\ntype Foo = Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends [] ? Foo3 : Foo4") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong =\n  [];\n\ntype Foo =\n  Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends []\n    ? Foo3\n    : Foo4;");
}
#[test]
fn test_trailing_comma_trailing_rest_ts_trailing_commaall_format_1_ee5bdb6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type ValidateArgs = [\n\t{\n\t\t[key: string]: any;\n\t},\n\tstring,\n\tstring,\n\t...string[],\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type ValidateArgs = [\n  {\n    [key: string]: any;\n  },\n  string,\n  string,\n  ...string[],\n];");
}
#[test]
fn test_trailing_comma_trailing_rest_ts_trailing_commaes_5_format_1_ee5bdb6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type ValidateArgs = [\n\t{\n\t\t[key: string]: any;\n\t},\n\tstring,\n\tstring,\n\t...string[],\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type ValidateArgs = [\n  {\n    [key: string]: any;\n  },\n  string,\n  string,\n  ...string[],\n];");
}
#[test]
fn test_trailing_comma_trailing_rest_ts_trailing_commanone_format_1_ee5bdb6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type ValidateArgs = [\n\t{\n\t\t[key: string]: any;\n\t},\n\tstring,\n\tstring,\n\t...string[],\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type ValidateArgs = [\n  {\n    [key: string]: any;\n  },\n  string,\n  string,\n  ...string[]\n];");
}
#[test]
fn test_tuple_ts_trailing_commaall_format_1_ad52557e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nexport type SCMRawResource = [\n\tnumber /*handle*/,\n\tstring /*resourceUri*/,\n\tmodes.Command /*command*/,\n\tstring[] /*icons: light, dark*/,\n\tboolean /*strike through*/,\n\tboolean /*faded*/\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type SCMRawResource = [\n  number /*handle*/,\n  string /*resourceUri*/,\n  modes.Command /*command*/,\n  string[] /*icons: light, dark*/,\n  boolean /*strike through*/,\n  boolean /*faded*/,\n];");
}
#[test]
fn test_tuple_ts_trailing_commaes_5_format_1_ad52557e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nexport type SCMRawResource = [\n\tnumber /*handle*/,\n\tstring /*resourceUri*/,\n\tmodes.Command /*command*/,\n\tstring[] /*icons: light, dark*/,\n\tboolean /*strike through*/,\n\tboolean /*faded*/\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type SCMRawResource = [\n  number /*handle*/,\n  string /*resourceUri*/,\n  modes.Command /*command*/,\n  string[] /*icons: light, dark*/,\n  boolean /*strike through*/,\n  boolean /*faded*/,\n];");
}
#[test]
fn test_tuple_ts_trailing_commanone_format_1_ad52557e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nexport type SCMRawResource = [\n\tnumber /*handle*/,\n\tstring /*resourceUri*/,\n\tmodes.Command /*command*/,\n\tstring[] /*icons: light, dark*/,\n\tboolean /*strike through*/,\n\tboolean /*faded*/\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type SCMRawResource = [\n  number /*handle*/,\n  string /*resourceUri*/,\n  modes.Command /*command*/,\n  string[] /*icons: light, dark*/,\n  boolean /*strike through*/,\n  boolean /*faded*/\n];");
}
#[test]
fn test_tuple_labeled_ts_trailing_commaall_format_1_2bf48e61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]]\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]];\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];");
}
#[test]
fn test_tuple_labeled_ts_trailing_commaes_5_format_1_2bf48e61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]]\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]];\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];");
}
#[test]
fn test_tuple_labeled_ts_trailing_commanone_format_1_2bf48e61() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]]\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11754\n\ntype T = [x: A, y?: B, ...z: C];\n\ntype T = [A, y: B];\n\nlet x: [A: string, ...B: number[]];\n\ntype T = [foo: string, bar?: number];\n\ntype T = [x?: A, y: B];\n\ntype T = [x: A, ...B];\n\ntype T = [...B, x: A];");
}
#[test]
fn test_tuple_rest_not_last_ts_trailing_commaall_format_1_bc8b4f39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string]",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string];"
    );
}
#[test]
fn test_tuple_rest_not_last_ts_trailing_commaes_5_format_1_bc8b4f39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string]",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string];"
    );
}
#[test]
fn test_tuple_rest_not_last_ts_trailing_commanone_format_1_bc8b4f39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string]",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// https://github.com/babel/babel/pull/11753\n\nlet x: [...[number, string], string];"
    );
}
