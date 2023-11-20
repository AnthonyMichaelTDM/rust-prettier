#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_ts_format_1_600cbf3c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class ViewTokensChangedEvent {\n  public readonly ranges: {\n    /**\n     * Start line number of range\n     */\n    readonly fromLineNumber: number;\n  }[];\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class ViewTokensChangedEvent {\n  public readonly ranges: {\n    /**\n     * Start line number of range\n     */\n    readonly fromLineNumber: number;\n  }[];\n}");
}
#[test]
fn test_key_ts_format_1_baaf2ef5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const subtractDuration = moment.duration(\n  subtractMap[interval][0],\n  subtractMap[interval][1] as unitOfTime.DurationConstructor\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const subtractDuration = moment.duration(\n  subtractMap[interval][0],\n  subtractMap[interval][1] as unitOfTime.DurationConstructor,\n);");
}
