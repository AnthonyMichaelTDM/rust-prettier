//! defines the options for `PrettyPrinter`

use std::{fmt::Display, path::Path};

use derive_builder::Builder;

#[derive(Debug, Clone, Copy)]
pub enum Parser {
    Javascript,
}

impl<T: AsRef<str>> From<T> for Parser {
    fn from(s: T) -> Self {
        match s.as_ref() {
            "javascript" | "js" => Self::Javascript,
            _ => unimplemented!("no parser for {} is implemented yet", s.as_ref()),
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::Javascript
    }
}

impl Parser {
    pub fn infer_from_file_name(file_path: &Path) -> Option<Self> {
        match file_path.extension().and_then(|e| e.to_str()) {
            Some("js") => Some(Self::Javascript),
            _ => None,
        }
    }
}

#[allow(dead_code, clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Builder)]
#[builder(default)]
/// See [the documentation](https://prettier.io/docs/en/options)
pub struct PrettyPrinter {
    /// Try prettier's new ternary formatting before it becomes the default behavior.
    ///
    /// Valid options:
    /// - true - Use curious ternaries, with the question mark after the condition.
    /// - false - Retain the default behavior of ternaries; keep question marks on the same line as the consequent.
    ///
    /// TODO: Support
    pub experimental_ternaries: bool,
    /// Specify the line length that the printer will wrap on.
    pub print_width: usize,
    /// Specify the number of spaces per indentation-level.
    pub tab_width: usize,
    /// Indent lines with tabs instead of spaces.
    pub use_tabs: bool,
    /// Print semicolons at the ends of statements.
    /// Valid options:
    /// - true - Add a semicolon at the end of every statement.
    /// - false - Only add semicolons at the beginning of lines that [may introduce ASI failures](https://prettier.io/docs/en/rationale#semicolons).
    ///
    /// TODO: support
    pub semi: bool,
    /// Use single quotes instead of double quotes.
    ///
    /// Notes:
    /// - JSX quotes ignore this option – see [jsx-single-quote](https://prettier.io/docs/en/options#jsx-quotes).
    /// - If the number of quotes outweighs the other quote, the quote which is less used will be used to format the string - Example: "I'm double quoted" results in "I'm double quoted" and "This \"example\" is single quoted" results in 'This "example" is single quoted'.
    ///
    /// See the [strings rationale](https://prettier.io/docs/en/rationale#strings) for more information.
    ///
    /// TODO: Support
    pub single_quote: bool,
    /// Change when properties in objects are quoted.
    ///
    /// Valid options:
    /// - "as-needed" - Only add quotes around object properties where required.
    /// - "consistent" - If at least one property in an object requires quotes, quote all properties.
    /// - "preserve" - Respect the input use of quotes in object properties.
    ///
    /// Note that Prettier never unquotes numeric property names in Angular expressions, TypeScript, and Flow because the distinction between string and numeric keys is significant in these languages. Also Prettier doesn’t unquote numeric properties for Vue.
    ///
    /// If this option is set to preserve, singleQuote to false (default value), and parser to json5, double quotes are always used for strings.
    ///
    /// TODO: support
    #[builder(setter(into))]
    pub quote_props: QuoteProps,
    /// Use single quotes instead of double quotes in JSX.
    ///
    /// TODO: support
    pub jsx_single_quote: bool,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. (A single-line array, for example, never gets trailing commas.)
    ///
    /// valid options:
    /// - "all" - Trailing commas wherever possible (including [function parameters and calls](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#Trailing_commas_in_functions)). To run, JavaScript code formatted this way needs an engine that supports ES2017 (Node.js 8+ or a modern browser) or [downlevel compilation](https://babeljs.io/docs/en/index). This also enables trailing commas in type parameters in TypeScript (supported since TypeScript 2.7 released in January 2018).
    /// - "es5" - Trailing commas where valid in ES5 (objects, arrays, etc.). Trailing commas in type parameters in TypeScript and Flow
    /// - "none" - No trailing commas
    ///
    /// TODO: support
    #[builder(setter(into))]
    pub trailing_comma: TrailingComma,
    /// Print spaces between brackets in object literals.
    ///
    /// Valid options:
    ///
    /// - true - Example: { foo: bar }.
    /// - false - Example: {foo: bar}.
    pub bracket_spacing: bool,
    /// Put the > of a multi-line HTML (HTML, JSX, Vue, Angular) element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    ///
    /// Valid options:
    /// - `true` - Example:
    ///   ```html
    ///   <button
    ///    className="prettier-class"
    ///    id="prettier-id"
    ///    onClick={this.handleClick}>
    ///    Click Here
    ///   </button>
    ///   ```
    /// - `false` - Example:
    ///   ```html
    ///   <button
    ///     className="prettier-class"
    ///     id="prettier-id"
    ///     onClick={this.handleClick}
    ///   >
    ///     Click Here
    ///   </button>
    ///   ```
    ///
    /// TODO: Support
    pub bracket_same_line: bool,
    /// Include parentheses around a sole arrow function parameter.
    ///
    /// Valid options:
    /// - "always" - Always include parens. Example: `(x) => x`
    /// - "avoid" - Omit parens when possible. Example: `x => x`
    ///
    /// At first glance, avoiding parentheses may look like a better choice because of less visual noise. However, when Prettier removes parentheses, it becomes harder to add type annotations, extra arguments or default values as well as making other changes. Consistent use of parentheses provides a better developer experience when editing real codebases, which justifies the default value for the option.
    ///
    /// TODO: Support
    #[builder(setter(into))]
    pub arrow_parens: ArrowParens,
    /// Format only a segment of a file.
    ///
    /// These two options can be used to format code starting and ending at a given character offset (inclusive and exclusive, respectively). The range will extend:
    /// - Backwards to the start of the first line containing the selected statement.
    /// - Forwards to the end of the selected statement.
    ///
    /// These options cannot be used with cursorOffset.
    ///
    /// TODO: Support
    #[builder(setter(strip_option))]
    pub range_start: Option<usize>,
    /// Format only a segment of a file.
    ///
    /// These two options can be used to format code starting and ending at a given character offset (inclusive and exclusive, respectively). The range will extend:
    /// - Backwards to the start of the first line containing the selected statement.
    /// - Forwards to the end of the selected statement.
    ///
    /// These options cannot be used with cursorOffset.
    ///
    /// TODO: Support
    #[builder(setter(strip_option))]
    pub range_end: Option<usize>,
    /// Specify which language parser to use.
    ///
    /// Prettier automatically infers the parser from the input file path,
    /// so you shouldn’t have to change this setting.
    ///
    /// TODO: "Support" (we only need to support one parser so this may as well be implemented)
    #[builder(setter(strip_option, into))]
    pub parser: Option<Parser>,
    /// Specify the file name to use to infer which parser to use.
    ///
    /// For example, the following will use the CSS parser:
    /// ```sh
    /// cat foo | prettier --stdin-filepath foo.css
    /// ```
    /// This option is only useful in the CLI and API. It doesn’t make sense to use it in a configuration file.
    ///
    /// TODO: Support
    #[builder(setter(strip_option))]
    pub file_path: Option<String>,
    /// Prettier can restrict itself to only format files that contain a special comment, called a pragma, at the top of the file. This is very useful when gradually transitioning large, unformatted codebases to Prettier.
    ///
    /// A file with the following as its first comment will be formatted when --require-pragma is supplied:
    /// ```js
    /// /**
    ///  * @prettier
    ///  */
    /// ```
    /// or
    /// ```js
    /// /**
    ///  * @format
    ///  */
    /// ```
    ///
    /// TODO: Support
    pub require_pragma: bool,
    /// Prettier can insert a special `@format` marker at the top of files
    /// specifying that the file has been formatted with Prettier.
    /// This works well when used in tandem with the `--require-pragma` option.
    /// If there is already a docblock at the top of the file then
    /// this option will add a newline to it with the @format marker.
    ///
    /// Note that “in tandem” doesn’t mean “at the same time”.
    /// When the two options are used simultaneously, `--require-pragma` has priority,
    /// so `--insert-pragma` is ignored.
    /// The idea is that during an incremental adoption of Prettier in a big codebase,
    /// the developers participating in the transition process use `--insert-pragma`
    /// whereas `--require-pragma` is used by the rest of the team and automated tooling
    /// to process only files already transitioned.
    /// The feature has been inspired by Facebook’s adoption strategy.
    ///
    /// TODO: Support
    pub insert_pragma: bool,
    /// By default, Prettier will not change wrapping in markdown text since
    /// some services use a linebreak-sensitive renderer,
    /// e.g. GitHub comments and BitBucket.
    /// To have Prettier wrap prose to the print width, change this option to `"always"`.
    /// If you want Prettier to force all prose blocks to be on a single line and rely on
    /// editor/viewer soft wrapping instead, you can use `"never"`.
    ///
    /// Valid options:
    /// - "always" - Wrap prose if it exceeds the print width.
    /// - "never" - Un-wrap each block of prose into one line.
    /// - "preserve" - Do nothing, leave prose as-is. First available in v1.9.0
    ///
    /// TODO: Support
    #[builder(setter(into))]
    pub prose_wrap: ProseWrap,
    /// Specify the global whitespace sensitivity for HTML, Vue, Angular, and Handlebars.
    /// See [whitespace-sensitive formatting](https://prettier.io/blog/2018/11/07/1.15.0#whitespace-sensitive-formatting) for more info.
    ///
    /// Valid options:
    ///
    /// - `"css"` - Respect the default value of CSS display property. For Handlebars treated same as strict.
    /// - `"strict"` - Whitespace (or the lack of it) around all tags is considered significant.
    /// - `"ignore"` - Whitespace (or the lack of it) around all tags is considered insignificant.
    ///
    /// TODO: "Support" (we only need to support JS, and this isn't used by JS, so we actually don't need to support this yet)
    #[builder(setter(into))]
    pub html_whitespace_sensitivity: HtmlWhitespaceSensitivity,
    /// Whether or not to indent the code inside `<script>` and `<style>` tags in Vue files.
    ///
    /// Valid options:
    ///
    /// - `false` - Do not indent script and style tags in Vue files.
    /// - `true` - Indent script and style tags in Vue files.
    ///
    /// TODO: "Supprt" (we only need to support JS, and this isn't JS, so we don't actually need this)
    pub vue_indent_script_and_style: bool,
    /// For historical reasons, there exist two common flavors of line endings in text files.
    /// That is `\n` (or `LF` for Line Feed) and `\r\n` (or `CRLF` for Carriage Return + Line Feed).
    /// The former is common on Linux and macOS, while the latter is prevalent on Windows.
    /// Some details explaining why it is so [can be found on Wikipedia](https://en.wikipedia.org/wiki/Newline).
    ///
    /// When people collaborate on a project from different operating systems,
    /// it becomes easy to end up with mixed line endings in a shared git repository.
    /// It is also possible for Windows users to accidentally change line endings in a previously committed file from `LF` to `CRLF`.
    /// Doing so produces a large `git diff` and thus makes the line-by-line history for a file (`git blame`) harder to explore.
    ///
    /// If you want to make sure that your entire git repository only contains Linux-style line endings in files covered by Prettier:
    ///
    /// 1. Ensure Prettier’s `endOfLine` option is set to `lf` (this is the default)
    /// 2. Configure [a pre-commit hook](https://prettier.io/docs/en/precommit) that will run Prettier
    /// 3. Configure Prettier to run in your CI pipeline using [`--check` flag](https://prettier.io/docs/en/cli#--check).
    /// If you use Travis CI, set pthe `autocrlf` option](https://docs.travis-ci.com/user/customizing-the-build#git-end-of-line-conversion-control)
    /// to `input` in `.travis.yml`.
    /// 4. Add `* text=auto eol=lf` to the repo’s `.gitattributes` file.
    /// You may need to ask Windows users to re-clone your repo after this change to ensure git has not converted `LF` to `CRLF` on checkout.
    ///
    /// All modern text editors in all operating systems are able to correctly display line endings when `\n` (`LF`) is used.
    /// However, old versions of Notepad for Windows will visually squash such lines into one as they can only deal with `\r\n` (`CRLF`).
    ///
    /// Valid options:
    ///
    /// - `"lf"` – Line Feed only (`\n`), common on Linux and macOS as well as inside git repos
    /// - `"crlf"` - Carriage Return + Line Feed characters (`\r\n`), common on Windows
    /// - `"cr"` - Carriage Return character only (`\r`), used very rarely
    /// - `"auto"` - Maintain existing line endings (mixed values within one file are normalised by looking at what’s used after the first line)
    ///
    /// TODO: Support "auto"
    #[builder(setter(into))]
    pub end_of_line: EndOfLine,
    /// Control whether Prettier formats quoted code embedded in the file.
    ///
    /// When Prettier identifies cases where it looks like you've placed some code it knows how to format within a string in another file,
    /// like in a tagged template in JavaScript with a tag named `html` or in code blocks in Markdown, it will by default try to format that code.
    ///
    /// Sometimes this behavior is undesirable, particularly in cases where you might not have intended the string to be interpreted as code.
    /// This option allows you to switch between the default behavior (`auto`) and disabling this feature entirely (`off`).
    ///
    /// Valid options:
    ///
    /// - `"auto"` – Format embedded code if Prettier can automatically identify it.
    /// - `"off"` - Never automatically format embedded code.
    ///
    /// TODO: Support
    #[builder(setter(into))]
    pub embedded_language_formatting: EmbeddedLanguageFormatting,
    /// Enforce single attribute per line in HTML, Vue and JSX.
    ///
    /// Valid options:
    ///
    /// - `false` - Do not enforce single attribute per line.
    /// - `true` - Enforce single attribute per line.
    ///
    /// TODO: Support
    pub single_attribute_per_line: bool,
    /// Specify the location of the cursor, as a character offset from the start of the file (similar to how `range_start` and `range_end` are set)
    ///
    /// This is useful for editor integrations to allow the formatter to maintain the relative position of the cursor.
    ///
    /// This option cannot be used with rangeStart and rangeEnd.
    ///
    /// Note: We do not need to support this yet
    #[builder(setter(strip_option))]
    pub cursor_offset: Option<usize>,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            experimental_ternaries: false,
            print_width: 80,
            tab_width: 2,
            use_tabs: false,
            semi: true,
            single_quote: false,
            quote_props: QuoteProps::default(),
            jsx_single_quote: false,
            trailing_comma: TrailingComma::default(),
            bracket_spacing: true,
            bracket_same_line: false,
            arrow_parens: ArrowParens::default(),
            range_start: None,
            range_end: None,
            parser: Some(Parser::default()), // TODO: when we can infer parser from file path, default to None here
            file_path: None,
            require_pragma: false,
            insert_pragma: false,
            prose_wrap: ProseWrap::default(),
            html_whitespace_sensitivity: HtmlWhitespaceSensitivity::default(),
            vue_indent_script_and_style: false,
            end_of_line: EndOfLine::default(),
            embedded_language_formatting: EmbeddedLanguageFormatting::default(),
            single_attribute_per_line: false,
            cursor_offset: None,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Change when properties in objects are quoted.
pub enum QuoteProps {
    /// Only add quotes around object properties where required.
    #[default]
    AsNeeded,
    /// If at least one property in an object requires quotes, quote all properties.
    Conistent,
    /// Respect the input use of quotes in object properties.
    Preserve,
}

impl From<&str> for QuoteProps {
    /// defaults to `QuoteProp::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "as-needed" => Self::AsNeeded,
            "consistent" => Self::Conistent,
            "preserve" => Self::Preserve,
            _ => Self::default(),
        }
    }
}

