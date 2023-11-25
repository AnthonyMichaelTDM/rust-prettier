use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cssnext_example_css_format_1_ad643a16() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* custom properties */\n:root{--fontSize: 1rem;\n  --mainColor       :#12345678;\n--highlightColor:hwb(190, 35%, 20%);\n}\n\n/* custom media queries */\n@custom-media\n\n--viewport-medium(width<=50rem);\n\n/* some var() & calc() */\nbody{color:var(--mainColor);\n    font-size:var(--fontSize);\n line-height: calc(var(--fontSize) * 1.5);\npadding: calc((var(--fontSize) / 2) + 1px)}\n\n/* custom media query usage */\n@media (--viewport-medium) {\nbody {font-size: calc(var(--fontSize) * 1.2); }\n}\n/* custom selectors */\n@custom-selector :--heading h1,h2,h3,    h4,h5,h6;\n:--heading { margin-top:0 }\n\n/* colors stuff */\na{\ncolor:var(--highlightColor);\n    transition:color 1s;\n}\na:hover{color  :gray(255,50%) }\na:active{color : rebeccapurple }\na:any-link { color:color(var(--highlightColor) blackness(+ 20%)) }\n\n/* font stuff */\nh2 {font-variant-caps:small-caps;\n}table{font-variant-numeric: lining-nums;\n}\n/* filters */\n.blur{filter:blur(4px)}.sepia{\nfilter: sepia(.8);}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* custom properties */\n:root {\n  --fontSize: 1rem;\n  --mainColor: #12345678;\n  --highlightColor: hwb(190, 35%, 20%);\n}\n\n/* custom media queries */\n@custom-media --viewport-medium(width<=50rem);\n\n/* some var() & calc() */\nbody {\n  color: var(--mainColor);\n  font-size: var(--fontSize);\n  line-height: calc(var(--fontSize) * 1.5);\n  padding: calc((var(--fontSize) / 2) + 1px);\n}\n\n/* custom media query usage */\n@media (--viewport-medium) {\n  body {\n    font-size: calc(var(--fontSize) * 1.2);\n  }\n}\n/* custom selectors */\n@custom-selector :--heading h1, h2, h3, h4, h5, h6;\n:--heading {\n  margin-top: 0;\n}\n\n/* colors stuff */\na {\n  color: var(--highlightColor);\n  transition: color 1s;\n}\na:hover {\n  color: gray(255, 50%);\n}\na:active {\n  color: rebeccapurple;\n}\na:any-link {\n  color: color(var(--highlightColor) blackness(+ 20%));\n}\n\n/* font stuff */\nh2 {\n  font-variant-caps: small-caps;\n}\ntable {\n  font-variant-numeric: lining-nums;\n}\n/* filters */\n.blur {\n  filter: blur(4px);\n}\n.sepia {\n  filter: sepia(0.8);\n}");
}
