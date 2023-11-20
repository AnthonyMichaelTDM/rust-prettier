#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_tags_html_format_1_82ca4616() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<a>Lorem</a>, ispum dolor sit <strong>amet</strong>.\n<div><a>Lorem</a>, ispum dolor sit <strong>amet</strong>.</div>\n<div><div><a>Lorem</a>, ispum dolor sit <strong>amet</strong>.</div></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<a>Lorem</a>, ispum dolor sit <strong>amet</strong>.\n<div><a>Lorem</a>, ispum dolor sit <strong>amet</strong>.</div>\n<div>\n  <div><a>Lorem</a>, ispum dolor sit <strong>amet</strong>.</div>\n</div>");
}
#[test]
fn test_display_inline_block_html_format_1_3a1c9255() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n<button>\nClick here! Click here! Click here! Click here! Click here! Click here!\n</button>\n<div>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button><button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n</div>\n<div>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n<button>Click here! Click here! Click here! Click here! Click here! Click here!</button>\n</div>\n<video src=\"brave.webm\"><track kind=subtitles src=brave.en.vtt srclang=en label=\"English\"><track kind=subtitles src=brave.en.vtt srclang=en label=\"English\"></video>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<button>\n  Click here! Click here! Click here! Click here! Click here! Click here!\n</button>\n<button>\n  Click here! Click here! Click here! Click here! Click here! Click here!\n</button>\n<div>\n  <button>\n    Click here! Click here! Click here! Click here! Click here! Click here!</button\n  ><button>\n    Click here! Click here! Click here! Click here! Click here! Click here!\n  </button>\n</div>\n<div>\n  <button>\n    Click here! Click here! Click here! Click here! Click here! Click here!\n  </button>\n  <button>\n    Click here! Click here! Click here! Click here! Click here! Click here!\n  </button>\n</div>\n<video src=\"brave.webm\">\n  <track kind=\"subtitles\" src=\"brave.en.vtt\" srclang=\"en\" label=\"English\" />\n  <track kind=\"subtitles\" src=\"brave.en.vtt\" srclang=\"en\" label=\"English\" />\n</video>");
}
#[test]
fn test_display_none_html_format_1_4b498e50() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html><HTML CLASS=\"no-js mY-ClAsS\"><HEAD><META CHARSET=\"utf-8\"><TITLE>My tITlE</TITLE><META NAME=\"description\" content=\"My CoNtEnT\"></HEAD></HTML>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html class=\"no-js mY-ClAsS\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>My tITlE</title>\n    <meta name=\"description\" content=\"My CoNtEnT\" />\n  </head>\n</html>");
}
#[test]
fn test_fill_html_format_1_a51f7b7e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>\n  <img\n    src=\"/images/pansies.jpg\"\n    alt=\"about fedco bottom image\"\n    style=\"float: left;\"\n  /><strong>We are a cooperative</strong>, one of the few seed companies so organized\n  in the United States. Because we do not have an individual owner or beneficiary,\n  profit is not our primary goal. Consumers own 60% of the cooperative and worker\n  members 40%. Consumer and worker members share proportionately in the cooperative&#8217;s\n  profits through our annual patronage dividends.\n</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>\n  <img\n    src=\"/images/pansies.jpg\"\n    alt=\"about fedco bottom image\"\n    style=\"float: left\"\n  /><strong>We are a cooperative</strong>, one of the few seed companies so\n  organized in the United States. Because we do not have an individual owner or\n  beneficiary, profit is not our primary goal. Consumers own 60% of the\n  cooperative and worker members 40%. Consumer and worker members share\n  proportionately in the cooperative&#8217;s profits through our annual\n  patronage dividends.\n</p>");
}
#[test]
fn test_inline_leading_trailing_spaces_html_format_1_e652d2b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<span> 321 </span>\n\n<span> <a>321</a> </span>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<span> 321 </span>\n\n<span> <a>321</a> </span>");
}
#[test]
fn test_inline_nodes_html_format_1_cd949be9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa vel augue \nvestibulum facilisis in porta turpis. Ut faucibus lectus sit amet urna consectetur dignissim.\nSam vitae neque quis ex dapibus faucibus at sed ligula. Nulla sit amet aliquet nibh.\nVestibulum at congue mi. Suspendisse vitae odio vitae massa hendrerit mattis sed eget dui.\nSed eu scelerisque neque. Donec <b>maximus</b> rhoncus pellentesque. Aenean purus turpis, vehicula \neuismod ante vel, ultricies eleifend dui. Class aptent taciti sociosqu ad litora torquent per \nconubia nostra, per inceptos himenaeos. Donec in ornare velit.</p>\n\n<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa vel augue \nvestibulum facilisis in porta turpis. Ut faucibus lectus sit amet urna consectetur dignissim.\nSam vitae neque quis ex dapibus faucibus at sed ligula. Nulla sit amet aliquet nibh.\nVestibulum at congue mi. Suspendisse vitae odio vitae massa hendrerit mattis sed eget dui.\nSed eu scelerisque neque. Donec <a href=\"#\"><b>maximus</b></a> rhoncus pellentesque. Aenean purus turpis, vehicula \neuismod ante vel, ultricies eleifend dui. Class aptent taciti sociosqu ad litora torquent per \nconubia nostra, per inceptos himenaeos. Donec in ornare velit.</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa\n  vel augue vestibulum facilisis in porta turpis. Ut faucibus lectus sit amet\n  urna consectetur dignissim. Sam vitae neque quis ex dapibus faucibus at sed\n  ligula. Nulla sit amet aliquet nibh. Vestibulum at congue mi. Suspendisse\n  vitae odio vitae massa hendrerit mattis sed eget dui. Sed eu scelerisque\n  neque. Donec <b>maximus</b> rhoncus pellentesque. Aenean purus turpis,\n  vehicula euismod ante vel, ultricies eleifend dui. Class aptent taciti\n  sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec\n  in ornare velit.\n</p>\n\n<p>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit. Fusce cursus massa\n  vel augue vestibulum facilisis in porta turpis. Ut faucibus lectus sit amet\n  urna consectetur dignissim. Sam vitae neque quis ex dapibus faucibus at sed\n  ligula. Nulla sit amet aliquet nibh. Vestibulum at congue mi. Suspendisse\n  vitae odio vitae massa hendrerit mattis sed eget dui. Sed eu scelerisque\n  neque. Donec <a href=\"#\"><b>maximus</b></a> rhoncus pellentesque. Aenean purus\n  turpis, vehicula euismod ante vel, ultricies eleifend dui. Class aptent taciti\n  sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Donec\n  in ornare velit.\n</p>");
}
#[test]
fn test_nested_inline_without_whitespace_html_format_1_fa9630bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<a href=\"/wiki/Help:IPA/English\" title=\"Help:IPA/English\">/<span style=\"border-bottom:1px dotted\"><span title=\"/ˌ/: secondary stress follows\">ˌ</span\n><span title=\"/ɪ/: &#39;i&#39; in &#39;kit&#39;\">ɪ</span\n><span title=\"&#39;l&#39; in &#39;lie&#39;\">l</span\n><span title=\"/ə/: &#39;a&#39; in &#39;about&#39;\">ə</span\n><span title=\"/ˈ/: primary stress follows\">ˈ</span\n><span title=\"&#39;n&#39; in &#39;nigh&#39;\">n</span\n><span title=\"/ɔɪ/: &#39;oi&#39; in &#39;choice&#39;\">ɔɪ</span></span>/</a>\n\n<span class=\"word\"><span class=\"syllable\"><span class=\"letter vowel\">i</span><span class=\"letter consonant\">p</span></span\n><span class=\"syllable\"><span class=\"letter consonant onset\">s</span><span class=\"letter vowel\">u</span><span class=\"letter consonant\">m</span></span></span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<a href=\"/wiki/Help:IPA/English\" title=\"Help:IPA/English\"\n  >/<span style=\"border-bottom: 1px dotted\"\n    ><span title=\"/ˌ/: secondary stress follows\">ˌ</span\n    ><span title=\"/ɪ/: &#39;i&#39; in &#39;kit&#39;\">ɪ</span\n    ><span title=\"&#39;l&#39; in &#39;lie&#39;\">l</span\n    ><span title=\"/ə/: &#39;a&#39; in &#39;about&#39;\">ə</span\n    ><span title=\"/ˈ/: primary stress follows\">ˈ</span\n    ><span title=\"&#39;n&#39; in &#39;nigh&#39;\">n</span\n    ><span title=\"/ɔɪ/: &#39;oi&#39; in &#39;choice&#39;\">ɔɪ</span></span\n  >/</a\n>\n\n<span class=\"word\"\n  ><span class=\"syllable\"\n    ><span class=\"letter vowel\">i</span\n    ><span class=\"letter consonant\">p</span></span\n  ><span class=\"syllable\"\n    ><span class=\"letter consonant onset\">s</span\n    ><span class=\"letter vowel\">u</span\n    ><span class=\"letter consonant\">m</span></span\n  ></span\n>");
}
#[test]
fn test_non_breaking_whitespace_html_format_1_36686950() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!-- normal whitespaces -->\n<span>Nihil aut odit omnis. Quam maxime est molestiae. Maxime dolorem dolores voluptas quaerat ut qui sunt vitae error.</span>\n<!-- non-breaking whitespaces -->\n<span>Nihil\u{a0}aut\u{a0}odit\u{a0}omnis.\u{a0}Quam\u{a0}maxime\u{a0}est\u{a0}molestiae.\u{a0}Maxime\u{a0}dolorem\u{a0}dolores\u{a0}voluptas\u{a0}quaerat\u{a0}ut\u{a0}qui\u{a0}sunt\u{a0}vitae\u{a0}error.</span>\n<!-- non-breaking narrow whitespaces -->\n<span>Prix\u{202f}:\u{202f}32\u{202f}€</span>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!-- normal whitespaces -->\n<span\n  >Nihil aut odit omnis. Quam maxime est molestiae. Maxime dolorem dolores\n  voluptas quaerat ut qui sunt vitae error.</span\n>\n<!-- non-breaking whitespaces -->\n<span\n  >Nihil\u{a0}aut\u{a0}odit\u{a0}omnis.\u{a0}Quam\u{a0}maxime\u{a0}est\u{a0}molestiae.\u{a0}Maxime\u{a0}dolorem\u{a0}dolores\u{a0}voluptas\u{a0}quaerat\u{a0}ut\u{a0}qui\u{a0}sunt\u{a0}vitae\u{a0}error.</span\n>\n<!-- non-breaking narrow whitespaces -->\n<span>Prix\u{202f}:\u{202f}32\u{202f}€</span>");
}
#[test]
fn test_surrounding_linebreak_html_format_1_2c745e6f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<span>123</span>\n<span>\n123</span>\n<span>123\n</span>\n<span>\n123\n</span>\n\n<div>123</div>\n<div>\n123</div>\n<div>123\n</div>\n<div>\n123\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span>123</span>\n<span> 123</span>\n<span>123 </span>\n<span> 123 </span>\n\n<div>123</div>\n<div>123</div>\n<div>123</div>\n<div>123</div>");
}
#[test]
fn test_table_html_format_1_2b5f20f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table><thead><tr><th>A</th><th>B</th><th>C</th></tr></thead></table>\n\n<table> <thead> <tr> <th> A </th> <th> B </th> <th> C </th> </tr> </thead> </table>\n\n<table>\n  <thead>\n    <tr>\n    </tr>\n  </thead>\n</table>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table>\n  <thead>\n    <tr>\n      <th>A</th>\n      <th>B</th>\n      <th>C</th>\n    </tr>\n  </thead>\n</table>\n\n<table>\n  <thead>\n    <tr></tr>\n  </thead>\n</table>");
}
#[test]
fn test_template_html_format_1_a77d18b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <template>foo</template>\n</template>\n\n<template>\n  <template>foooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo</template>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <template>foo</template>\n</template>\n\n<template>\n  <template\n    >foooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo</template\n  >\n</template>");
}
