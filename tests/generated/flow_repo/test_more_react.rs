#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_api_react_js_format_1_a061eb99() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("react.js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nvar app = require('JSX');\n\napp.setProps({y:42}); // error, y:number but foo expects string in App.react\napp.setState({z:42}); // error, z:number but foo expects string in App.react\n\nfunction bar(x:number) { }\nbar(app.props.children); // No error, App doesn't specify propTypes so anything goes") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var app = require(\"JSX\");\n\napp.setProps({ y: 42 }); // error, y:number but foo expects string in App.react\napp.setState({ z: 42 }); // error, z:number but foo expects string in App.react\n\nfunction bar(x: number) {}\nbar(app.props.children); // No error, App doesn't specify propTypes so anything goes");
}
#[test]
fn test_app_react_js_format_1_8907865e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("react.js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/**\n * @providesModule App.react\n * @jsx React.DOM\n */\n\nvar React = require('react');\n\n// expect args to be strings\nfunction foo(p:string,q:string):string { return p+q; }\n\nvar App = React.createClass({\n\n  getDefaultProps: function(): { y: string } {\n    return {y:\"\"}; // infer props.y: string\n  },\n\n  getInitialState: function() {\n    return {z:0}; // infer state.z: number\n  },\n\n  handler: function() {\n    this.setState({z:42}); // ok\n  },\n\n  render: function() {\n    var x = this.props.x;\n    var y = this.props.y;\n    var z = this.state.z;\n\n    //this.state;\n\n    return (\n      <div>\n        {foo(x,y)}\n        {foo(z,x)} // error, since z: number\n      </div>\n    );\n  }\n\n});\n\nmodule.exports = App;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule App.react\n * @jsx React.DOM\n */\n\nvar React = require(\"react\");\n\n// expect args to be strings\nfunction foo(p: string, q: string): string {\n  return p + q;\n}\n\nvar App = React.createClass({\n  getDefaultProps: function (): { y: string } {\n    return { y: \"\" }; // infer props.y: string\n  },\n\n  getInitialState: function () {\n    return { z: 0 }; // infer state.z: number\n  },\n\n  handler: function () {\n    this.setState({ z: 42 }); // ok\n  },\n\n  render: function () {\n    var x = this.props.x;\n    var y = this.props.y;\n    var z = this.state.z;\n\n    //this.state;\n\n    return (\n      <div>\n        {foo(x, y)}\n        {foo(z, x)} // error, since z: number\n      </div>\n    );\n  },\n});\n\nmodule.exports = App;");
}
#[test]
fn test_initialized_fields_js_format_1_725d3e33() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule InitializedFields.react\n */\n\nvar React = require('react');\n\n/** This is a regression test for a bug where we forgot to mark the fields of\n * react classes as initialized, when the class was created with createClass().\n * This would manifest as complaining that metric requires an annotation */\nvar App = React.createClass({\n  metrics: [1,2,3],\n});\n\nmodule.exports = App;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule InitializedFields.react\n */\n\nvar React = require(\"react\");\n\n/** This is a regression test for a bug where we forgot to mark the fields of\n * react classes as initialized, when the class was created with createClass().\n * This would manifest as complaining that metric requires an annotation */\nvar App = React.createClass({\n  metrics: [1, 2, 3],\n});\n\nmodule.exports = App;");
}
#[test]
fn test_jsx_js_format_1_9cd2d6b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n/* @providesModule JSX */\n\nvar React = require('react');\nvar App = require('App.react');\n\nvar app =\n  <App y={42}> // error, y: number but foo expects string in App.react\n    Some text.\n  </App>;\n\nmodule.exports = app;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @providesModule JSX */\n\nvar React = require(\"react\");\nvar App = require(\"App.react\");\n\nvar app = (\n  <App y={42}>\n    {\" \"}\n    // error, y: number but foo expects string in App.react Some text.\n  </App>\n);\n\nmodule.exports = app;");
}
#[test]
fn test_check_prop_types_js_format_1_ffc87535() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nimport { PropTypes, checkPropTypes } from \"react\";\n\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }, 'value', 'TestComponent'); // OK\n\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }); // error: missing arguments\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }, 'value'); // error: missing argument\n\ncheckPropTypes({ bar: PropTypes.string }, { foo: 'foo' }, 'value', 'TestComponent'); // error: property not found\n\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }, 'value', 'TestComponent', () => 123); // error: number ~> string\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }, 'value', 'TestComponent', () => null); // OK\ncheckPropTypes({ foo: PropTypes.string }, { foo: 'foo' }, 'value', 'TestComponent', () => undefined); // OK") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nimport { PropTypes, checkPropTypes } from \"react\";\n\ncheckPropTypes(\n  { foo: PropTypes.string },\n  { foo: \"foo\" },\n  \"value\",\n  \"TestComponent\",\n); // OK\n\ncheckPropTypes({ foo: PropTypes.string }, { foo: \"foo\" }); // error: missing arguments\ncheckPropTypes({ foo: PropTypes.string }, { foo: \"foo\" }, \"value\"); // error: missing argument\n\ncheckPropTypes(\n  { bar: PropTypes.string },\n  { foo: \"foo\" },\n  \"value\",\n  \"TestComponent\",\n); // error: property not found\n\ncheckPropTypes(\n  { foo: PropTypes.string },\n  { foo: \"foo\" },\n  \"value\",\n  \"TestComponent\",\n  () => 123,\n); // error: number ~> string\ncheckPropTypes(\n  { foo: PropTypes.string },\n  { foo: \"foo\" },\n  \"value\",\n  \"TestComponent\",\n  () => null,\n); // OK\ncheckPropTypes(\n  { foo: PropTypes.string },\n  { foo: \"foo\" },\n  \"value\",\n  \"TestComponent\",\n  () => undefined,\n); // OK");
}
#[test]
fn test_prop_types_js_format_1_35255060() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\nvar C = React.createClass({\n  propTypes: {\n    title: React.PropTypes.string.isRequired,\n  }\n});\nvar D = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n    ...C.propTypes,\n  }\n});\n\n<D />; // errors: properties \\`name\\` and \\`title\\` not found") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\nvar C = React.createClass({\n  propTypes: {\n    title: React.PropTypes.string.isRequired,\n  },\n});\nvar D = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n    ...C.propTypes,\n  },\n});\n\n<D />; // errors: properties \\`name\\` and \\`title\\` not found");
}