impl Display for QuoteProps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AsNeeded => write!(f, "as-needed"),
            Self::Conistent => write!(f, "consistent"),
            Self::Preserve => write!(f, "preserve"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. (A single-line array, for example, never gets trailing commas.)
pub enum TrailingComma {
    /// Trailing commas wherever possible (including [function parameters and calls](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#Trailing_commas_in_functions)). To run, JavaScript code formatted this way needs an engine that supports ES2017 (Node.js 8+ or a modern browser) or [downlevel compilation](https://babeljs.io/docs/en/index). This also enables trailing commas in type parameters in TypeScript (supported since TypeScript 2.7 released in January 2018).
    #[default]
    All,
    /// Trailing commas where valid in ES5 (objects, arrays, etc.). Trailing commas in type parameters in TypeScript and Flow
    ES5,
    /// No trailing commas
    None,
}

impl From<&str> for TrailingComma {
    /// defaults to `TrailingComma::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "all" => Self::All,
            "es5" => Self::ES5,
            "none" => Self::None,
            _ => Self::default(),
        }
    }
}

impl Display for TrailingComma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::ES5 => write!(f, "es5"),
            Self::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Include parentheses around a sole arrow function parameter.
pub enum ArrowParens {
    /// Always include parens. Example: `(x) => x`
    #[default]
    Always,
    /// Omit parens when possible. Example: `x => x`
    Avoid,
}

