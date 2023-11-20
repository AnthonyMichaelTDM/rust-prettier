#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_is_css_format_1_2ea71cdb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (":is(ol, ul, menu:unsupported) :is(ol, ul) {\n    color: green;\n}\n\n:is(ol, ul) :is(ol, ul) ol {\n    list-style-type: lower-greek;\n    color: chocolate;\n}\n\n:is(ol, ul, menu, dir) :is(ol, ul, menu, dir) :is(ul, menu, dir) {\n  list-style-type: square;\n}\n\n/* Level 0 */\nh1 {\n  font-size: 30px;\n}\n/* Level 1 */\n:is(section, article, aside, nav) h1 {\n  font-size: 25px;\n}\n/* Level 2 */\n:is(section, article, aside, nav) :is(section, article, aside, nav) h1 {\n  font-size: 20px;\n}\n/* Level 3 */\n:is(section, article, aside, nav)  :is(section, article, aside, nav)  :is(section, article, aside, nav)  h1 {\n  font-size: 15px;\n}\n\nsome-element:is(::before, ::after) {\n  display: block;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ":is(ol, ul, menu:unsupported) :is(ol, ul) {\n  color: green;\n}\n\n:is(ol, ul) :is(ol, ul) ol {\n  list-style-type: lower-greek;\n  color: chocolate;\n}\n\n:is(ol, ul, menu, dir) :is(ol, ul, menu, dir) :is(ul, menu, dir) {\n  list-style-type: square;\n}\n\n/* Level 0 */\nh1 {\n  font-size: 30px;\n}\n/* Level 1 */\n:is(section, article, aside, nav) h1 {\n  font-size: 25px;\n}\n/* Level 2 */\n:is(section, article, aside, nav) :is(section, article, aside, nav) h1 {\n  font-size: 20px;\n}\n/* Level 3 */\n:is(section, article, aside, nav)\n  :is(section, article, aside, nav)\n  :is(section, article, aside, nav)\n  h1 {\n  font-size: 15px;\n}\n\nsome-element:is(::before, ::after) {\n  display: block;\n}");
}
#[test]
fn test_pseudo_call_css_format_1_3571c339() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("div:not(:last-child) {\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "div:not(:last-child) {\n}");
}
#[test]
fn test_where_css_format_1_6c5f61e6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (":where(#p0:checked ~ #play:checked ~ #c1:checked, #p1:checked ~ #play:checked ~ #c2:checked, #p2:checked ~ #play:checked ~ #cO:checked) ~ #result >\n#c { display: block; }\n\n:where(ol, ul, menu:unsupported) :where(ol, ul) {\n    color: green;\n}\n\n:where(ol, ul) :where(ol, ul) ol {\n    list-style-type: lower-greek;\n    color: chocolate;\n}\n\n:is(section.is-styling, aside.is-styling, footer.is-styling) a {\n  color: red;\n}\n\n:where(section.where-styling, aside.where-styling, footer.where-styling) a {\n  color: orange;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ":where(\n    #p0:checked ~ #play:checked ~ #c1:checked,\n    #p1:checked ~ #play:checked ~ #c2:checked,\n    #p2:checked ~ #play:checked ~ #cO:checked\n  )\n  ~ #result\n  > #c {\n  display: block;\n}\n\n:where(ol, ul, menu:unsupported) :where(ol, ul) {\n  color: green;\n}\n\n:where(ol, ul) :where(ol, ul) ol {\n  list-style-type: lower-greek;\n  color: chocolate;\n}\n\n:is(section.is-styling, aside.is-styling, footer.is-styling) a {\n  color: red;\n}\n\n:where(section.where-styling, aside.where-styling, footer.where-styling) a {\n  color: orange;\n}");
}
