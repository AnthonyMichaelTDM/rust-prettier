#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_html_format_1_035236f6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<script></script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<script></script>");
}
#[test]
fn test_js_html_format_1_176c0134() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"text/javascript\">\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script type=\"application/javascript\">\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script>\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script type=\"text/babel\">\n            const    someJS    =   'this should be formatted'\n</script>\n<script type=\"module\">\n      import lib from './lib.js';\n  \n        function myFunction() { return 'foo'; }\n  </script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"text/javascript\">\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script type=\"application/javascript\">\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script>\n  var message = \"Alert!\";\n\n  alert(message);\n</script>\n<script type=\"text/babel\">\n  const someJS = \"this should be formatted\";\n</script>\n<script type=\"module\">\n  import lib from \"./lib.js\";\n\n  function myFunction() {\n    return \"foo\";\n  }\n</script>");
}
#[test]
fn test_simple_html_format_1_972833c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html>\n  <head>\n    <title>Sample styled page</title>\n    <script>alert('test');</script>\n    <script>\n      var message = \"Alert!\";\n\n      alert(message);\n    </script>\n  </head>\n  <body>\n    <h1>Sample styled page</h1>\n    <p>This page is just a demo.</p>\n  </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html>\n  <head>\n    <title>Sample styled page</title>\n    <script>\n      alert(\"test\");\n    </script>\n    <script>\n      var message = \"Alert!\";\n\n      alert(message);\n    </script>\n  </head>\n  <body>\n    <h1>Sample styled page</h1>\n    <p>This page is just a demo.</p>\n  </body>\n</html>");
}
#[test]
fn test_single_script_html_format_1_51caaeb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script>alert('test');</script>\n<script>\n  document.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script>\n  alert(\"test\");\n</script>\n<script>\n  document.getElementById(\"demo\").innerHTML = \"Hello JavaScript!\";\n</script>");
}
#[test]
fn test_something_else_html_format_1_e47fe3f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script type=\"text/template\">\n <div>\n    </div>\n</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script type=\"text/template\">\n  <div>\n     </div>\n</script>"
    );
}
#[test]
fn test_template_literal_html_format_1_e80685dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n    <head>\n    </head>\n    <body>\n        <script>\n            function foo() {\n                return \\`\n                    <div>\n                        <p>Text</p>\n                    </div>\n                \\`;\n            }\n        </script>\n    </body>\n</html>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head> </head>\n  <body>\n    <script>\n      function foo() {\n        return \\`\n                    <div>\n                        <p>Text</p>\n                    </div>\n                \\`;\n      }\n    </script>\n  </body>\n</html>");
}
#[test]
fn test_typescript_html_format_1_65fa3d79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"application/x-typescript\">\n  class Student {\n    fullName: string;\n    constructor(public firstName: string, public middleInitial: string, public lastName: string) {\n    this.fullName = firstName + \" \" + middleInitial + \" \" + lastName;\n  }\n  }\n\n  interface Person {\n    firstName: string;\n    lastName: string;\n  }\n\n  function greeter(person : Person) {\n    return \"Hello, \" + person.firstName + \" \" + person.lastName;\n  }\n\n  let user = new Student(\"Jane\", \"M.\", \"User\");\n\n  document.body.innerHTML = greeter(user);\n</script>\n<script lang=\"ts\">\n  class Student {\n    fullName: string;\n    constructor(public firstName: string, public middleInitial: string, public lastName: string) {\n    this.fullName = firstName + \" \" + middleInitial + \" \" + lastName;\n  }\n  }\n\n  interface Person {\n    firstName: string;\n    lastName: string;\n  }\n\n  function greeter(person : Person) {\n    return \"Hello, \" + person.firstName + \" \" + person.lastName;\n  }\n\n  let user = new Student(\"Jane\", \"M.\", \"User\");\n\n  document.body.innerHTML = greeter(user);\n</script>\n<script lang=\"tsx\">\n  class CommentBox extends React.Component<{ url: string, pollInterval: number}, CommentData> {\n    constructor(){\n      super()\n      this.state = { data: [] };\n    }\n    fetchComments() {\n      $.ajax({\n        url: this.props.url,\n        dataType: 'json',\n        cache: false,\n        success: (data) => this.setState({ data: data }),\n        error: (xhr, status, err) => console.error(status, err)\n      })\n    }\n    componentDidMount() {\n      this.fetchComments();\n      setInterval(this.fetchComments.bind(this), this.props.pollInterval);\n    }\n    render() {\n      let handleCommentSubmit = (comment: { author: string, text: string }) => {\n        console.warn('comment submitted!', comment);\n        const updated = this.state.data.slice(0);\n        updated.push(comment);\n        this.setState({ data: updated });\n      }\n      return (\n        <div className=\"commentBox\">\n        <h1>Comments</h1>\n        <CommentList data={this.state.data}/>\n      <CommentForm onCommentSubmit={handleCommentSubmit} />\n      </div>\n    );\n    }\n  }\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"application/x-typescript\">\n  class Student {\n    fullName: string;\n    constructor(\n      public firstName: string,\n      public middleInitial: string,\n      public lastName: string,\n    ) {\n      this.fullName = firstName + \" \" + middleInitial + \" \" + lastName;\n    }\n  }\n\n  interface Person {\n    firstName: string;\n    lastName: string;\n  }\n\n  function greeter(person: Person) {\n    return \"Hello, \" + person.firstName + \" \" + person.lastName;\n  }\n\n  let user = new Student(\"Jane\", \"M.\", \"User\");\n\n  document.body.innerHTML = greeter(user);\n</script>\n<script lang=\"ts\">\n  class Student {\n    fullName: string;\n    constructor(\n      public firstName: string,\n      public middleInitial: string,\n      public lastName: string,\n    ) {\n      this.fullName = firstName + \" \" + middleInitial + \" \" + lastName;\n    }\n  }\n\n  interface Person {\n    firstName: string;\n    lastName: string;\n  }\n\n  function greeter(person: Person) {\n    return \"Hello, \" + person.firstName + \" \" + person.lastName;\n  }\n\n  let user = new Student(\"Jane\", \"M.\", \"User\");\n\n  document.body.innerHTML = greeter(user);\n</script>\n<script lang=\"tsx\">\n  class CommentBox extends React.Component<\n    { url: string; pollInterval: number },\n    CommentData\n  > {\n    constructor() {\n      super();\n      this.state = { data: [] };\n    }\n    fetchComments() {\n      $.ajax({\n        url: this.props.url,\n        dataType: \"json\",\n        cache: false,\n        success: (data) => this.setState({ data: data }),\n        error: (xhr, status, err) => console.error(status, err),\n      });\n    }\n    componentDidMount() {\n      this.fetchComments();\n      setInterval(this.fetchComments.bind(this), this.props.pollInterval);\n    }\n    render() {\n      let handleCommentSubmit = (comment: { author: string; text: string }) => {\n        console.warn(\"comment submitted!\", comment);\n        const updated = this.state.data.slice(0);\n        updated.push(comment);\n        this.setState({ data: updated });\n      };\n      return (\n        <div className=\"commentBox\">\n          <h1>Comments</h1>\n          <CommentList data={this.state.data} />\n          <CommentForm onCommentSubmit={handleCommentSubmit} />\n        </div>\n      );\n    }\n  }\n</script>");
}
