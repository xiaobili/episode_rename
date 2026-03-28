use openlist_tui::validate::{validate_input, InputType};

#[test]
fn test_empty_show_name() {
    assert!(validate_input("", InputType::ShowName).is_err());
}

#[test]
fn test_valid_show_name() {
    assert!(validate_input("The Office", InputType::ShowName).is_ok());
}

#[test]
fn test_invalid_season() {
    assert!(validate_input("abc", InputType::Season).is_err());
}

#[test]
fn test_valid_season() {
    assert!(validate_input("5", InputType::Season).is_ok());
}

#[test]
fn test_invalid_regex() {
    assert!(validate_input("[invalid", InputType::Regex).is_err());
}

#[test]
fn test_valid_regex() {
    assert!(validate_input(r"\d+", InputType::Regex).is_ok());
}
