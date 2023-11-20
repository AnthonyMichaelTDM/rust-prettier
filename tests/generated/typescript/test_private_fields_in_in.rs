#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_ts_format_1_0dd23231() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Person {\n  #name: string;\n  constructor(name: string) {\n    this.#name = name;\n  }\n  \n  equals(other: unknown) {\n    return (\n      other &&\n      typeof other === \"object\" &&\n      #name in other && // <- this is new!\n      this.#name === other.#name\n    );\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Person {\n  #name: string;\n  constructor(name: string) {\n    this.#name = name;\n  }\n\n  equals(other: unknown) {\n    return (\n      other &&\n      typeof other === \"object\" &&\n      #name in other && // <- this is new!\n      this.#name === other.#name\n    );\n  }\n}");
}
