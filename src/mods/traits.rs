pub trait Formatter {
    fn replace_ws(&self) -> String;

    fn format_ws(&self) -> String;

    fn format_url(&self) -> String;
}
const UNICODE_WHITESPACE: &[char] = &[
    '\u{3000}', '\u{3164}', '\u{00A0}', '\u{1680}', '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
    '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
    '\u{202f}', '\u{205f}',
];

impl Formatter for String {
    fn replace_ws(&self) -> String {
        self.replace(UNICODE_WHITESPACE, " ")
    }

    fn format_ws(&self) -> String {
        self.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
    fn format_url(&self) -> String {
        url::form_urlencoded::byte_serialize(self.as_bytes()).collect::<String>()
    }
}

impl Formatter for &str {
    fn replace_ws(&self) -> String {
        self.replace(UNICODE_WHITESPACE, " ")
    }
    fn format_ws(&self) -> String {
        self.split_whitespace().collect::<Vec<&str>>().join(" ")
    }
    fn format_url(&self) -> String {
        url::form_urlencoded::byte_serialize(self.as_bytes()).collect::<String>()
    }
}
