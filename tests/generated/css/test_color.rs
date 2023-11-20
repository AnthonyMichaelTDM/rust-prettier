#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_color_adjuster_css_format_1_59dd4df9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n    color: color(red l(+ 20%));\n    color: color(red w(+ 20%) s(+ 20%));\n    color: color(swopc, 0 206 190 77);\n    color: color(indigo, 24 160 86 42 0 18 31);\n    color: color(prophoto, 233 150 122);\n    color: color(P3, 97 253 36);\n    color: color(#eb8fa9 alpha(75%) blackness(20%));\n    color: color(red blue(20));\n    color: color(red blue(20%));\n    color: color(red green(+ 20));\n    color: color(red green(+ 20%));\n    color: color(red red(- 20));\n    color: color(red red(- 20%));\n    color: color(red red(- 128));\n    color: color(red alpha(- 50%));\n    color: color(red alpha(- .75));\n    color: color(red rgb(+ 0 255 0));\n    color: color(red rgb(+ #0f0));\n    color: color(red rgb(- 60% 0 0));\n    color: color(red rgb(- #900));\n    color: color(rebeccapurple rgb(* 1%));\n    color: color(red hue(20));\n    color: color(red hue(20deg));\n    color: color(red hue(+ 20));\n    color: color(red hue(+ 20deg));\n    color: color(red hue(- 20));\n    color: color(red hue(- 20deg));\n    color: color(red hue(* 20));\n    color: color(red hue(* 20deg));\n    color: color(red lightness(50%));\n    color: color(red lightness(20%));\n    color: color(red lightness(+ 20%));\n    color: color(red lightness(- 20%));\n    color: color(red lightness(* 1.5%));\n    color: color(beige saturation(20%));\n    color: color(beige saturation(+ 20%));\n    color: color(beige saturation(- 20%));\n    color: color(beige saturation(* 1.5%));\n    color: color(beige blackness(20%));\n    color: color(beige blackness(+ 20%));\n    color: color(beige blackness(- 1%));\n    color: color(beige blackness(* 20%));\n    color: color(beige whiteness(20%));\n    color: color(beige whiteness(+ 1%));\n    color: color(beige whiteness(- 20%));\n    color: color(beige whiteness(* .5%));\n    color: color(red);\n    color: color(red tint(0%));\n    color: color(red shade(0%));\n    color: color(red tint(100%));\n    color: color(red shade(100%));\n    color: color(red tint(20%));\n    color: color(red shade(20%));\n    color: color(yellow blend(red 50%));\n    color: color(yellow blend(red 50% rgb));\n    color: color(yellow blend(red 50% hsl));\n    color: color(yellow blend(red 50% hwb));\n    color: color(#937b19 contrast(25%));\n    color: color(hotpink blend(yellow 59%));\n    color: color(red);\n    color: color(color-mod(red));\n    color: color(color-mod(color-mod(red)));\n    color: color(#f00);\n    color: color(#f00f);\n    color: color(#ff0000);\n    color: color(#ff0000ff);\n    color: color(rgb(100% 0% 0%));\n    color: color(rgb(100% 0% 0% / 100%));\n    color: color(rgb(255, 0, 0));\n    color: color(rgb(255, 0, 0, 1));\n    color: color(rgba(255, 0, 0, 1));\n    color: color(hsl(0 100% 50%));\n    color: color(hsl(0 100% 50% / 100%));\n    color: color(hsl(0, 100%, 50%));\n    color: color(hsl(0, 100%, 50%, 1));\n    color: color(hsla(0, 100%, 50%, 1));\n    color: color(hwb(0 0 0));\n    color: color(hwb(0 0% 0%));\n    color: color(hwb(0 0% 0% / 100%));\n    color: color(0);\n    color: color(0deg);\n    color: color(0grad);\n    color: color(0rad);\n    color: color(beige);\n    color: color(beige hue(+ 0deg));\n    color: color(beige saturation(+ 0%));\n    color: color(beige lightness(+ 0%));\n    color: color(beige alpha(1));\n    color: color(beige alpha(+ 0));\n    color: color(beige alpha(+ 0%));\n    color: color(beige blend(beige 0% hsl));\n    color: color(red);\n    color: color(red tint(0%));\n    color: color(red shade(0%));\n    color: color(red tint(100%));\n    color: color(red shade(100%));\n    color: color(red tint(20%));\n    color: color(red shade(20%));\n    color: color(yellow blend(red 50%));\n    color: color(yellow blend(red 50% rgb));\n    color: color(yellow blend(red 50% hsl));\n    color: color(yellow blend(red 50% hwb));\n    color: color(yellow contrast(0%));\n    color: color(yellow contrast(25%));\n    color: color(yellow contrast(50%));\n    color: color(yellow contrast(75%));\n    color: color(yellow contrast(100%));\n    color: color(color-mod(0deg blue(10%)) rgb(+ 0 10 0) hue(+ 10deg) tint(10%) lightness(+ 10%) saturation(+ 10%) blend(rebeccapurple 50%));\n    color: color(var(--color));\n    color: color(var(--color) l(+ 20%));\n    color: color(red l(+20%)); /* interpreted as part of the number */\n    color: color(red l(-20%)); /* interpreted as part of the number */\n    color: color(red hue(+20));\n    color: color(red hue(+20deg));\n    color: color(red hue(-20));\n    color: color(red hue(-20deg));\n    color: color(red hue(*20));\n    color: color(red hue(*20deg));\n    color: color(var(--highlightColor) blackness(+ 20%));\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  color: color(red l(+ 20%));\n  color: color(red w(+ 20%) s(+ 20%));\n  color: color(swopc, 0 206 190 77);\n  color: color(indigo, 24 160 86 42 0 18 31);\n  color: color(prophoto, 233 150 122);\n  color: color(P3, 97 253 36);\n  color: color(#eb8fa9 alpha(75%) blackness(20%));\n  color: color(red blue(20));\n  color: color(red blue(20%));\n  color: color(red green(+ 20));\n  color: color(red green(+ 20%));\n  color: color(red red(- 20));\n  color: color(red red(- 20%));\n  color: color(red red(- 128));\n  color: color(red alpha(- 50%));\n  color: color(red alpha(- 0.75));\n  color: color(red rgb(+ 0 255 0));\n  color: color(red rgb(+ #0f0));\n  color: color(red rgb(- 60% 0 0));\n  color: color(red rgb(- #900));\n  color: color(rebeccapurple rgb(* 1%));\n  color: color(red hue(20));\n  color: color(red hue(20deg));\n  color: color(red hue(+ 20));\n  color: color(red hue(+ 20deg));\n  color: color(red hue(- 20));\n  color: color(red hue(- 20deg));\n  color: color(red hue(* 20));\n  color: color(red hue(* 20deg));\n  color: color(red lightness(50%));\n  color: color(red lightness(20%));\n  color: color(red lightness(+ 20%));\n  color: color(red lightness(- 20%));\n  color: color(red lightness(* 1.5%));\n  color: color(beige saturation(20%));\n  color: color(beige saturation(+ 20%));\n  color: color(beige saturation(- 20%));\n  color: color(beige saturation(* 1.5%));\n  color: color(beige blackness(20%));\n  color: color(beige blackness(+ 20%));\n  color: color(beige blackness(- 1%));\n  color: color(beige blackness(* 20%));\n  color: color(beige whiteness(20%));\n  color: color(beige whiteness(+ 1%));\n  color: color(beige whiteness(- 20%));\n  color: color(beige whiteness(* 0.5%));\n  color: color(red);\n  color: color(red tint(0%));\n  color: color(red shade(0%));\n  color: color(red tint(100%));\n  color: color(red shade(100%));\n  color: color(red tint(20%));\n  color: color(red shade(20%));\n  color: color(yellow blend(red 50%));\n  color: color(yellow blend(red 50% rgb));\n  color: color(yellow blend(red 50% hsl));\n  color: color(yellow blend(red 50% hwb));\n  color: color(#937b19 contrast(25%));\n  color: color(hotpink blend(yellow 59%));\n  color: color(red);\n  color: color(color-mod(red));\n  color: color(color-mod(color-mod(red)));\n  color: color(#f00);\n  color: color(#f00f);\n  color: color(#ff0000);\n  color: color(#ff0000ff);\n  color: color(rgb(100% 0% 0%));\n  color: color(rgb(100% 0% 0% / 100%));\n  color: color(rgb(255, 0, 0));\n  color: color(rgb(255, 0, 0, 1));\n  color: color(rgba(255, 0, 0, 1));\n  color: color(hsl(0 100% 50%));\n  color: color(hsl(0 100% 50% / 100%));\n  color: color(hsl(0, 100%, 50%));\n  color: color(hsl(0, 100%, 50%, 1));\n  color: color(hsla(0, 100%, 50%, 1));\n  color: color(hwb(0 0 0));\n  color: color(hwb(0 0% 0%));\n  color: color(hwb(0 0% 0% / 100%));\n  color: color(0);\n  color: color(0deg);\n  color: color(0grad);\n  color: color(0rad);\n  color: color(beige);\n  color: color(beige hue(+ 0deg));\n  color: color(beige saturation(+ 0%));\n  color: color(beige lightness(+ 0%));\n  color: color(beige alpha(1));\n  color: color(beige alpha(+ 0));\n  color: color(beige alpha(+ 0%));\n  color: color(beige blend(beige 0% hsl));\n  color: color(red);\n  color: color(red tint(0%));\n  color: color(red shade(0%));\n  color: color(red tint(100%));\n  color: color(red shade(100%));\n  color: color(red tint(20%));\n  color: color(red shade(20%));\n  color: color(yellow blend(red 50%));\n  color: color(yellow blend(red 50% rgb));\n  color: color(yellow blend(red 50% hsl));\n  color: color(yellow blend(red 50% hwb));\n  color: color(yellow contrast(0%));\n  color: color(yellow contrast(25%));\n  color: color(yellow contrast(50%));\n  color: color(yellow contrast(75%));\n  color: color(yellow contrast(100%));\n  color: color(\n    color-mod(0deg blue(10%)) rgb(+ 0 10 0) hue(+ 10deg) tint(10%)\n      lightness(+ 10%) saturation(+ 10%) blend(rebeccapurple 50%)\n  );\n  color: color(var(--color));\n  color: color(var(--color) l(+ 20%));\n  color: color(red l(+20%)); /* interpreted as part of the number */\n  color: color(red l(-20%)); /* interpreted as part of the number */\n  color: color(red hue(+20));\n  color: color(red hue(+20deg));\n  color: color(red hue(-20));\n  color: color(red hue(-20deg));\n  color: color(red hue(* 20));\n  color: color(red hue(* 20deg));\n  color: color(var(--highlightColor) blackness(+ 20%));\n}");
}
#[test]
fn test_color_level_4_css_format_1_75a67dc1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* https://www.w3.org/TR/css-color-4 */\nfoo {\n  color: rgb(0, 128, 255);\n  color: rgba(0, 128, 255, 0.5);\n  color: hsl(198, 28%, 50%);\n  color: hsla(198, 28%, 50%, 0.5);\n  color: lab(56.29% 019.93 16.58 / 50%);\n  color: lch(56.29% 19.86 236.62 / 50%);\n  /* color(sRGB 0 0.50 1 / 50%); */\n}\n\nbar {\n  color: rgb(0 128 255);\n  color: rgb(0 128 255 / 50%);\n  color: hsl(198deg 28% 50%);\n  color: hsl(198deg 28% 50% / 50%);\n  color: lab(56.29% 019.93 16.58 / 50%);\n  color: lch(56.29% 19.86 236.62deg / 50%);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* https://www.w3.org/TR/css-color-4 */\nfoo {\n  color: rgb(0, 128, 255);\n  color: rgba(0, 128, 255, 0.5);\n  color: hsl(198, 28%, 50%);\n  color: hsla(198, 28%, 50%, 0.5);\n  color: lab(56.29% 019.93 16.58 / 50%);\n  color: lch(56.29% 19.86 236.62 / 50%);\n  /* color(sRGB 0 0.50 1 / 50%); */\n}\n\nbar {\n  color: rgb(0 128 255);\n  color: rgb(0 128 255 / 50%);\n  color: hsl(198deg 28% 50%);\n  color: hsl(198deg 28% 50% / 50%);\n  color: lab(56.29% 019.93 16.58 / 50%);\n  color: lch(56.29% 19.86 236.62deg / 50%);\n}");
}
#[test]
fn test_current_color_css_format_1_53db9028() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        ".foo {\n    border: 1px dashed currentColor;\n    border: 1px dashed currentcolor;\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ".foo {\n  border: 1px dashed currentColor;\n  border: 1px dashed currentcolor;\n}"
    );
}
#[test]
fn test_functional_syntax_css_format_1_937c4804() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n    color: rgb(255, 0, 153);\n    color: rgb(100%, 0%, 60%);\n    color: rgba(51, 170, 51, 0.1);\n    color: hsl(.75turn, 60%, 70%);\n    color: hsl(270 60% 50% / .15);\n    color: hsla(240, 100%, 50%, .05)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  color: rgb(255, 0, 153);\n  color: rgb(100%, 0%, 60%);\n  color: rgba(51, 170, 51, 0.1);\n  color: hsl(0.75turn, 60%, 70%);\n  color: hsl(270 60% 50% / 0.15);\n  color: hsla(240, 100%, 50%, 0.05);\n}");
}
#[test]
fn test_hexcolor_css_format_1_aedb2458() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n    color: #AAA;\n    -o-color: #fabcd3;\n    -webkit-color: #873;\n    -moz-color: #6bcdef;\n    -ms-color: #AAbbCC;\n    color: #F09F;\n    color: #FF0099FF;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  color: #aaa;\n  -o-color: #fabcd3;\n  -webkit-color: #873;\n  -moz-color: #6bcdef;\n  -ms-color: #aabbcc;\n  color: #f09f;\n  color: #ff0099ff;\n}");
}
#[test]
fn test_whitespace_syntax_css_format_1_1ede3259() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".foo {\n    color: rgba(51 170 51 / 0.4);\n    color: rgba(51 170 51 / 40%);\n    color: hsl(270 60% 50% / .15);\n    color: hsla(240 100% 50% / .05);\n    color: hsla(240 100% 50% / 5%);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  color: rgba(51 170 51 / 0.4);\n  color: rgba(51 170 51 / 40%);\n  color: hsl(270 60% 50% / 0.15);\n  color: hsla(240 100% 50% / 0.05);\n  color: hsla(240 100% 50% / 5%);\n}");
}
