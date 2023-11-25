use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_last_line_js_bracket_same_linefalse_format_1_7a632ffd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<SomeHighlyConfiguredComponent\n  onEnter={this.onEnter}\n  onLeave={this.onLeave}\n  onChange={this.onChange}\n  initialValue={this.state.initialValue}\n  ignoreStuff={true}\n>\n  <div>and the children go here</div>\n  <div>and here too</div>\n</SomeHighlyConfiguredComponent>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<SomeHighlyConfiguredComponent\n  onEnter={this.onEnter}\n  onLeave={this.onLeave}\n  onChange={this.onChange}\n  initialValue={this.state.initialValue}\n  ignoreStuff={true}\n>\n  <div>and the children go here</div>\n  <div>and here too</div>\n</SomeHighlyConfiguredComponent>;");
}
#[test]
fn test_last_line_js_bracket_same_linetrue_format_1_7a632ffd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<SomeHighlyConfiguredComponent\n  onEnter={this.onEnter}\n  onLeave={this.onLeave}\n  onChange={this.onChange}\n  initialValue={this.state.initialValue}\n  ignoreStuff={true}\n>\n  <div>and the children go here</div>\n  <div>and here too</div>\n</SomeHighlyConfiguredComponent>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<SomeHighlyConfiguredComponent\n  onEnter={this.onEnter}\n  onLeave={this.onLeave}\n  onChange={this.onChange}\n  initialValue={this.state.initialValue}\n  ignoreStuff={true}>\n  <div>and the children go here</div>\n  <div>and here too</div>\n</SomeHighlyConfiguredComponent>;");
}
#[test]
fn test_single_prop_multiline_string_js_bracket_same_linefalse_format_1_019b0603() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<path d=\"M4.765 16.829l3.069-2.946 5.813 5.748\n  11.33-11.232 3.006 3.18-14.36 14.080z\" />;\n\n<Component text=\"Text\n  with\n  newlines\">Content</Component>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<path\n  d=\"M4.765 16.829l3.069-2.946 5.813 5.748\n  11.33-11.232 3.006 3.18-14.36 14.080z\"\n/>;\n\n<Component\n  text=\"Text\n  with\n  newlines\"\n>\n  Content\n</Component>;");
}
#[test]
fn test_single_prop_multiline_string_js_bracket_same_linetrue_format_1_019b0603() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<path d=\"M4.765 16.829l3.069-2.946 5.813 5.748\n  11.33-11.232 3.006 3.18-14.36 14.080z\" />;\n\n<Component text=\"Text\n  with\n  newlines\">Content</Component>;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<path\n  d=\"M4.765 16.829l3.069-2.946 5.813 5.748\n  11.33-11.232 3.006 3.18-14.36 14.080z\"\n/>;\n\n<Component\n  text=\"Text\n  with\n  newlines\">\n  Content\n</Component>;");
}
