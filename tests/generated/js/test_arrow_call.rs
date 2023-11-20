#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_call_js_arrow_parensalways_format_1_c84100cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const testResults = results.testResults.map(testResult =>\n  formatResult(testResult, formatter, reporter)\n);\n\nit('mocks regexp instances', () => {\n  expect(\n    () => moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint\" }))\n  .not.toThrowError();\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" }))\n  .not.toThrowError();\n\nconst a = Observable\n  .fromPromise(axiosInstance.post('/carts/mine'))\n  .map((response) => response.data)\n\nconst b = Observable.fromPromise(axiosInstance.get(url))\n  .map((response) => response.data)\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  veryLooooooooooooooooooooooooongName =>\n    veryLoooooooooooooooongName.something()\n);\n\npromise.then(result => result.veryLongVariable.veryLongPropertyName > someOtherVariable ? \"ok\" : \"fail\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const testResults = results.testResults.map((testResult) =>\n  formatResult(testResult, formatter, reporter),\n);\n\nit(\"mocks regexp instances\", () => {\n  expect(() =>\n    moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" })).toThrowError(\n  /Required parameter/,\n);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" }),\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" }),\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint\" }),\n).not.toThrowError();\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" }),\n).not.toThrowError();\n\nconst a = Observable.fromPromise(axiosInstance.post(\"/carts/mine\")).map(\n  (response) => response.data,\n);\n\nconst b = Observable.fromPromise(axiosInstance.get(url)).map(\n  (response) => response.data,\n);\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  (veryLooooooooooooooooooooooooongName) =>\n    veryLoooooooooooooooongName.something(),\n);\n\npromise.then((result) =>\n  result.veryLongVariable.veryLongPropertyName > someOtherVariable\n    ? \"ok\"\n    : \"fail\",\n);");
}
#[test]
fn test_arrow_call_js_trailing_commaall_format_1_c84100cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const testResults = results.testResults.map(testResult =>\n  formatResult(testResult, formatter, reporter)\n);\n\nit('mocks regexp instances', () => {\n  expect(\n    () => moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint\" }))\n  .not.toThrowError();\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" }))\n  .not.toThrowError();\n\nconst a = Observable\n  .fromPromise(axiosInstance.post('/carts/mine'))\n  .map((response) => response.data)\n\nconst b = Observable.fromPromise(axiosInstance.get(url))\n  .map((response) => response.data)\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  veryLooooooooooooooooooooooooongName =>\n    veryLoooooooooooooooongName.something()\n);\n\npromise.then(result => result.veryLongVariable.veryLongPropertyName > someOtherVariable ? \"ok\" : \"fail\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const testResults = results.testResults.map((testResult) =>\n  formatResult(testResult, formatter, reporter),\n);\n\nit(\"mocks regexp instances\", () => {\n  expect(() =>\n    moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" })).toThrowError(\n  /Required parameter/,\n);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" }),\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" }),\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint\" }),\n).not.toThrowError();\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" }),\n).not.toThrowError();\n\nconst a = Observable.fromPromise(axiosInstance.post(\"/carts/mine\")).map(\n  (response) => response.data,\n);\n\nconst b = Observable.fromPromise(axiosInstance.get(url)).map(\n  (response) => response.data,\n);\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  (veryLooooooooooooooooooooooooongName) =>\n    veryLoooooooooooooooongName.something(),\n);\n\npromise.then((result) =>\n  result.veryLongVariable.veryLongPropertyName > someOtherVariable\n    ? \"ok\"\n    : \"fail\",\n);");
}
#[test]
fn test_arrow_call_js_trailing_commaes_5_format_1_c84100cf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const testResults = results.testResults.map(testResult =>\n  formatResult(testResult, formatter, reporter)\n);\n\nit('mocks regexp instances', () => {\n  expect(\n    () => moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" }))\n  .toThrowError(/Required parameter/);\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint\" }))\n  .not.toThrowError();\n\nexpect(() => asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" }))\n  .not.toThrowError();\n\nconst a = Observable\n  .fromPromise(axiosInstance.post('/carts/mine'))\n  .map((response) => response.data)\n\nconst b = Observable.fromPromise(axiosInstance.get(url))\n  .map((response) => response.data)\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  veryLooooooooooooooooooooooooongName =>\n    veryLoooooooooooooooongName.something()\n);\n\npromise.then(result => result.veryLongVariable.veryLongPropertyName > someOtherVariable ? \"ok\" : \"fail\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const testResults = results.testResults.map((testResult) =>\n  formatResult(testResult, formatter, reporter)\n);\n\nit(\"mocks regexp instances\", () => {\n  expect(() =>\n    moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/))\n  ).not.toThrow();\n});\n\nexpect(() => asyncRequest({ url: \"/test-endpoint\" })).toThrowError(\n  /Required parameter/\n);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-long-url\" })\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ url: \"/test-endpoint-but-with-a-suuuuuuuuper-long-url\" })\n).toThrowError(/Required parameter/);\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint\" })\n).not.toThrowError();\n\nexpect(() =>\n  asyncRequest({ type: \"foo\", url: \"/test-endpoint-but-with-a-long-url\" })\n).not.toThrowError();\n\nconst a = Observable.fromPromise(axiosInstance.post(\"/carts/mine\")).map(\n  (response) => response.data\n);\n\nconst b = Observable.fromPromise(axiosInstance.get(url)).map(\n  (response) => response.data\n);\n\nfunc(\n  veryLoooooooooooooooooooooooongName,\n  (veryLooooooooooooooooooooooooongName) =>\n    veryLoooooooooooooooongName.something()\n);\n\npromise.then((result) =>\n  result.veryLongVariable.veryLongPropertyName > someOtherVariable\n    ? \"ok\"\n    : \"fail\"\n);");
}
#[test]
fn test_class_property_js_arrow_parensalways_format_1_af0c3ec6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .arrow_parens("always")
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };");
}
#[test]
fn test_class_property_js_trailing_commaall_format_1_af0c3ec6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };");
}
#[test]
fn test_class_property_js_trailing_commaes_5_format_1_af0c3ec6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const composition = (ViewComponent, ContainerComponent) =>\n  class extends React.Component {\n    static propTypes = {};\n  };");
}