impl From<&str> for ArrowParens {
    /// defaults to `ArrowParens::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "avoid" => Self::Avoid,
            _ => Self::default(),
        }
    }
}

impl Display for ArrowParens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Avoid => write!(f, "avoid"),
        }
    }
}

/// By default, Prettier will not change wrapping in markdown text since
/// some services use a linebreak-sensitive renderer,
/// e.g. GitHub comments and `BitBucket`.
/// To have Prettier wrap prose to the print width, change this option to `"always"`.
/// If you want Prettier to force all prose blocks to be on a single line and rely on
/// editor/viewer soft wrapping instead, you can use `"never"`.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum ProseWrap {
    /// Wrap prose if it exceeds the print width.
    Always,
    /// Un-wrap each block of prose into one line.
    Never,
    /// Do nothing, leave prose as-is.
    #[default]
    Preserve,
}

impl From<&str> for ProseWrap {
    /// defaults to `ProseWrap::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "always" => Self::Always,
            "never" => Self::Never,
            "preserve" => Self::Preserve,
            _ => Self::default(),
        }
    }
}

impl Display for ProseWrap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Always => write!(f, "always"),
            Self::Never => write!(f, "never"),
            Self::Preserve => write!(f, "preserve"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum EndOfLine {
    /// Line Feed only (\n), common on Linux and macOS as well as inside git repos
    #[default]
    Lf,
    /// Carriage Return + Line Feed characters (`\r\n`), common on Windows
    CrLf,
    /// Carriage Return character only (`\r`), used very rarely
    Cr,
    /// Maintain existing line endings (mixed values within one file are normalised by looking at what’s used after the first line)
    Auto,
}

impl From<&str> for EndOfLine {
    /// defaults to `EndOfLine::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "lf" => Self::Lf,
            "crlf" => Self::CrLf,
            "cr" => Self::Cr,
            "auto" => Self::Auto,
            _ => Self::default(),
        }
    }
}

