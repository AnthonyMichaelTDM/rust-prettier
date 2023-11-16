#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_type_arguments_ts_semifalse_format_1_fa10f631() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #6988\n\n// functional component with ugly linebreak\nexport const MyLongNamedReactFunctionalComponent1: FunctionComponent<ALongNamedInterface1 & ALongNamedInterface2> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent2: FunctionComponent<ALongNamedInterface1 | ALongNamedInterface2> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent3: FunctionComponent<ALongNamedInterface1, ALongNamedInterface2> = (props) => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #6988\n\n// functional component with ugly linebreak\nexport const MyLongNamedReactFunctionalComponent1: FunctionComponent<\n  ALongNamedInterface1 & ALongNamedInterface2\n> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent2: FunctionComponent<\n  ALongNamedInterface1 | ALongNamedInterface2\n> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent3: FunctionComponent<\n  ALongNamedInterface1,\n  ALongNamedInterface2\n> = (props) => {}");
}
#[test]
fn test_type_arguments_ts_format_1_fa10f631() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #6988\n\n// functional component with ugly linebreak\nexport const MyLongNamedReactFunctionalComponent1: FunctionComponent<ALongNamedInterface1 & ALongNamedInterface2> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent2: FunctionComponent<ALongNamedInterface1 | ALongNamedInterface2> = (props) => {}\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent3: FunctionComponent<ALongNamedInterface1, ALongNamedInterface2> = (props) => {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #6988\n\n// functional component with ugly linebreak\nexport const MyLongNamedReactFunctionalComponent1: FunctionComponent<\n  ALongNamedInterface1 & ALongNamedInterface2\n> = (props) => {};\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent2: FunctionComponent<\n  ALongNamedInterface1 | ALongNamedInterface2\n> = (props) => {};\n\n// functional component with valid linebreak\nexport const MyLongNamedReactFunctionalComponent3: FunctionComponent<\n  ALongNamedInterface1,\n  ALongNamedInterface2\n> = (props) => {};");
}
