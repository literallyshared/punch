#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn date_creation() {
        let good_input = ["2025-05-05".to_string(), "25-05-05".to_string()];
        let bad_input = "204a,-43".to_string();
        for input in good_input.iter() {
            assert!(Date::new(input).is_some());
        }
        assert!(Date::new(&bad_input).is_none());
        for input in good_input {
            assert_eq!(
                format!(
                    "{}/{DIRECTORY}/2025/May/2025-05-05",
                    std::env::var("HOME").unwrap()
                ),
                Date::new(&input).unwrap().get_file_path_for_date().unwrap()
            );
        }
    }

    #[test]
    fn command_parsing() {
        assert!(Command::from_args(vec![]).is_none());
        assert!(Command::from_args(vec!["invalid".to_string()]).is_none());

        assert!(matches!(
            Command::from_args(vec!["in".to_string()]),
            Some(Command::PunchIn)
        ));

        let out_cmd = Command::from_args(vec!["out".to_string(), "work".to_string()]);
        assert!(matches!(out_cmd, Some(Command::PunchOut(Some(activity))) if activity == "work"));

        let out_no_activity = Command::from_args(vec!["out".to_string()]);
        assert!(matches!(out_no_activity, Some(Command::PunchOut(None))));

        let report_today = Command::from_args(vec!["report".to_string()]);
        assert!(matches!(report_today, Some(Command::Report(_))));

        let report_date = Command::from_args(vec!["report".to_string(), "2023-05-01".to_string()]);
        assert!(matches!(report_date, Some(Command::Report(date)) if date == "2023-05-01"));

        let edit_today = Command::from_args(vec!["edit".to_string()]);
        assert!(matches!(edit_today, Some(Command::Edit(_))));

        let edit_date = Command::from_args(vec!["edit".to_string(), "2023-05-01".to_string()]);
        assert!(matches!(edit_date, Some(Command::Edit(date)) if date == "2023-05-01"));

        assert!(matches!(
            Command::from_args(vec!["--help".to_string()]),
            Some(Command::Help)
        ));
        assert!(matches!(
            Command::from_args(vec!["-h".to_string()]),
            Some(Command::Help)
        ));
        assert!(matches!(
            Command::from_args(vec!["--version".to_string()]),
            Some(Command::Version)
        ));
        assert!(matches!(
            Command::from_args(vec!["-v".to_string()]),
            Some(Command::Version)
        ));
    }

    #[test]
    fn report_parsing() {
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

    #[test]
    fn report_parsing_empty_lines() {
        let input = r#"
08:00 - 12:00 work

16:00 - 17:00 meeting

"#;
        let report = parse_report_file(input).unwrap();
        assert_eq!(report.activities.len(), 2);
        assert_eq!(*report.activities.get("work").unwrap(), 14400);
        assert_eq!(*report.activities.get("meeting").unwrap(), 3600);
    }

    #[test]
    fn report_parsing_current_activity() {
        let input = "08:00 -";
        let report = parse_report_file(input).unwrap();
        assert_eq!(report.activities.len(), 1);
        assert!(report.activities.contains_key("[*Current*]"));
    }

    #[test]
    fn report_parsing_activity_with_spaces() {
        let input = "08:00 - 09:00 coding project";
        let report = parse_report_file(input).unwrap();
        assert_eq!(report.activities.len(), 1);
        assert!(report.activities.contains_key("coding project"));
        assert_eq!(*report.activities.get("coding project").unwrap(), 3600);
    }

    #[test]
    fn report_parsing_duplicate_activities() {
        let input = r#"08:00 - 09:00 work
10:00 - 11:00 work
12:00 - 13:00 lunch"#;
        let report = parse_report_file(input).unwrap();
        assert_eq!(report.activities.len(), 2);
        assert_eq!(*report.activities.get("work").unwrap(), 7200); // 2 hours
        assert_eq!(*report.activities.get("lunch").unwrap(), 3600); // 1 hour
    }

    #[test]
    fn get_file_path_for_date_valid() {
        let path = get_file_path_for_date(2023, 5, 15);
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.contains(".punch"));
        assert!(path.contains("2023"));
        assert!(path.contains("May"));
        assert!(path.contains("2023-05-15"));
    }

    #[test]
    fn get_file_path_for_invalid_dates() {
        assert!(get_file_path_for_date(2023, 13, 1).is_none());
        assert!(get_file_path_for_date(2023, 2, 30).is_none());
        assert!(get_file_path_for_date(2023, 4, 31).is_none());
    }

    #[test]
    fn get_today_format() {
        let today = get_today();
        assert!(today.len() >= 8);
        assert!(today.contains("-"));
        let parts: Vec<&str> = today.split('-').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].len(), 4); // year
        assert_eq!(parts[1].len(), 2); // month
        assert_eq!(parts[2].len(), 2); // day
    }
}
