#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_invalid_vue_format_1_4df85fe5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<template>\n<div\n  v-for=\"  item instanceof  items \"\n></div>\n</template>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<template>\n  <div v-for=\"  item instanceof  items \"></div>\n</template>"
    );
}
#[test]
fn test_ts_vue_format_1_90911f05() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["vue"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"ts\"></script>\n<template>\n<div\nv-for=\"a:number       of x as number[]\"\nv-for=\"[ a  , b]   : [   string,string ]     of x as Array<     [string, string]>\"\nv-for=\"  a  of list.map( (x:any): unknown     => x.foo.bar)\"\nv-for=\"([longLongProp, longLongProp, [longLongProp, longLongProp='Hello, Prettier!', [longLongProp, longLongProp, anotherLongLongProp=[longLongProp, longLongProp, anotherLongLongProp, yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], yetAnotherLongLongProp], index) of longLongLongLongLongLongLongLongList\"\n>\n</div>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script lang=\"ts\"></script>\n<template>\n  <div\n    v-for=\"a: number of x as number[]\"\n    v-for=\"[a, b]: [string, string] of x as Array<[string, string]>\"\n    v-for=\"a of list.map((x: any): unknown => x.foo.bar)\"\n    v-for=\"(\n      [\n        longLongProp,\n        longLongProp,\n        [\n          longLongProp,\n          longLongProp = 'Hello, Prettier!',\n          [\n            longLongProp,\n            longLongProp,\n            anotherLongLongProp = [\n              longLongProp,\n              longLongProp,\n              anotherLongLongProp,\n              yetAnotherLongLongProp,\n            ],\n            yetAnotherLongLongProp,\n          ],\n          yetAnotherLongLongProp,\n        ],\n        yetAnotherLongLongProp,\n      ],\n      index\n    ) of longLongLongLongLongLongLongLongList\"\n  ></div>\n</template>");
}
