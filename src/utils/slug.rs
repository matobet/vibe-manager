/// Convert a name to a URL-friendly slug
/// "Alex Chen" -> "alex-chen"
/// "María García" -> "maria-garcia"
pub fn name_to_slug(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' {
                '-'
            } else {
                // Handle accented characters
                match c {
                    'á' | 'à' | 'â' | 'ä' | 'ã' => 'a',
                    'é' | 'è' | 'ê' | 'ë' => 'e',
                    'í' | 'ì' | 'î' | 'ï' => 'i',
                    'ó' | 'ò' | 'ô' | 'ö' | 'õ' => 'o',
                    'ú' | 'ù' | 'û' | 'ü' => 'u',
                    'ñ' => 'n',
                    'ç' => 'c',
                    _ => '-',
                }
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_name() {
        assert_eq!(name_to_slug("Alex Chen"), "alex-chen");
    }

    #[test]
    fn test_accented_name() {
        assert_eq!(name_to_slug("María García"), "maria-garcia");
    }

    #[test]
    fn test_extra_spaces() {
        assert_eq!(name_to_slug("  John   Doe  "), "john-doe");
    }

    #[test]
    fn test_special_chars() {
        assert_eq!(name_to_slug("O'Brien"), "o-brien");
    }
}
