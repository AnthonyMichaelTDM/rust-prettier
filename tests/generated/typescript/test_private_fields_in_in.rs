#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_ts_format_1_0dd23231() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Person {\n  #name: string;\n  constructor(name: string) {\n    this.#name = name;\n  }\n  \n  equals(other: unknown) {\n    return (\n      other &&\n      typeof other === \"object\" &&\n      #name in other && // <- this is new!\n      this.#name === other.#name\n    );\n  }\n}") ? ;
    assert_eq ! (formatted , "class Person {\n  #name: string;\n  constructor(name: string) {\n    this.#name = name;\n  }\n\n  equals(other: unknown) {\n    return (\n      other &&\n      typeof other === \"object\" &&\n      #name in other && // <- this is new!\n      this.#name === other.#name\n    );\n  }\n}");
    Ok(())
}
