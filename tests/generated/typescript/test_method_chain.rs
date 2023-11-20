#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_ts_format_1_62cca1fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nthis.firebase.object(\\`/shops/\\${shopLocation.shop}\\`)\n  // keep distance info\n  .first((shop: ShopQueryResult, index: number, source: Observable<ShopQueryResult>): any => {\n      // add distance to result\n      const s = shop;\n      s.distance = shopLocation.distance;\n      return s;\n  });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "this.firebase\n  .object(\\`/shops/\\${shopLocation.shop}\\`)\n  // keep distance info\n  .first(\n    (\n      shop: ShopQueryResult,\n      index: number,\n      source: Observable<ShopQueryResult>,\n    ): any => {\n      // add distance to result\n      const s = shop;\n      s.distance = shopLocation.distance;\n      return s;\n    },\n  );");
}
