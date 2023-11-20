#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_optional_chaining_jsx_format_1_07449498() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow", "babel", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n  // https://github.com/marmelab/react-admin/blob/5ae855aa958ba54438b144bf0907b1437c5a5d77/examples/demo/src/orders/Totals.tsx#L38-L43\n  return (\n      <TableCell className={classes.rightAlignedCell}>\n          {record?.delivery_fees.toLocaleString(undefined, {\n              style: 'currency',\n              currency: 'USD',\n          })}\n      </TableCell>\n  )\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {\n  // https://github.com/marmelab/react-admin/blob/5ae855aa958ba54438b144bf0907b1437c5a5d77/examples/demo/src/orders/Totals.tsx#L38-L43\n  return (\n    <TableCell className={classes.rightAlignedCell}>\n      {record?.delivery_fees.toLocaleString(undefined, {\n        style: \"currency\",\n        currency: \"USD\",\n      })}\n    </TableCell>\n  );\n}");
}
