use regex::Regex;

/// character filter
fn char_filter(source: &str, regex: &str, replace: &str) -> String {
    let rg = Regex::new(regex).unwrap();
    rg.replace_all(source, replace).into()
}

#[allow(unused)]
pub(crate) fn multi_space_to_single(source: &str) -> String {
    char_filter(source, r"\s\s+", " ")
}

#[allow(unused)]
pub(crate) fn nl_filter(source: &str) -> String {
    char_filter(source, r"\n", "")
}

#[cfg(test)]
mod test {
    #[test]
    fn char_filter() {
        let raw = "hello\n     \n     world\n    ";
        let ret = super::char_filter(raw.trim(), r"[\n]+[\s]*", " ");
        assert_eq!(ret, "hello world");
    }
    #[test]
    fn multi_space_filter() {
        let raw = "hello      world";

        let ret = super::multi_space_to_single(raw);
        assert_eq!(ret, "hello world");
    }
    #[test]
    fn nl_filter() {
        let raw = "\n\n";
        let ret = super::nl_filter(raw);
        assert_eq!(ret, "");
    }
}
