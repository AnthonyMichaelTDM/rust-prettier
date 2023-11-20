#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EndLine {
    Cr,
    Lf,
    CrLf,
}

impl ToString for EndLine {
    fn to_string(&self) -> String {
        match self {
            EndLine::Cr => "\r",
            EndLine::Lf => "\n",
            EndLine::CrLf => "\r\n",
        }
        .into()
    }
}

impl From<&str> for EndLine {
    fn from(input: &str) -> EndLine {
        match input {
            "\r" => EndLine::Cr,
            "\n" => EndLine::Lf,
            "\r\n" => EndLine::CrLf,
            _ => panic!("invalid end of line"),
        }
    }
}

impl EndLine {
    pub fn guess_from_str(input: impl AsRef<str>) -> EndLine {
        let input = input.as_ref();
        if input.contains("\r\n") {
            EndLine::CrLf
        } else if input.contains("\r") {
            EndLine::Cr
        } else {
            EndLine::Lf
        }
    }

    pub fn count_occurrences(&self, input: impl AsRef<str>) -> usize {
        let input = input.as_ref();
        match self {
            EndLine::Cr => input.matches("\r").count(),
            EndLine::Lf => input.matches("\n").count(),
            EndLine::CrLf => input.matches("\r\n").count(),
        }
    }
}

pub fn normalize_end_of_line(input: impl AsRef<str>) -> String {
    input.as_ref().replace("\r\n", "\n").replace("\r", "\n")
}
