//! Name formatting helpers for dense card lines

/// Abbreviate a full name to first name + last-name initial
/// "Sam Taylor" -> "Sam T"
/// "Jonas" -> "Jonas"
/// "Mary Jane Watson" -> "Mary W"
pub fn abbreviate_name(name: &str) -> String {
    let mut parts = name.split_whitespace();
    let first = parts.next().unwrap_or(name);
    match parts.last().and_then(|l| l.chars().next()) {
        Some(initial) => format!("{} {}", first, initial),
        None => first.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_part_name() {
        assert_eq!(abbreviate_name("Sam Taylor"), "Sam T");
    }

    #[test]
    fn test_single_name() {
        assert_eq!(abbreviate_name("Jonas"), "Jonas");
    }

    #[test]
    fn test_three_part_name() {
        assert_eq!(abbreviate_name("Mary Jane Watson"), "Mary W");
    }

    #[test]
    fn test_empty_name() {
        assert_eq!(abbreviate_name(""), "");
    }

    #[test]
    fn test_whitespace_padding() {
        assert_eq!(abbreviate_name("  Ana Perez  "), "Ana P");
    }
}
