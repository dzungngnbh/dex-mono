use anyhow::Result;

pub fn format_hash(hash: String) -> Result<String> {
    if !hash.is_empty() {
        return if hash.len() >= 8 {
            let prefix = &hash[..5];
            let suffix = &hash[hash.len() - 3..];
            Ok(format!("{}...{}", prefix, suffix))
        } else {
            Ok(hash)
        };
    }

    Ok(String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_hash() -> Result<()> {
        let res = format_hash("123".to_string())?;
        assert_eq!(res, "123");

        let res = format_hash("12345678".to_string())?;
        assert_eq!(res, "12345...678");
        Ok(())
    }
}
