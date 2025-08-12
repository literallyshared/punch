use editor_command::EditorBuilder;
use itertools::Itertools;
use simple_duration::Duration;
use std::collections::HashMap;

mod test;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DIRECTORY: &str = ".punch";

pub struct Report {
    // Activity - Seconds elapsed
    activities: HashMap<String, u64>,
}

pub struct Date {
    year: i32,
    month: u32,
    day: u32,
}

impl Date {
    pub fn new(date: &String) -> Option<Self> {
        let mut split = date.split('-');
        let year = split.next();
        let month = split.next();
        let day = split.next();
        if year.is_none() || month.is_none() || day.is_none() {
            println!("Error: Failed to parse date: [{date}].");
            return None;
        }
        let mut year = year.unwrap().parse::<i32>().unwrap();
        if year < 100 {
            year += 2000;
        }
        let month = month.unwrap().parse::<u32>().unwrap();
        let day = day.unwrap().parse::<u32>().unwrap();
        Some(Self { year, month, day })
    }

    pub fn get_file_path_for_date(&self) -> Option<String> {
        get_file_path_for_date(self.year, self.month, self.day)
    }
}

pub enum Command {
    PunchIn,
    PunchOut(Option<String>),
    Report(String),
    Edit(String),
    Help,
    Version,
}

impl Command {
    fn from_args(input: Vec<String>) -> Option<Command> {
        if input.is_empty() {
            return None;
        }
        let first = &input[0];
        match first.as_str() {
            "in" => Some(Command::PunchIn),
            "out" => {
                if input.len() < 2 {
                    println!(
                        "Warning: Punching out without activity type. You need to edit this manually!"
                    );
                    return Some(Command::PunchOut(None));
                }
                Some(Command::PunchOut(Some(input[1].clone())))
            }
            "report" => {
                if input.len() < 2 {
                    return Some(Command::Report(get_today()));
                }
                Some(Command::Report(input[1].clone()))
            }
            "edit" => {
                if input.len() < 2 {
                    return Some(Command::Edit(get_today()));
                }
                Some(Command::Edit(input[1].clone()))
            }
            "--help" | "-h" => Some(Command::Help),
            "--version" | "-v" => Some(Command::Version),
            _ => None,
        }
    }
}

fn main() {
    try_create_directory(get_todays_dir());
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);
    if let Some(command) = Command::from_args(args) {
        match command {
            Command::PunchIn => punch_in(),
            Command::PunchOut(maybe_activity) => punch_out(maybe_activity),
            Command::Report(date) => print_report_for_date(date),
            Command::Help => print_help(),
            Command::Edit(date) => edit(date),
            Command::Version => print_version(),
            _ => {}
        }
    } else {
        print_help();
    }
}

fn punch_in() {
    let full_path = get_todays_file_path();
    if let Ok(exists) = std::fs::exists(&full_path) {
        let contents = if exists {
            str::from_utf8(&std::fs::read(&full_path).unwrap())
                .unwrap()
                .to_string()
        } else {
            "".to_string()
        };
        let stamp = chrono::Local::now().format("%R");
        let to_be_written = if contents.is_empty() {
            format!("{contents}{stamp} - ")
        } else {
            format!("{contents}\n{stamp} - ")
        };
        if std::fs::write(&full_path, to_be_written).is_err() {
            println!("Error: Failed to create report file for [{}].", get_today());
            return;
        }
    }
}

fn punch_out(maybe_activity: Option<String>) {
    let full_path = get_todays_file_path();
    if let Ok(exists) = std::fs::exists(&full_path) {
        if !exists {
            println!("You need to punch in first! See punch --help.");
            return;
        }
    }
    let activity = maybe_activity.unwrap_or("Unknown activity".to_string());
    if let Ok(contents) = std::fs::read(&full_path) {
        let contents = str::from_utf8(&contents).unwrap();
        if contents.ends_with('-') {
            println!("Error: You have not punched in yet!");
            return;
        }
        let stamp = chrono::Local::now().format("%R");
        let appended = format!("{contents}{stamp} {activity}");
        if std::fs::write(&full_path, appended).is_err() {
            println!("Error: Failed to modify report file for [{}].", get_today());
            return;
        }
    }
}

fn print_help() {
    let help_text = r#"
    Usage:
    in              enables tracking time for the current day. Always start with this.
    out [arg]       registers elapsed time since last punch in until [now] as activity [arg].
    report          print a report for the current date.
    report [arg]    print a report for the provided date, e.g. "punch report 25-08-31".
    --help -h       shows this instruction.
    --version -v    shows the current version of punch.
    "#;
    println!("{help_text}");
}

