#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_efe8b11e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar geolocation = new Geolocation();\nvar id = geolocation.watchPosition(\n  position => {\n    var coords: Coordinates = position.coords;\n    var accuracy: number = coords.accuracy;\n  },\n  e => {\n    var message: string = e.message;\n    switch (e.code) {\n      case e.PERMISSION_DENIED:\n      case e.POSITION_UNAVAILABLE:\n      case e.TIMEOUT:\n      default:\n        break;\n    }\n  }\n);\ngeolocation.clearWatch(id);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar geolocation = new Geolocation();\nvar id = geolocation.watchPosition(\n  (position) => {\n    var coords: Coordinates = position.coords;\n    var accuracy: number = coords.accuracy;\n  },\n  (e) => {\n    var message: string = e.message;\n    switch (e.code) {\n      case e.PERMISSION_DENIED:\n      case e.POSITION_UNAVAILABLE:\n      case e.TIMEOUT:\n      default:\n        break;\n    }\n  },\n);\ngeolocation.clearWatch(id);");
}
