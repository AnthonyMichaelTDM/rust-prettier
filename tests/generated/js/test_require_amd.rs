#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_named_amd_module_js_format_1_da62af10() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("define(\"foo/title\",\n    [\"my/cart\", \"my/inventory\"],\n    function(cart, inventory) {\n        //Define foo/title object in here.\n   }\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "define(\"foo/title\", [\"my/cart\", \"my/inventory\"], function (cart, inventory) {\n  //Define foo/title object in here.\n});");
}
#[test]
fn test_non_amd_define_js_format_1_f5e8b9d6() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const someVariable = define(\n  \"some string literal\",\n  anotherVariable,\n  yetAnotherVariable\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const someVariable = define(\n  \"some string literal\",\n  anotherVariable,\n  yetAnotherVariable,\n);");
}
#[test]
fn test_require_js_format_1_68305155() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("require(\n    [\n        'jquery',\n        'common/global.context',\n        'common/log.event',\n        'some_project/square',\n        'some_project/rectangle',\n        'some_project/triangle',\n        'some_project/circle',\n        'some_project/star',\n    ],\n    function($, Context, EventLogger, Square, Rectangle, Triangle, Circle, Star) {\n\n        console.log('some code')\n    }\n);\n\ndefine(\n    [\n        'jquery',\n        'common/global.context',\n        'common/log.event',\n        'some_project/square',\n        'some_project/rectangle',\n        'some_project/triangle',\n        'some_project/circle',\n        'some_project/star',\n    ],\n    function($, Context, EventLogger, Square, Rectangle, Triangle, Circle, Star) {\n\n        console.log('some code')\n    }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "require([\n  \"jquery\",\n  \"common/global.context\",\n  \"common/log.event\",\n  \"some_project/square\",\n  \"some_project/rectangle\",\n  \"some_project/triangle\",\n  \"some_project/circle\",\n  \"some_project/star\",\n], function (\n  $,\n  Context,\n  EventLogger,\n  Square,\n  Rectangle,\n  Triangle,\n  Circle,\n  Star,\n) {\n  console.log(\"some code\");\n});\n\ndefine([\n  \"jquery\",\n  \"common/global.context\",\n  \"common/log.event\",\n  \"some_project/square\",\n  \"some_project/rectangle\",\n  \"some_project/triangle\",\n  \"some_project/circle\",\n  \"some_project/star\",\n], function (\n  $,\n  Context,\n  EventLogger,\n  Square,\n  Rectangle,\n  Triangle,\n  Circle,\n  Star,\n) {\n  console.log(\"some code\");\n});");
}