fn print_version() {
    println!("punch version v{VERSION}",);
}

fn edit(input_date: String) {
    let date = Date::new(&input_date);
    if date.is_none() {
        println!("Error: Failed to parse date: [{input_date}].");
        return;
    }
    let date = date.unwrap();
    let full_path = date.get_file_path_for_date();
    if full_path.is_none() {
        println!(
            "Error: No report file for the provided date [{}-{}-{}].",
            date.year, date.month, date.day
        );
        return;
    }
    let full_path = full_path.unwrap();
    let mut command = EditorBuilder::edit_file(full_path).unwrap();
    command.status().unwrap();
    /*if let Ok(editor) = std::env::var("EDITOR") {
        std::process::Command::new(editor)
            .args([full_path])
            .output()
            .expect("Failed to edit time report.");
    } else {
        println!(
            "No default editor configured. Please ensure that the enviroment variable 'EDITOR' is set to your preferred editor."
        );
    }*/
}

fn print_report_for_date(input_date: String) {
    let date = Date::new(&input_date);
    if date.is_none() {
        println!("Error: Failed to parse date: [{input_date}].");
        return;
    }
    let date = date.unwrap();
    let full_path = date.get_file_path_for_date();
    if full_path.is_none() {
        println!(
            "Error: No report file for the provided date [{}-{}-{}].",
            date.year, date.month, date.day
        );
        return;
    }
    let full_path = full_path.unwrap();
    let contents = std::fs::read(full_path);
    match contents {
        Ok(contents) => {
            if let Ok(contents) = str::from_utf8(&contents) {
                if let Some(report) = parse_report_file(contents) {
                    println!("--- [{}-{}-{}] ---", date.year, date.month, date.day);
                    for activity in report.activities.keys().sorted() {
                        if let Some(duration) = chrono::Duration::new(
                            *report.activities.get(activity).unwrap() as i64,
                            0,
                        ) {
                            println!(
                                "[{activity}]: {} hours, {} minutes.",
                                duration.num_hours(),
                                duration.num_minutes() % 60
                            );
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!(
                "Error: No report found for [{}-{}-{}]: [{e}]!",
                date.year, date.month, date.day
            );
        }
    }
}

fn try_create_directory(directory: String) {
    match std::fs::exists(&directory) {
        Ok(exists) => {
            if exists {
                return;
            }
            if std::fs::create_dir_all(&directory).is_err() {
                println!("Error: Failed to create [{directory}]");
                return;
            }
        }
        Err(e) => {
            println!("Error creating working directory [{DIRECTORY}]: [{e:?}]");
        }
    }
}

fn get_today() -> String {
    chrono::Local::now().format("%Y-%m-%d").to_string()
}

fn get_todays_dir() -> String {
    let current_year = chrono::Local::now().format("%Y");
    let current_month = chrono::Local::now().format("%B");
    format!(
        "{}/{DIRECTORY}/{current_year}/{current_month}",
        std::env::var("HOME").unwrap()
    )
}

fn get_file_path_for_date(year: i32, month: u32, day: u32) -> Option<String> {
    if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
        let full_path = format!(
            "{}/{DIRECTORY}/{year}/{}/{}",
            std::env::var("HOME").unwrap(),
            date.format("%B"),
            date.format("%Y-%m-%d")
        );
        return Some(full_path);
    }
    println!("Error: Failed to parse date from: [{year}-{month}-{day}].");
    return None;
}

fn get_todays_file_path() -> String {
    format!("{}/{}", get_todays_dir(), get_today())
}

fn parse_report_file(contents: &str) -> Option<Report> {
    let mut activities = HashMap::default();
    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }
        let mut split = line.split(' ');
        let start = split.next();
        let _ = split.next(); // '-'
        let end = split.next();
        if start.is_none() || end.is_none() {
            println!("Error: Failed to parse report file: [{start:?}] - [{end:?}]");
            return None;
        }
        let start = start.unwrap();
        let end = end.unwrap();
        let activity = split.collect::<String>();
        let start = Duration::parse(&format!("{start}:00")).unwrap();
        let end = Duration::parse(&format!("{end}:00")).unwrap();
        if let Some(value) = activities.get_mut(&activity) {
            *value += end.as_seconds() - start.as_seconds();
        } else {
            activities.insert(activity.to_string(), end.as_seconds() - start.as_seconds());
        }
    }
    Some(Report { activities })
}