impl Display for EndOfLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lf => write!(f, "lf"),
            Self::CrLf => write!(f, "crlf"),
            Self::Cr => write!(f, "cr"),
            Self::Auto => write!(f, "auto"),
        }
    }
}

/// Specify the global whitespace sensitivity for HTML, Vue, Angular, and Handlebars.
/// See [whitespace-sensitive formatting](https://prettier.io/blog/2018/11/07/1.15.0#whitespace-sensitive-formatting) for more info.
///
/// Valid options:
///
/// - `"css"` - Respect the default value of CSS `display` property. For Handlebars treated same as `strict`.
/// - `"strict"` - Whitespace (or the lack of it) around all tags is considered significant.
/// - `"ignore"` - Whitespace (or the lack of it) around all tags is considered insignificant.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum HtmlWhitespaceSensitivity {
    /// Respect the default value of CSS display property. For Handlebars treated same as strict.
    #[default]
    Css,
    /// Whitespace (or the lack of it) around all tags is considered significant.
    Strict,
    /// Whitespace (or the lack of it) around all tags is considered insignificant.
    Ignore,
}

impl From<&str> for HtmlWhitespaceSensitivity {
    fn from(value: &str) -> Self {
        match value {
            "css" => Self::Css,
            "strict" => Self::Strict,
            "ignore" => Self::Ignore,
            _ => Self::default(),
        }
    }
}

impl Display for HtmlWhitespaceSensitivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Css => write!(f, "css"),
            Self::Strict => write!(f, "strict"),
            Self::Ignore => write!(f, "ignore"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
/// Control whether Prettier formats quoted code embedded in the file.
pub enum EmbeddedLanguageFormatting {
    /// Format embedded code if Prettier can automatically identify it.
    #[default]
    Auto,
    /// Never automatically format embedded code.
    Off,
}

impl From<&str> for EmbeddedLanguageFormatting {
    /// defaults to `EmbeddedLanguageFormatting::default()` if `value` is not a supported string
    fn from(value: &str) -> Self {
        match value {
            "auto" => Self::Auto,
            "off" => Self::Off,
            _ => Self::default(),
        }
    }
}

impl Display for EmbeddedLanguageFormatting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Off => write!(f, "off"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PrettyPrinterBuilder::default().build().unwrap();

        assert!(config.bracket_spacing);
    }
}
