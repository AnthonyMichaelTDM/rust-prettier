#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ambient_ts_format_1_129082fd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module \"classnames\" {\n  export default function classnames(...inputs: (string | number | false | object | undefined)[]): string;\n  export class x {}\n  export interface y {}\n  export type z = y;\n}\n\ndeclare module \"x\" {\n  export default class x {}\n}\n\ndeclare module \"y\" {\n  const y = 5;\n  export default y;\n}") ? ;
    assert_eq ! (formatted , "declare module \"classnames\" {\n  export default function classnames(\n    ...inputs: (string | number | false | object | undefined)[]\n  ): string;\n  export class x {}\n  export interface y {}\n  export type z = y;\n}\n\ndeclare module \"x\" {\n  export default class x {}\n}\n\ndeclare module \"y\" {\n  const y = 5;\n  export default y;\n}");
    Ok(())
}
