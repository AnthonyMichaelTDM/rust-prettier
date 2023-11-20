#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basics_hbs_print_width_40_format_1_218a6559() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{!-- after --}}\n<span>\n    foo <span>bar</span>\n</span>\n\n{{!-- before --}}\n<span>\n  <span>bar</span> foo\n</span>\n\n{{!-- within --}}\n<span>\n  foo <span> bar </span>\n</span>\n\n{{!-- break components --}}\n<div>\n  <SuperSelect>\n    <p>foo<span>bar bar bar</span></p><h1><span><em>yep</em></span></h1>\n  </SuperSelect>\n  <h2>nope</h2>\n</div>\n\n<div>\n   hello <strong>hi</strong> <foo>sdkflsdfjk</foo>\n</div>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{! after }}\n<span>\n  foo\n  <span>bar</span>\n</span>\n\n{{! before }}\n<span>\n  <span>bar</span>\n  foo\n</span>\n\n{{! within }}\n<span>\n  foo\n  <span> bar </span>\n</span>\n\n{{! break components }}\n<div>\n  <SuperSelect>\n    <p>foo<span>bar bar bar</span></p><h1\n    ><span><em>yep</em></span></h1>\n  </SuperSelect>\n  <h2>nope</h2>\n</div>\n\n<div>\n  hello\n  <strong>hi</strong>\n  <foo>sdkflsdfjk</foo>\n</div>");
}
#[test]
fn test_boss_hbs_print_width_40_format_1_ca18ca6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>Hi  {{firstName}}  {{lastName}}   , welcome!</p>\n{{#component propA}}\n    for {{propB}}  do {{propC}} f\n{{/component}}\n{{#component propA}}\n    for {{propB}}  <span>name</span>do {{propC}} f\n{{/component}}\n{{propA}} {{propB}}\n{{propC}}{{propD}}\n<span>{{propE}} {{propF}}</span>\n<span>{{propG}}{{propH}}</span>\n\n  \n\nhey") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>Hi\n  {{firstName}}\n  {{lastName}}\n  , welcome!</p>\n{{#component propA}}\n  for\n  {{propB}}\n  do\n  {{propC}}\n  f\n{{/component}}\n{{#component propA}}\n  for\n  {{propB}}\n  <span>name</span>do\n  {{propC}}\n  f\n{{/component}}\n{{propA}}\n{{propB}}\n{{propC}}{{propD}}\n<span>{{propE}} {{propF}}</span>\n<span>{{propG}}{{propH}}</span>\n\nhe");
}
#[test]
fn test_curly_hbs_print_width_40_format_1_9cec655c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<p>Your username is @{{name}}</p>\n<p>Hi {{firstName}} {{lastName}}</p");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<p>Your username is @{{name}}</p>\n<p>Hi {{firstName}} {{lastName}}</p"
    );
}
#[test]
fn test_display_inline_block_hbs_print_width_40_format_1_69dd8a70() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(40)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n<button>\nClick here! Click here! Click here! Click here! Click here! Click here!\n</button>\n<div>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button><button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n</div>\n<div>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n</div>\n<video src=\"brave.webm\"></video>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<button>Click here! Click here! Click\n  here! Click here! Click here! Click\n  here!</button>\n<button>\n  Click here! Click here! Click here!\n  Click here! Click here! Click here!\n</button>\n<div>\n  <button>Click here! Click here! Click\n    here! Click here! Click here! Click\n    here!</button><button>Click here!\n    Click here! Click here! Click here!\n    Click here! Click here!</button>\n</div>\n<div>\n  <button>Click here! Click here! Click\n    here! Click here! Click here! Click\n    here!</button>\n  <button>Click here! Click here! Click\n    here! Click here! Click here! Click\n    here!</button>\n</div>\n<video src=\"brave.webm\"></video");
}
#[test]
fn test_display_none_hbs_print_width_40_format_1_30ef9dce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{!-- TO FIX --}}\n<HEAD><META CHARSET=\"utf-8\" /><TITLE>My tITlE</TITLE><META NAME=\"description\" content=\"My CoNtEnT\" /></HEAD>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{! TO FIX }}\n<HEAD><META CHARSET=\"utf-8\" /><TITLE>My\n    tITlE</TITLE><META\n    NAME=\"description\"\n    content=\"My CoNtEnT\"\n  /></HEAD");
}
#[test]
fn test_fill_hbs_print_width_40_format_1_a51f7b7e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>\n  <img\n    src=\"/images/pansies.jpg\"\n    alt=\"about fedco bottom image\"\n    style=\"float: left;\"\n  /><strong>We are a cooperative</strong>, one of the few seed companies so organized\n  in the United States. Because we do not have an individual owner or beneficiary,\n  profit is not our primary goal. Consumers own 60% of the cooperative and worker\n  members 40%. Consumer and worker members share proportionately in the cooperative&#8217;s\n  profits through our annual patronage dividends.\n</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>\n  <img\n    src=\"/images/pansies.jpg\"\n    alt=\"about fedco bottom image\"\n    style=\"float: left;\"\n  /><strong>We are a cooperative</strong>,\n  one of the few seed companies so\n  organized in the United States.\n  Because we do not have an individual\n  owner or beneficiary, profit is not\n  our primary goal. Consumers own 60% of\n  the cooperative and worker members\n  40%. Consumer and worker members share\n  proportionately in the\n  cooperative&#8217;s profits through\n  our annual patronage dividends.\n</p");
}
#[test]
fn test_inline_element_hbs_print_width_40_format_1_67ab3d79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{!-- TO FIX --}}\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa vel augue \nvestibulum facilisis in porta turpis. Ut faucibus lectus sit amet urna consectetur dignissim.\nSam vitae neque quis ex dapibus faucibus at sed ligula. Nulla sit amet aliquet nibh.\nVestibulum at congue mi. Suspendisse vitae odio vitae massa hendrerit mattis sed eget dui.\nSed eu scelerisque neque. Donec <b>maximus</b> rhoncus pellentesque. Aenean purus turpis, vehicula \neuismod ante vel, ultricies eleifend dui. Class aptent taciti sociosqu ad litora torquent per \nconubia nostra, per inceptos himenaeos. Donec in ornare velit.</p>\n\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa vel augue \nvestibulum facilisis in porta turpis. Ut faucibus lectus sit amet urna consectetur dignissim.\nSam vitae neque quis ex dapibus faucibus at sed ligula. Nulla sit amet aliquet nibh.\nVestibulum at congue mi. Suspendisse vitae odio vitae massa hendrerit mattis sed eget dui.\nSed eu scelerisque neque. Donec <a href=\"#\"><b>maximus</b></a> rhoncus pellentesque. Aenean purus turpis, vehicula \neuismod ante vel, ultricies eleifend dui. Class aptent taciti sociosqu ad litora torquent per \nconubia nostra, per inceptos himenaeos. Donec in ornare velit.</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{! TO FIX }}\n<p>Lorem ipsum dolor sit amet,\n  consectetur adipiscing elit. Fusce\n  cursus massa vel augue vestibulum\n  facilisis in porta turpis. Ut faucibus\n  lectus sit amet urna consectetur\n  dignissim. Sam vitae neque quis ex\n  dapibus faucibus at sed ligula. Nulla\n  sit amet aliquet nibh. Vestibulum at\n  congue mi. Suspendisse vitae odio\n  vitae massa hendrerit mattis sed eget\n  dui. Sed eu scelerisque neque. Donec\n  <b>maximus</b>\n  rhoncus pellentesque. Aenean purus\n  turpis, vehicula euismod ante vel,\n  ultricies eleifend dui. Class aptent\n  taciti sociosqu ad litora torquent per\n  conubia nostra, per inceptos\n  himenaeos. Donec in ornare velit.</p>\n\n<p>Lorem ipsum dolor sit amet,\n  consectetur adipiscing elit. Fusce\n  cursus massa vel augue vestibulum\n  facilisis in porta turpis. Ut faucibus\n  lectus sit amet urna consectetur\n  dignissim. Sam vitae neque quis ex\n  dapibus faucibus at sed ligula. Nulla\n  sit amet aliquet nibh. Vestibulum at\n  congue mi. Suspendisse vitae odio\n  vitae massa hendrerit mattis sed eget\n  dui. Sed eu scelerisque neque. Donec\n  <a href=\"#\"><b>maximus</b></a>\n  rhoncus pellentesque. Aenean purus\n  turpis, vehicula euismod ante vel,\n  ultricies eleifend dui. Class aptent\n  taciti sociosqu ad litora torquent per\n  conubia nostra, per inceptos\n  himenaeos. Donec in ornare velit.</p");
}
#[test]
fn test_non_breaking_whitespace_hbs_print_width_40_format_1_36686950() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(40)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- normal whitespaces -->\n<span>Nihil aut odit omnis. Quam maxime est molestiae. Maxime dolorem dolores voluptas quaerat ut qui sunt vitae error.</span>\n<!-- non-breaking whitespaces -->\n<span>Nihil\u{a0}aut\u{a0}odit\u{a0}omnis.\u{a0}Quam\u{a0}maxime\u{a0}est\u{a0}molestiae.\u{a0}Maxime\u{a0}dolorem\u{a0}dolores\u{a0}voluptas\u{a0}quaerat\u{a0}ut\u{a0}qui\u{a0}sunt\u{a0}vitae\u{a0}error.</span>\n<!-- non-breaking narrow whitespaces -->\n<span>Prix\u{202f}:\u{202f}32\u{202f}€</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- normal whitespaces -->\n<span>Nihil aut odit omnis. Quam maxime\n  est molestiae. Maxime dolorem dolores\n  voluptas quaerat ut qui sunt vitae\n  error.</span>\n<!-- non-breaking whitespaces -->\n<span\n>Nihil\u{a0}aut\u{a0}odit\u{a0}omnis.\u{a0}Quam\u{a0}maxime\u{a0}est\u{a0}molestiae.\u{a0}Maxime\u{a0}dolorem\u{a0}dolores\u{a0}voluptas\u{a0}quaerat\u{a0}ut\u{a0}qui\u{a0}sunt\u{a0}vitae\u{a0}error.</span>\n<!-- non-breaking narrow whitespaces -->\n<span>Prix\u{202f}:\u{202f}32\u{202f}€</span");
}
#[test]
fn test_preserved_spaces_and_breaks_hbs_print_width_40_format_1_5a5436e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(40)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<SomeComponent />\n{{name}}\n\n\n\nSome sentence with  {{dynamic}}  expressions.\n\nsometimes{{nogaps}}areimportant<Hello></Hello>\n\n\n{{name}}  is your name\n\n\n{{#block}}\n{{/block}}\n\n\n{{#block}}\n  some {{text}}\n{{/block}}\n\n{{#block}}\n\n  some {{text}}\n{{/block}}\n\n<HelloWorld>\n</HelloWorld>\n\n<HelloWorld>\n  some {{text}}\n</HelloWorld>\n\n<HelloWorld>\n\n  some {{text}}\n</HelloWorld>\n\n<div class=\"a list of classes\">\n</div>\n\n<div class=\"a list of classes\">\n\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<SomeComponent />\n{{name}}\n\nSome sentence with\n{{dynamic}}\nexpressions. sometimes{{nogaps}}areimportant<Hello\n/>\n\n{{name}}\nis your name\n\n{{#block}}{{/block}}\n\n{{#block}}\n  some\n  {{text}}\n{{/block}}\n\n{{#block}}\n\n  some\n  {{text}}\n{{/block}}\n\n<HelloWorld />\n\n<HelloWorld>\n  some\n  {{text}}\n</HelloWorld>\n\n<HelloWorld>\n\n  some\n  {{text}}\n</HelloWorld>\n\n<div class=\"a list of classes\">\n</div>\n\n<div class=\"a list of classes\">\n\n</div");
}
#[test]
fn test_punctuation_hbs_print_width_40_format_1_27ce49f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span>This is your name: {{name}}.</span>\n<span>This is your name: {{name}} (employee)</span>\n<span>This is your name: {{name}} ({{role}})</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span>This is your name:\n  {{name}}.</span>\n<span>This is your name:\n  {{name}}\n  (employee)</span>\n<span>This is your name:\n  {{name}}\n  ({{role}})</span");
}
#[test]
fn test_surrounding_linebreak_hbs_print_width_40_format_1_2c745e6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span>123</span>\n<span>\n123</span>\n<span>123\n</span>\n<span>\n123\n</span>\n\n<div>123</div>\n<div>\n123</div>\n<div>123\n</div>\n<div>\n123\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span>123</span>\n<span>\n  123</span>\n<span>123\n</span>\n<span>\n  123\n</span>\n\n<div>123</div>\n<div>\n  123</div>\n<div>123\n</div>\n<div>\n  123\n</div");
}
#[test]
fn test_table_hbs_print_width_40_format_1_2b5f20f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(40)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table><thead><tr><th>A</th><th>B</th><th>C</th></tr></thead></table>\n\n<table> <thead> <tr> <th> A </th> <th> B </th> <th> C </th> </tr> </thead> </table>\n\n<table>\n  <thead>\n    <tr>\n    </tr>\n  </thead>\n</table>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table><thead><tr><th>A</th><th\n      >B</th><th\n      >C</th></tr></thead></table>\n\n<table>\n  <thead>\n    <tr>\n      <th> A </th>\n      <th> B </th>\n      <th> C </th>\n    </tr>\n  </thead>\n</table>\n\n<table>\n  <thead>\n    <tr>\n    </tr>\n  </thead>\n</table");
}
#[test]
fn test_textarea_hbs_print_width_40_format_1_01024a9e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["glimmer"])
        .print_width(40)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<Textarea />\n{{#if true}}\n  Test\n{{/if}}\n\n<Textarea />\n\n{{#if true}}\n  Test\n{{/if}}\n\n<Textarea />\n\n\n\n\n{{#if true}}\n  Test\n{{/if}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<Textarea />\n{{#if true}}\n  Test\n{{/if}}\n\n<Textarea />\n\n{{#if true}}\n  Test\n{{/if}}\n\n<Textarea />\n\n{{#if true}}\n  Test\n{{/if}");
}
#[test]
fn test_whitespace_control_hbs_print_width_40_format_1_39af7daa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(40)
        .parsers(vec!["glimmer"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{{~#if foo~}}\n  abc\n{{~/if~}}\n\n{{~#if foo}}\n  abc\n{{/if~}}\n\n{{foo~}}\n\n{{~foo abc}}\n\n{{~#if bar}}\n  if1\n{{~else~}}\n  else\n{{~/if~}}\n\n{{~#if bar}}\n  if1\n{{~else if foo~}}\n  else if foo\n{{else~}}\n  else\n{{/if~}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{{~#if foo~}}\n  abc\n{{~/if~}}\n\n{{~#if foo}}\n  abc\n{{/if~}}\n\n{{foo~}}\n\n{{~foo abc}}\n\n{{~#if bar}}\n  if1\n{{~else~}}\n  else\n{{~/if~}}\n\n{{~#if bar}}\n  if1\n{{~else if foo~}}\n  else if foo\n{{else~}}\n  else\n{{/if~}");
}
