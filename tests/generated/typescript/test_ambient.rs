#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ambient_ts_format_1_129082fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare module \"classnames\" {\n  export default function classnames(...inputs: (string | number | false | object | undefined)[]): string;\n  export class x {}\n  export interface y {}\n  export type z = y;\n}\n\ndeclare module \"x\" {\n  export default class x {}\n}\n\ndeclare module \"y\" {\n  const y = 5;\n  export default y;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare module \"classnames\" {\n  export default function classnames(\n    ...inputs: (string | number | false | object | undefined)[]\n  ): string;\n  export class x {}\n  export interface y {}\n  export type z = y;\n}\n\ndeclare module \"x\" {\n  export default class x {}\n}\n\ndeclare module \"y\" {\n  const y = 5;\n  export default y;\n}");
}
