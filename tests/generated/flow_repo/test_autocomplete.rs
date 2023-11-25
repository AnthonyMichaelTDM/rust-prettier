use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_customfun_js_format_1_12ead5b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare var idx: $Facebookism$Idx;\ndeclare var merge: $Facebookism$Merge;\ndeclare var mergeDeepInto: $Facebookism$MergeDeepInto;\ndeclare var mergeInto: $Facebookism$MergeInto;\ndeclare var mixin: $Facebookism$Mixin;\ndeclare var objectGetPrototypeOf: Object$GetPrototypeOf\ndeclare var objectAssign: Object$Assign\n\nm") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare var idx: $Facebookism$Idx;\ndeclare var merge: $Facebookism$Merge;\ndeclare var mergeDeepInto: $Facebookism$MergeDeepInto;\ndeclare var mergeInto: $Facebookism$MergeInto;\ndeclare var mixin: $Facebookism$Mixin;\ndeclare var objectGetPrototypeOf: Object$GetPrototypeOf;\ndeclare var objectAssign: Object$Assign;\n\nm;");
}
#[test]
fn test_unknown_js_format_1_d5af9ad4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @noflow\nmodule.exports = { x: { y: \"\" } };");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @noflow\nmodule.exports = { x: { y: \"\" } };"
    );
}
