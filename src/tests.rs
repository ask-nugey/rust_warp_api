#[cfg(test)]
mod tests {
    use super::models::Name;

    #[test]
    fn test_name_new() {
        assert!(Name::new("Nrskt").is_ok());
        assert!(Name::new("N").is_ok());
        assert!(Name::new("NrsktNrskt").is_ok());

        assert!(Name::new("0").is_err());
        assert!(Name::new("").is_err());
        assert!(Name::new("NrsktNrsktN").is_err());
    }
}
