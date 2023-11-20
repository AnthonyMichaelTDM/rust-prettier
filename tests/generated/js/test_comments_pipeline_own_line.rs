#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_pipeline_own_line_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_pipeline_own_line_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_pipeline_own_line_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_pipeline_own_line_js_format_1_6ec95ed5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function pipeline() {\n\t0\n\t// Comment\n\t|> x\n}\n\nbifornCringerMoshedPerplexSawder(\n  askTrovenaBeenaDependsRowans,\n  glimseGlyphsHazardNoopsTieTie,\n  averredBathersBoxroomBuggyNurl\n) // comment\n|> kochabCooieGameOnOboleUnweave\n|> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder(\n  askTrovenaBeenaDependsRowans,\n  glimseGlyphsHazardNoopsTieTie\n)\n|> foo // comment\n|> kochabCooieGameOnOboleUnweave\n|> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder[\n  askTrovenaBeenaDependsRowans +\n  glimseGlyphsHazardNoopsTieTie +\n  averredBathersBoxroomBuggyNurl\n] // comment\n|> kochabCooieGameOnOboleUnweave\n|> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder[\n  askTrovenaBeenaDependsRowans +\n  glimseGlyphsHazardNoopsTieTie +\n  averredBathersBoxroomBuggyNurl\n]\n|> foo // comment\n|> kochabCooieGameOnOboleUnweave\n|> glimseGlyphsHazardNoopsTieTie;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function pipeline() {\n  0 |>\n    // Comment\n    x;\n}\n\nbifornCringerMoshedPerplexSawder(\n  askTrovenaBeenaDependsRowans,\n  glimseGlyphsHazardNoopsTieTie,\n  averredBathersBoxroomBuggyNurl,\n) // comment\n  |> kochabCooieGameOnOboleUnweave\n  |> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder(\n  askTrovenaBeenaDependsRowans,\n  glimseGlyphsHazardNoopsTieTie,\n)\n  |> foo // comment\n  |> kochabCooieGameOnOboleUnweave\n  |> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder[\n  askTrovenaBeenaDependsRowans +\n    glimseGlyphsHazardNoopsTieTie +\n    averredBathersBoxroomBuggyNurl\n] // comment\n  |> kochabCooieGameOnOboleUnweave\n  |> glimseGlyphsHazardNoopsTieTie;\n\nbifornCringerMoshedPerplexSawder[\n  askTrovenaBeenaDependsRowans +\n    glimseGlyphsHazardNoopsTieTie +\n    averredBathersBoxroomBuggyNurl\n]\n  |> foo // comment\n  |> kochabCooieGameOnOboleUnweave\n  |> glimseGlyphsHazardNoopsTieTie;");
}
