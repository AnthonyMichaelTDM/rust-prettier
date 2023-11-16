#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_colons_after_substitutions_js_format_1_305ed33b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Icon = styled.div\\`\n  flex: none;\n  transition:    fill 0.25s;\n  width: 48px;\n  height: 48px;\n\n  \\${Link}:hover {\n    fill:   rebeccapurple;\n  }\n\n  \\${Link} :hover {\n    fill: yellow;\n  }\n\n  \\${media.smallDown}::before {}\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Icon = styled.div\\`\n  flex: none;\n  transition: fill 0.25s;\n  width: 48px;\n  height: 48px;\n\n  \\${Link}:hover {\n    fill: rebeccapurple;\n  }\n\n  \\${Link} :hover {\n    fill: yellow;\n  }\n\n  \\${media.smallDown}::before {\n  }\n\\`;");
}
#[test]
fn test_colons_after_substitutions_2_js_format_1_aa592faf() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Icon = styled.div\\`\n  height: 48px;\n\n  \\${Link}:nth-child(2) {\n    fill: rebeccapurple;\n  }\n\\`;\n\nconst Icon2 = styled.div\\`\n  height: 48px;\n\n  \\${Link}:empty:before{\n    fill: rebeccapurple;\n  }\n\\`;\n\nconst Icon3 = styled.div\\`\n  height: 48px;\n\n  \\${Link}:not(:first-child) {\n    fill: rebeccapurple;\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Icon = styled.div\\`\n  height: 48px;\n\n  \\${Link}:nth-child(2) {\n    fill: rebeccapurple;\n  }\n\\`;\n\nconst Icon2 = styled.div\\`\n  height: 48px;\n\n  \\${Link}:empty:before {\n    fill: rebeccapurple;\n  }\n\\`;\n\nconst Icon3 = styled.div\\`\n  height: 48px;\n\n  \\${Link}:not(:first-child) {\n    fill: rebeccapurple;\n  }\n\\`;");
}
#[test]
fn test_issue_2636_js_format_1_5492a56c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const ButtonWrapper = styled.button\\`\n  \\${base}\n  \\${hover}\n  \\${opaque}\n  \\${block}\n  \\${active}\n  \\${disabled}\n  \\${outline}\n  \\${dashed}\n  \\${spacing}\n\\`;\n\nexport const ButtonWrapper2 = styled.button\\`\n  \\${base} \\${hover} \\${opaque} \\${block} \\${active} \\${disabled} \\${outline} \\${dashed} \\${spacing}\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const ButtonWrapper = styled.button\\`\n  \\${base}\n  \\${hover}\n  \\${opaque}\n  \\${block}\n  \\${active}\n  \\${disabled}\n  \\${outline}\n  \\${dashed}\n  \\${spacing}\n\\`;\n\nexport const ButtonWrapper2 = styled.button\\`\n  \\${base} \\${hover} \\${opaque} \\${block} \\${active} \\${disabled} \\${outline} \\${dashed} \\${spacing}\n\\`;");
}
#[test]
fn test_issue_2883_js_format_1_8793694b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const foo = css\\`\n&.foo .\\${bar}::before,&.foo[value=\"hello\"] .\\${bar}::before {\n\tposition: absolute;\n}\n\\`;\n\nexport const foo2 = css\\`\na.\\${bar}:focus,a.\\${bar}:hover {\n  color: red;\n}\n\\`;\n\nexport const global = css\\`\nbutton.\\${foo}.\\${bar} {\n  color: #fff;\n}\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const foo = css\\`\n  &.foo .\\${bar}::before,&.foo[value=\"hello\"] .\\${bar}::before {\n    position: absolute;\n  }\n\\`;\n\nexport const foo2 = css\\`\n  a.\\${bar}:focus,a.\\${bar}:hover {\n    color: red;\n  }\n\\`;\n\nexport const global = css\\`\n  button.\\${foo}.\\${bar} {\n    color: #fff;\n  }\n\\`;");
}
#[test]
fn test_issue_5697_js_format_1_90263308() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const StyledH1 = styled.div\\`\n  font-size: 2.5em;\n  font-weight: \\${(props) => (props.strong ? 500 : 100)};\n  font-family: \\${constants.text.displayFont.fontFamily};\n  letter-spacing: \\${(props) => (props.light ? '0.04em' : 0)};\n  color: \\${(props) => props.textColor};\n  \\${(props) =>\n    props.center\n      ? \\` display: flex;\n                align-items: center;\n                justify-content: center;\n                text-align: center;\\`\n      : ''}\n  @media (max-width: \\${(props) => (props.noBreakPoint ? '0' : constants.layout.breakpoint.break1)}px) {\n    font-size: 2em;\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const StyledH1 = styled.div\\`\n  font-size: 2.5em;\n  font-weight: \\${(props) => (props.strong ? 500 : 100)};\n  font-family: \\${constants.text.displayFont.fontFamily};\n  letter-spacing: \\${(props) => (props.light ? \"0.04em\" : 0)};\n  color: \\${(props) => props.textColor};\n  \\${(props) =>\n    props.center\n      ? \\` display: flex;\n                align-items: center;\n                justify-content: center;\n                text-align: center;\\`\n      : \"\"}\n  @media (max-width: \\${(props) =>\n    props.noBreakPoint ? \"0\" : constants.layout.breakpoint.break1}px) {\n    font-size: 2em;\n  }\n\\`;");
}
#[test]
fn test_issue_5961_js_format_1_e15a20ac() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Steps = styled.div\\`\n  @media (min-width: 1px) {\n    \\${Step}:nth-child(odd) {}\n  }\n\\`;\n\nconst Steps2 = styled.div\\`\n  @media (min-width: \\${breakpoints.lg}) {\n    \\${Step} {\n      margin-bottom: 90px;\n    }\n\n    \\${Step}:nth-child(odd) {\n      \\${StepItemDescription} {\n        grid-row: 1;\n        grid-column: 3 / span 3;\n      }\n      \\${Image} {\n        grid-row: 1;\n        grid-column: 7 / span 6;\n      }\n    }\n\n    \\${Step}:nth-child(even) {\n      \\${Image} {\n        grid-row: 1;\n        grid-column: 3 / span 6;\n      }\n      \\${StepItemDescription} {\n        grid-row: 1;\n        grid-column: 10 / span 3;\n      }\n    }\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Steps = styled.div\\`\n  @media (min-width: 1px) {\n    \\${Step}:nth-child(odd) {\n    }\n  }\n\\`;\n\nconst Steps2 = styled.div\\`\n  @media (min-width: \\${breakpoints.lg}) {\n    \\${Step} {\n      margin-bottom: 90px;\n    }\n\n    \\${Step}:nth-child(odd) {\n      \\${StepItemDescription} {\n        grid-row: 1;\n        grid-column: 3 / span 3;\n      }\n      \\${Image} {\n        grid-row: 1;\n        grid-column: 7 / span 6;\n      }\n    }\n\n    \\${Step}:nth-child(even) {\n      \\${Image} {\n        grid-row: 1;\n        grid-column: 3 / span 6;\n      }\n      \\${StepItemDescription} {\n        grid-row: 1;\n        grid-column: 10 / span 3;\n      }\n    }\n  }\n\\`;");
}
#[test]
fn test_issue_6259_js_format_1_4c7963cc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const Group = styled.div\\`\n  margin: 0;\n\n  .input {\n    margin: 0;\n  }\n\n  \\${StyledInput}:not(:first-child) {\n    margin: 0;\n  }\n\n  & > :not(.\\${inputWrap}):not(\\${Button}) {\n    display: flex;\n  }\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const Group = styled.div\\`\n  margin: 0;\n\n  .input {\n    margin: 0;\n  }\n\n  \\${StyledInput}:not(:first-child) {\n    margin: 0;\n  }\n\n  & > :not(.\\${inputWrap}):not(\\${Button}) {\n    display: flex;\n  }\n\\`;");
}
#[test]
fn test_issue_8352_js_format_1_40748966() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const StyledComponent = styled.div\\`\n  margin-right: -4px;\n\n  \\${Container}.isExpanded & {\n    transform: rotate(-180deg);\n  }\n\\`;\n\nconst StyledComponent2 = styled.div\\`\n  margin-right: -4px;\n\n  \\${abc}.camelCase + \\${def}.camelCase & {\n    transform: rotate(-180deg);\n  }\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const StyledComponent = styled.div\\`\n  margin-right: -4px;\n\n  \\${Container}.isExpanded & {\n    transform: rotate(-180deg);\n  }\n\\`;\n\nconst StyledComponent2 = styled.div\\`\n  margin-right: -4px;\n\n  \\${abc}.camelCase + \\${def}.camelCase & {\n    transform: rotate(-180deg);\n  }\n\\`;");
}
#[test]
fn test_issue_9072_js_format_1_02207beb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const style1 = css\\`\n  width:\\${size+10}\\${sizeUnit};\n  border:\\${size/10} \\${sizeUnit} solid \\${color};\n\\`;\n\nconst style2 = css\\`\n  width: \\${size + 10}\\${sizeUnit};\n  border: \\${size / 10} \\${sizeUnit} solid \\${color};\n\\`;\n\nconst style3 = css\\`\n  foo: \\${foo}\\${bar}       \\${baz};\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const style1 = css\\`\n  width: \\${size + 10}\\${sizeUnit};\n  border: \\${size / 10} \\${sizeUnit} solid \\${color};\n\\`;\n\nconst style2 = css\\`\n  width: \\${size + 10}\\${sizeUnit};\n  border: \\${size / 10} \\${sizeUnit} solid \\${color};\n\\`;\n\nconst style3 = css\\`\n  foo: \\${foo}\\${bar} \\${baz};\n\\`;");
}
#[test]
fn test_issue_11797_js_format_1_bed2a6f1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const paragraph1 = css\\`\n  font-size: 12px;\n  transform: \\${vert ? 'translateY' : 'translateX'}(\\${translation + handleOffset}px);\n\\`;\n\nconst paragraph2 = css\\`\n  transform: \\${expr}(30px);\n\\`;\n\nconst paragraph3 = css\\`\n  transform: \\${expr} (30px);\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const paragraph1 = css\\`\n  font-size: 12px;\n  transform: \\${vert ? \"translateY\" : \"translateX\"}\n    (\\${translation + handleOffset}px);\n\\`;\n\nconst paragraph2 = css\\`\n  transform: \\${expr}(30px);\n\\`;\n\nconst paragraph3 = css\\`\n  transform: \\${expr} (30px);\n\\`;");
}
#[test]
fn test_styled_components_js_format_1_516081b1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const ListItem1 = styled.li\\`\\`;\n\nconst ListItem2 = styled.li\\` \\`;\n\nconst Dropdown = styled.div\\`position: relative;\\`\n\nconst Button = styled.button\\`\n\t  color:   palevioletred ;\n\n\tfont-size : 1em   ;\n\\`;\n\nconst TomatoButton = Button.extend\\`\n\tcolor  : tomato  ;\n\nborder-color : tomato\n    ;\n\n\\`;\n\nButton.extend.attr({})\\`\nborder-color : black;\n\\`\n\nstyled(ExistingComponent)\\`\n       color : papayawhip ; background-color: firebrick\\`;\n\n\nstyled.button.attr({})\\`\nborder : rebeccapurple\\`;\n\nstyled(ExistingComponent).attr({})\\`\nborder : rebeccapurple\\`;\n\nstyled.div\\`\n  color: \\${props => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${props => props.small ? 'font-size: 0.8em;' : ''};\n\\`\n\nstyled.div\\`\n  color: \\${props => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${props => props.small ? 'font-size: 0.8em;' : ''}\n\\`\n\nstyled.div\\`\n   /* prettier-ignore */\n  color: \\${props => props.theme.colors.paragraph};\n  \\${props => props.small ? 'font-size: 0.8em;' : ''};\n\\`\n\nstyled.div\\`\n  color: \\${props => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${props => props.small ? 'font-size: 0.8em;' : ''};\n  /* prettier-ignore */\n  \\${props => props.red ? 'color: red;' : ''};\n\\`\n\nstyled.div\\`\n  /* prettier-ignore */\n  color: \\${props => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${props => props.small ? 'font-size: 0.8em;' : ''};\n  /* prettier-ignore */\n  \\${props => props.red ? 'color: red;' : ''};\n  /* prettier-ignore */\n\\`\n\nstyled.div\\`\n \\${sanitize} \\${fonts}\n  html {\n    margin: 0;\n  }\n\\`\n\nstyled.div\\`\n  \\${bar}\n  baz\n\\`\n\nstyled.span\\`\n  foo\n  \\${bar}\n  baz\n\\`\n\nstyled.div\\`\n  foo\n  \\${bar}\n  \\${baz}\n\\`\n\nstyled.span\\`\n  \\${foo}\n  \\${bar}\n\\`\n\nstyled.div\\`\n  \\${foo} bar\n\\`\n\nstyled.span\\`\n  \\${foo} \\${bar}\n  baz: \\${foo}\n\\`\n\nstyled.span\\`\n\\${foo};\n\\${bar};\n\\`\n\nstyled.span\\`\n\\${foo}: \\${bar};\n\\`\n\nstyled.span\\`\n\\${foo}: \\${bar}\n\\`\n\nstyled.span\\`\n\\${foo}:\n\\${bar}\n\\`\n\nstyled.span\\`\n\\${foo}:\n\\${bar};\n\\`\n\nstyled.a\\`\n  \\${feedbackCountBlockCss}\n  text-decoration: none;\n\n  \\${FeedbackCount} {\n    margin: 0;\n  }\n\\`\n\nconst StyledComponent1 = styled.div\\`\n  \\${anInterpolation}\n  /* a comment */\n\n  .aRule {\n    color: red\n  }\n\\`;\n\nconst StyledComponent2 = styled.div\\`\n  \\${anInterpolation}\n\n  /* a comment */\n\n  .aRule {\n    color: red\n  }\n\\`;\n\nconst Direction = styled.span\\`\n  \\${({ up }) => up && \\`color: \\${color.positive};\\`}\n  \\${({ down }) => down && \\`color: \\${color.negative};\\`}\n\\`;\n\nconst Direction2 = styled.span\\`\n  \\${({ up }) => up && \\`color: \\${color.positive}\\`};\n  \\${({ down }) => down && \\`color: \\${color.negative}\\`};\n\\`;\n\nconst mixin = css\\`\n  color: \\${props => props.color};\n  \\${props => props.otherProperty}: \\${props => props.otherValue};\n\\`;\n\nconst foo = styled.div\\`\n  display: flex;\n  \\${props => props.useMixin && mixin}\n\\`;\n\nconst Single1 = styled.div\\`\n  color: red\n\\`;\n\nconst Single2 = styled.div\\`\n  color: red;\n\\`;\n\nconst Dropdown2 = styled.div\\`\n  /* A comment to avoid the prettier issue: https://github.com/prettier/prettier/issues/2291 */\n  position: relative;\n\\`;\n\nconst bar = styled.div\\`\n  border-radius: 50%;\n  border: 5px solid rgba(var(--green-rgb), 0);\n  display: inline-block;\n  height: 40px;\n  width: 40px;\n\n  \\${props =>\n    (props.complete || props.inProgress) &&\n    css\\`\n      border-color: rgba(var(--green-rgb), 0.15);\n    \\`}\n\n  div {\n    background-color: var(--purpleTT);\n    border-radius: 50%;\n    border: 4px solid rgba(var(--purple-rgb), 0.2);\n    color: var(--purpleTT);\n    display: inline-flex;\n\n    \\${props =>\n    props.complete &&\n    css\\`\n        background-color: var(--green);\n        border-width: 7px;\n      \\`}\n\n    \\${props =>\n    (props.complete || props.inProgress) &&\n    css\\`\n        border-color: var(--green);\n      \\`}\n  }\n\\`;\n\nconst A = styled.a\\`\n  display: inline-block;\n  color: #fff;\n  \\${props => props.a &&css\\`\n    display: none;\n  \\`}\n   height: 30px;\n\\`;\n\nconst Foo = styled.p\\`\n  max-width: 980px;\n  \\${mediaBreakpointOnlyXs\\`\n    && {\n      font-size: 0.8rem;\n    }\n  \\`}\n\n  &.bottom {\n    margin-top: 3rem;\n  }\n\\`;\n\nstyled(A)\\`\n  // prettier-ignore\n  @media (aaaaaaaaaaaaa) {\n\tz-index: \\${(props) => (props.isComplete ? '1' : '0')};\n  }\n\\`;\n\nconst StyledDiv = styled.div\\`\n  \\${props => getSize(props.$size.xs)}\n  \\${props => getSize(props.$size.sm, 'sm')}\n  \\${props => getSize(props.$size.md, 'md')}\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const ListItem1 = styled.li\\`\\`;\n\nconst ListItem2 = styled.li\\`\\`;\n\nconst Dropdown = styled.div\\`\n  position: relative;\n\\`;\n\nconst Button = styled.button\\`\n  color: palevioletred;\n\n  font-size: 1em;\n\\`;\n\nconst TomatoButton = Button.extend\\`\n  color: tomato;\n\n  border-color: tomato;\n\\`;\n\nButton.extend.attr({})\\`\n  border-color: black;\n\\`;\n\nstyled(ExistingComponent)\\`\n  color: papayawhip;\n  background-color: firebrick;\n\\`;\n\nstyled.button.attr({})\\`\n  border: rebeccapurple;\n\\`;\n\nstyled(ExistingComponent).attr({})\\`\n  border: rebeccapurple;\n\\`;\n\nstyled.div\\`\n  color: \\${(props) => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${(props) => (props.small ? \"font-size: 0.8em;\" : \"\")};\n\\`;\n\nstyled.div\\`\n  color: \\${(props) => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${(props) => (props.small ? \"font-size: 0.8em;\" : \"\")}\n\\`;\n\nstyled.div\\`\n  /* prettier-ignore */\n  color: \\${(props) => props.theme.colors.paragraph};\n  \\${(props) => (props.small ? \"font-size: 0.8em;\" : \"\")};\n\\`;\n\nstyled.div\\`\n  color: \\${(props) => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${(props) => (props.small ? \"font-size: 0.8em;\" : \"\")};\n  /* prettier-ignore */\n  \\${(props) => (props.red ? \"color: red;\" : \"\")};\n\\`;\n\nstyled.div\\`\n  /* prettier-ignore */\n  color: \\${(props) => props.theme.colors.paragraph};\n  /* prettier-ignore */\n  \\${(props) => (props.small ? \"font-size: 0.8em;\" : \"\")};\n  /* prettier-ignore */\n  \\${(props) => (props.red ? \"color: red;\" : \"\")};\n  /* prettier-ignore */\n\\`;\n\nstyled.div\\`\n  \\${sanitize} \\${fonts}\n  html {\n    margin: 0;\n  }\n\\`;\n\nstyled.div\\`\n  \\${bar}\n  baz\n\\`;\n\nstyled.span\\`\n  foo\n  \\${bar}\n  baz\n\\`;\n\nstyled.div\\`\n  foo\n  \\${bar}\n  \\${baz}\n\\`;\n\nstyled.span\\`\n  \\${foo}\n  \\${bar}\n\\`;\n\nstyled.div\\`\n  \\${foo} bar\n\\`;\n\nstyled.span\\`\n  \\${foo} \\${bar}\n  baz: \\${foo}\n\\`;\n\nstyled.span\\`\n  \\${foo};\n  \\${bar};\n\\`;\n\nstyled.span\\`\n  \\${foo}: \\${bar};\n\\`;\n\nstyled.span\\`\n  \\${foo}: \\${bar}\n\\`;\n\nstyled.span\\`\n  \\${foo}: \\${bar}\n\\`;\n\nstyled.span\\`\n  \\${foo}: \\${bar};\n\\`;\n\nstyled.a\\`\n  \\${feedbackCountBlockCss}\n  text-decoration: none;\n\n  \\${FeedbackCount} {\n    margin: 0;\n  }\n\\`;\n\nconst StyledComponent1 = styled.div\\`\n  \\${anInterpolation}\n  /* a comment */\n\n  .aRule {\n    color: red;\n  }\n\\`;\n\nconst StyledComponent2 = styled.div\\`\n  \\${anInterpolation}\n\n  /* a comment */\n\n  .aRule {\n    color: red;\n  }\n\\`;\n\nconst Direction = styled.span\\`\n  \\${({ up }) => up && \\`color: \\${color.positive};\\`}\n  \\${({ down }) => down && \\`color: \\${color.negative};\\`}\n\\`;\n\nconst Direction2 = styled.span\\`\n  \\${({ up }) => up && \\`color: \\${color.positive}\\`};\n  \\${({ down }) => down && \\`color: \\${color.negative}\\`};\n\\`;\n\nconst mixin = css\\`\n  color: \\${(props) => props.color};\n  \\${(props) => props.otherProperty}: \\${(props) => props.otherValue};\n\\`;\n\nconst foo = styled.div\\`\n  display: flex;\n  \\${(props) => props.useMixin && mixin}\n\\`;\n\nconst Single1 = styled.div\\`\n  color: red;\n\\`;\n\nconst Single2 = styled.div\\`\n  color: red;\n\\`;\n\nconst Dropdown2 = styled.div\\`\n  /* A comment to avoid the prettier issue: https://github.com/prettier/prettier/issues/2291 */\n  position: relative;\n\\`;\n\nconst bar = styled.div\\`\n  border-radius: 50%;\n  border: 5px solid rgba(var(--green-rgb), 0);\n  display: inline-block;\n  height: 40px;\n  width: 40px;\n\n  \\${(props) =>\n    (props.complete || props.inProgress) &&\n    css\\`\n      border-color: rgba(var(--green-rgb), 0.15);\n    \\`}\n\n  div {\n    background-color: var(--purpleTT);\n    border-radius: 50%;\n    border: 4px solid rgba(var(--purple-rgb), 0.2);\n    color: var(--purpleTT);\n    display: inline-flex;\n\n    \\${(props) =>\n      props.complete &&\n      css\\`\n        background-color: var(--green);\n        border-width: 7px;\n      \\`}\n\n    \\${(props) =>\n      (props.complete || props.inProgress) &&\n      css\\`\n        border-color: var(--green);\n      \\`}\n  }\n\\`;\n\nconst A = styled.a\\`\n  display: inline-block;\n  color: #fff;\n  \\${(props) =>\n    props.a &&\n    css\\`\n      display: none;\n    \\`}\n  height: 30px;\n\\`;\n\nconst Foo = styled.p\\`\n  max-width: 980px;\n  \\${mediaBreakpointOnlyXs\\`\n    && {\n      font-size: 0.8rem;\n    }\n  \\`}\n\n  &.bottom {\n    margin-top: 3rem;\n  }\n\\`;\n\nstyled(A)\\`\n  // prettier-ignore\n  @media (aaaaaaaaaaaaa) {\n\tz-index: \\${(props) => (props.isComplete ? \"1\" : \"0\")};\n  }\n\\`;\n\nconst StyledDiv = styled.div\\`\n  \\${(props) => getSize(props.$size.xs)}\n  \\${(props) => getSize(props.$size.sm, \"sm\")}\n  \\${(props) => getSize(props.$size.md, \"md\")}\n\\`;");
}
#[test]
fn test_styled_components_multiple_expressions_js_format_1_d544e68f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Header = styled.div\\`\n  \\${something()}\n  & > \\${Child}:not(:first-child) {\nmargin-left:5px;\n}\n\\`\n\nconst Header2 = styled.div\\`\n  \\${something()}\n  & > \\${Child}\\${Child2}:not(:first-child) {\nmargin-left:5px;\n}\n\\`\n\nstyled.div\\`\\${foo}-idle { }\\`\n\nstyled.div\\`\\${foo}-0-idle { }\\`\n\nstyled.div\\`\nfont-family: \"\\${a}\", \"\\${b}\";\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Header = styled.div\\`\n  \\${something()}\n  & > \\${Child}:not(:first-child) {\n    margin-left: 5px;\n  }\n\\`;\n\nconst Header2 = styled.div\\`\n  \\${something()}\n  & > \\${Child}\\${Child2}:not(:first-child) {\n    margin-left: 5px;\n  }\n\\`;\n\nstyled.div\\`\n  \\${foo}-idle {\n  }\n\\`;\n\nstyled.div\\`\n  \\${foo}-0-idle {\n  }\n\\`;\n\nstyled.div\\`\n  font-family: \"\\${a}\", \"\\${b}\";\n\\`;");
}
#[test]
fn test_url_js_format_1_8adae460() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("styled.div\\`color:red;background: url(http://example.com?q=\\${foo})\\`");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "styled.div\\`\n  color: red;\n  background: url(http://example.com?q=\\${foo});\n\\`;"
    );
}
#[test]
fn test_var_js_format_1_14aa88cc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Something = styled.div\\`\n  background: var(--\\${one}); /* ... */\n  border: 1px solid var(--\\${two}); /* ... */\n\\`;\n\nconst StyledPurchaseCard = styled(Card)\\`\n  min-width: 200px;\n  background-color: var(--\\${props => props.color});\n  color: #fff;\n\\`;\n\nconst v1 =  css\\`\nprop: var(--global--color--\\${props.variant});\n\\`;\n\nconst v2 = css\\`\n        background-color: var(--global--color--\\${props.variant});\n\n        &:hover {\n          background-color: var(--global--color--\\${props.variant}__one);\n        }\n      \\`\n\nexport const StyledComponent = styled.div\\`\n  grid-area:  area-\\${props => props.propName};\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Something = styled.div\\`\n  background: var(--\\${one}); /* ... */\n  border: 1px solid var(--\\${two}); /* ... */\n\\`;\n\nconst StyledPurchaseCard = styled(Card)\\`\n  min-width: 200px;\n  background-color: var(--\\${(props) => props.color});\n  color: #fff;\n\\`;\n\nconst v1 = css\\`\n  prop: var(--global--color--\\${props.variant});\n\\`;\n\nconst v2 = css\\`\n  background-color: var(--global--color--\\${props.variant});\n\n  &:hover {\n    background-color: var(--global--color--\\${props.variant}__one);\n  }\n\\`;\n\nexport const StyledComponent = styled.div\\`\n  grid-area: area-\\${(props) => props.propName};\n\\`;");
}
