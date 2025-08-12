#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn date_creation() {
        let good_input = "2025-05-05".to_string();
        let bad_input = "204a,-43".to_string();
        assert!(Date::new(&good_input).is_some());
        assert!(Date::new(&bad_input).is_none());
        assert_eq!(
            format!("{}/{DIRECTORY}/2025/May/2025-05-05", std::env::var("HOME").unwrap()),
            Date::new(&good_input).unwrap().get_file_path_for_date().unwrap()
        );
    }

    #[test]
    fn report() {
        let input = r#"08:00 - 12:00 test
12:30 - 16:30 test
16:30 - 16:40 other
"#;
        let report = parse_report_file(input);
        assert!(report.is_some());
        let report = report.unwrap();
        assert_eq!(report.activities.len(), 2);
        assert_eq!(*report.activities.get("other").unwrap(), 600);
        assert_eq!(*report.activities.get("test").unwrap(), 28800);
    }
}
