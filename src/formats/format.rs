use std::str::FromStr;
use std::io::ErrorKind;

#[derive(Debug,PartialEq)]
pub enum Format {
    Bitbar,
    Markdown,
}

impl FromStr for Format {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitbar" => Ok(Format::Bitbar),
            "markdown" => Ok(Format::Markdown),
            &_ => Err(std::io::Error::new(ErrorKind::InvalidInput, format!("{}", s))),
        }
    }
}

#[test]
fn should_derive_output_from_string_correctly() {
    assert_eq!(Format::from_str("bitbar").unwrap(), Format::Bitbar);
    assert_eq!(Format::from_str("markdown").unwrap(), Format::Markdown);
    assert_eq!(Err(std::io::ErrorKind::InvalidInput), Format::from_str("mark").map_err(|e| e.kind()));
}

