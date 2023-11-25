#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_createclass_callsite_js_format_1_11c2d590() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar React = require('react');\nvar Hello = require('./createclass-module');\n\nvar HelloLocal = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n  },\n\n  render: function(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n});\n\nvar Callsite = React.createClass({\n  render: function(): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  }\n});\n\nmodule.exports = Callsite;") ? ;
    assert_eq ! (formatted , "/* @flow */\nvar React = require(\"react\");\nvar Hello = require(\"./createclass-module\");\n\nvar HelloLocal = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n  },\n\n  render: function (): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  },\n});\n\nvar Callsite = React.createClass({\n  render: function (): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  },\n});\n\nmodule.exports = Callsite;");
    Ok(())
}
#[test]
fn test_createclass_module_js_format_1_7e117fd5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nvar React = require('react');\n\nvar Hello = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n  },\n\n  render: function(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n});\n\nmodule.exports = Hello;") ? ;
    assert_eq ! (formatted , "/* @flow */\nvar React = require(\"react\");\n\nvar Hello = React.createClass({\n  propTypes: {\n    name: React.PropTypes.string.isRequired,\n  },\n\n  render: function (): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  },\n});\n\nmodule.exports = Hello;");
    Ok(())
}
#[test]
fn test_es_6_class_proptypes_callsite_js_format_1_7c18b6f7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nimport React from 'react';\nimport Hello from './es6class-proptypes-module';\n\nclass HelloLocal extends React.Component<void, {name: string}, void> {\n  defaultProps = {};\n  propTypes = {\n    name: React.PropTypes.string.isRequired,\n  };\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nclass Callsite extends React.Component<void, {}, void> {\n  render(): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  }\n}\n\nmodule.exports = Callsite;") ? ;
    assert_eq ! (formatted , "/* @flow */\nimport React from \"react\";\nimport Hello from \"./es6class-proptypes-module\";\n\nclass HelloLocal extends React.Component<void, { name: string }, void> {\n  defaultProps = {};\n  propTypes = {\n    name: React.PropTypes.string.isRequired,\n  };\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nclass Callsite extends React.Component<void, {}, void> {\n  render(): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  }\n}\n\nmodule.exports = Callsite;");
    Ok(())
}
#[test]
fn test_es_6_class_proptypes_module_js_format_1_91c34460() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nimport React from 'react';\n\nclass Hello extends React.Component<void, {name: string}, void> {\n  defaultProps = {};\n  propTypes = {\n    name: React.PropTypes.string.isRequired,\n  };\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nmodule.exports = Hello;") ? ;
    assert_eq ! (formatted , "/* @flow */\nimport React from \"react\";\n\nclass Hello extends React.Component<void, { name: string }, void> {\n  defaultProps = {};\n  propTypes = {\n    name: React.PropTypes.string.isRequired,\n  };\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nmodule.exports = Hello;");
    Ok(())
}
#[test]
fn test_es_6_class_types_callsite_js_format_1_4dc21221() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nimport React from 'react';\nimport Hello from './es6class-types-module';\n\ntype Props = {name: string};\n\nclass HelloLocal extends React.Component<void, Props, void> {\n  props: Props;\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nclass Callsite extends React.Component<void, Props, void> {\n  render(): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  }\n}\n\nmodule.exports = Callsite;") ? ;
    assert_eq ! (formatted , "/* @flow */\nimport React from \"react\";\nimport Hello from \"./es6class-types-module\";\n\ntype Props = { name: string };\n\nclass HelloLocal extends React.Component<void, Props, void> {\n  props: Props;\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nclass Callsite extends React.Component<void, Props, void> {\n  render(): React.Element<*> {\n    return (\n      <div>\n        <Hello />\n        <HelloLocal />\n      </div>\n    );\n  }\n}\n\nmodule.exports = Callsite;");
    Ok(())
}
#[test]
fn test_es_6_class_types_module_js_format_1_c15d9078() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nimport React from 'react';\n\ntype Props = {name: string};\n\nclass Hello extends React.Component<{}, Props, void>{\n  props: Props;\n  static defaultProps: {};\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nmodule.exports = Hello;") ? ;
    assert_eq ! (formatted , "/* @flow */\nimport React from \"react\";\n\ntype Props = { name: string };\n\nclass Hello extends React.Component<{}, Props, void> {\n  props: Props;\n  static defaultProps: {};\n\n  render(): React.Element<*> {\n    return <div>{this.props.name}</div>;\n  }\n}\n\nmodule.exports = Hello;");
    Ok(())
}
