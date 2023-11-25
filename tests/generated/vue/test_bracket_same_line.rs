use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_vue_html_bracket_same_linefalse_format_1_76db7dc4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<template>\n  <div id=\"js-app\">\n<div :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<span :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<img :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"/>\n<div class=\"a\">\ntext\n</div>\n<span  class=\"a\">\ntext\n</span>\n<img class=\"a\"/>\n  </div>\n</template>\n\n<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">new Vue({el: '#js-app'})</script>\n<style  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">.a {color: #f00}</style>\n</body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <template>\n      <div id=\"js-app\">\n        <div\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \"\n        >\n          text\n        </div>\n        <div\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \"\n        ></div>\n        <span\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \"\n        >\n          text\n        </span>\n        <span\n          :long_long_attribute=\"\n            long_long_long_long_long_long_long_long_long_long_long_value\n          \"\n        ></span>\n        <img\n          :long_long_attribute=\"\n            long_long_long_long_long_long_long_long_long_long_long_value\n          \"\n        />\n        <div class=\"a\">text</div>\n        <span class=\"a\"> text </span>\n        <img class=\"a\" />\n      </div>\n    </template>\n\n    <script\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n    >\n      new Vue({ el: \"#js-app\" });\n    </script>\n    <style\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n    >\n      .a {\n        color: #f00;\n      }\n    </style>\n  </body>\n</html>");
}
#[test]
fn test_vue_html_bracket_same_linetrue_format_1_76db7dc4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<template>\n  <div id=\"js-app\">\n<div :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<span :click=\"long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<img :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"/>\n<div class=\"a\">\ntext\n</div>\n<span  class=\"a\">\ntext\n</span>\n<img class=\"a\"/>\n  </div>\n</template>\n\n<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">new Vue({el: '#js-app'})</script>\n<style  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">.a {color: #f00}</style>\n</body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <template>\n      <div id=\"js-app\">\n        <div\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \">\n          text\n        </div>\n        <div\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \"></div>\n        <span\n          :click=\"\n            long_long_long_long_long_long_long_long_long_long_long_long_long_long_value\n          \">\n          text\n        </span>\n        <span\n          :long_long_attribute=\"\n            long_long_long_long_long_long_long_long_long_long_long_value\n          \"></span>\n        <img\n          :long_long_attribute=\"\n            long_long_long_long_long_long_long_long_long_long_long_value\n          \" />\n        <div class=\"a\">text</div>\n        <span class=\"a\"> text </span>\n        <img class=\"a\" />\n      </div>\n    </template>\n\n    <script\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n      new Vue({ el: \"#js-app\" });\n    </script>\n    <style\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n      .a {\n        color: #f00;\n      }\n    </style>\n  </body>\n</html>");
}
#[test]
fn test_vue_vue_bracket_same_linefalse_format_1_d3219d2a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n<div :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div v-on:long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<img long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"/>\n<div class=\"a\">\ntext\n</div>\n<span  class=\"a\">\ntext\n</span>\n<img class=\"a\"/>\n  </div>\n</template>\n\n<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">export default {}</script>\n<style  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">.a {color: #f00}</style>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div>\n    <div\n      :long_long_attribute=\"\n        long_long_long_long_long_long_long_long_long_long_long_value\n      \"\n    >\n      text\n    </div>\n    <div\n      v-on:long_long_attribute=\"\n        long_long_long_long_long_long_long_long_long_long_long_value\n      \"\n    ></div>\n    <span\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n    >\n      text\n    </span>\n    <span\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n    ></span>\n    <img\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n    />\n    <div class=\"a\">text</div>\n    <span class=\"a\"> text </span>\n    <img class=\"a\" />\n  </div>\n</template>\n\n<script\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\nexport default {};\n</script>\n<style\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n.a {\n  color: #f00;\n}\n</style>");
}
#[test]
fn test_vue_vue_bracket_same_linetrue_format_1_d3219d2a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>\n<div :long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</div>\n<div v-on:long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></div>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\ntext\n</span>\n<span long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n<img long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"/>\n<div class=\"a\">\ntext\n</div>\n<span  class=\"a\">\ntext\n</span>\n<img class=\"a\"/>\n  </div>\n</template>\n\n<script long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">export default {}</script>\n<style  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">.a {color: #f00}</style>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div>\n    <div\n      :long_long_attribute=\"\n        long_long_long_long_long_long_long_long_long_long_long_value\n      \">\n      text\n    </div>\n    <div\n      v-on:long_long_attribute=\"\n        long_long_long_long_long_long_long_long_long_long_long_value\n      \"></div>\n    <span\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n      text\n    </span>\n    <span\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\"></span>\n    <img\n      long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\" />\n    <div class=\"a\">text</div>\n    <span class=\"a\"> text </span>\n    <img class=\"a\" />\n  </div>\n</template>\n\n<script\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\nexport default {};\n</script>\n<style\n  long_long_attribute=\"long_long_long_long_long_long_long_long_long_long_long_value\">\n.a {\n  color: #f00;\n}\n</style>");
}
