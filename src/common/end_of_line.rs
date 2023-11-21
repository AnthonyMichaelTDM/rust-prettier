#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum EndLine {
    Cr,
    #[default]
    Lf,
    CrLf,
}

impl ToString for EndLine {
    fn to_string(&self) -> String {
        match self {
            Self::Cr => "\r",
            Self::Lf => "\n",
            Self::CrLf => "\r\n",
        }
        .into()
    }
}

impl From<&str> for EndLine {
    fn from(input: &str) -> Self {
        match input {
            "\r" => Self::Cr,
            "\n" => Self::Lf,
            "\r\n" => Self::CrLf,
            _ => Self::default(),
        }
    }
}
impl EndLine {
    pub fn guess_from_str(input: impl AsRef<str>) -> Self {
        let input = input.as_ref();
        if input.contains("\r\n") {
            Self::CrLf
        } else if input.contains('\r') {
            Self::Cr
        } else {
            Self::Lf
        }
    }

    pub fn count_occurrences(&self, input: impl AsRef<str>) -> usize {
        let input = input.as_ref();
        match self {
            Self::Cr => input.matches('\r').count(),
            Self::Lf => input.matches('\n').count(),
            Self::CrLf => input.matches("\r\n").count(),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub fn normalize_end_of_line(input: impl AsRef<str>) -> String {
    input.as_ref().replace("\r\n", "\n").replace('\r', "\n")
}
