use std::path::PathBuf;

use simple_expand_tilde::*;

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> PathBuf {
        std::env::home_dir().unwrap_or_else(|| panic!("no home dir in test environment"))
    }

    #[test]
    fn expands_tilde_with_single_component() {
        assert_eq!(expand_tilde("~/foo"), Some(home().join("foo")));
    }

    #[test]
    fn expands_tilde_with_nested_path() {
        assert_eq!(
            expand_tilde("~/foo/bar/baz"),
            Some(home().join("foo/bar/baz"))
        );
    }

    #[test]
    fn expands_bare_tilde() {
        assert_eq!(expand_tilde("~"), Some(home()));
    }

    #[test]
    fn no_expansion_for_absolute_path() {
        assert_eq!(
            expand_tilde("/home/user/foo"),
            Some(PathBuf::from("/home/user/foo"))
        );
    }

    #[test]
    fn no_expansion_for_relative_path() {
        assert_eq!(expand_tilde("foo/bar"), Some(PathBuf::from("foo/bar")));
    }

    #[test]
    fn no_expansion_for_tilde_in_middle() {
        assert_eq!(expand_tilde("foo/~/bar"), Some(PathBuf::from("foo/~/bar")));
    }

    #[test]
    fn no_expansion_for_tilde_suffix() {
        // "foo~" is not a tilde path — should pass through unchanged
        assert_eq!(expand_tilde("foo~"), Some(PathBuf::from("foo~")));
    }

    #[test]
    fn expands_tilde_with_pathbuf_input() {
        let input = PathBuf::from("~/foo");
        assert_eq!(expand_tilde(input), Some(home().join("foo")));
    }

    #[test]
    fn expands_tilde_only_component() {
        // Ensures "~foo" (no separator) is NOT treated as a tilde expansion
        assert_eq!(expand_tilde("~foo"), Some(PathBuf::from("~foo")));
    }

    #[test]
    fn empty_path() {
        assert_eq!(expand_tilde(""), Some(PathBuf::from("")));
    }
}
