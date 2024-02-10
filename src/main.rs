use chrono::NaiveDate;
use core::panic;
use itertools::Itertools;
use serde::Serialize;
use std::error::Error;
use std::{process::Command, str::FromStr};

#[derive(Debug)]
struct CommandOutput {
    date: NaiveDate,
    message: String,
    author: String,
}
impl CommandOutput {
    fn new(output: String) -> CommandOutput {
        let out_string = output.replace("'", "");
        let mut out = out_string.split(",");
        let date = match out.next() {
            Some(v) => v.to_string(),
            None => panic!("No date found"),
        };
        let message = match out.next() {
            Some(v) => v.to_string(),
            None => panic!("No message found"),
        };
        let author = match out.next() {
            Some(v) => v.to_string(),
            None => panic!("No author found"),
        };
        let date_string = NaiveDate::parse_from_str(&date, "%Y-%m-%d");

        let date = match date_string {
            Ok(date_string) => date_string,
            Err(..) => panic!("Wrong date format"),
        };
        CommandOutput {
            date,
            message,
            author,
        }
    }
}

#[derive(Debug, Serialize)]
struct IssueLog {
    issue: String,
    date: NaiveDate,
}
impl IssueLog {
    fn new((issue, date): (String, NaiveDate)) -> IssueLog {
        IssueLog { issue, date }
    }
}

fn get_git_logs(file_path: String) -> Vec<CommandOutput> {
    let output = Command::new(String::from("git"))
        .current_dir(file_path)
        .args(["log", "--pretty='%as,%s,%an'"])
        .output()
        .expect("failed to execute process");

    let result_output = String::from_utf8(output.stdout);
    let output = match result_output {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let commands_output = output
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| CommandOutput::new(x.to_string()))
        .collect::<Vec<CommandOutput>>();
    commands_output
}

fn filter_logs(
    command_logs: Vec<CommandOutput>,
    user_name: &String,
    date: &NaiveDate,
) -> Vec<CommandOutput> {
    command_logs
        .into_iter()
        .filter(|x| &x.date >= &date)
        .filter(|x| &x.author == user_name)
        .collect::<Vec<CommandOutput>>()
}

fn extract_issue_from_msg(command_log: &CommandOutput) -> (String, NaiveDate) {
    let msgs = &command_log.message.split("|").collect::<Vec<&str>>();
    let issue = match msgs.get(0) {
        Some(v) => v.trim(),
        None => panic!("No issue found"),
    };
    (issue.to_string(), command_log.date)
}

fn get_issue_logs(command_logs: Vec<CommandOutput>) -> Vec<IssueLog> {
    let mut issue_logs = Vec::new();
    let grouped_logs = command_logs
        .into_iter()
        .group_by(|x| extract_issue_from_msg(&x));
    for (key, _) in &grouped_logs {
        issue_logs.push(IssueLog::new(key));
    }
    issue_logs
}

fn run(issue_logs: Vec<IssueLog>, write_file_path: &String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_file_path)?;
    for issue_log in issue_logs {
        wtr.serialize(issue_log)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let example_file_path = String::from("/home/tomislav/Projects/test_project1");
    let outputs = get_git_logs(example_file_path);
    let date_result = NaiveDate::from_str("2023-11-01");
    let date = match date_result {
        Ok(date_result) => date_result,
        Err(..) => panic!("Error with the date"),
    };
    let user = String::from("AnixDrone");
    let result_file_path = String::from("worklog.csv");
    let mut logs = filter_logs(outputs, &user, &date);
    logs.sort_by_key(|a| extract_issue_from_msg(&a));

    let mut issue_logs = get_issue_logs(logs);
    issue_logs.sort_by_key(|a| a.date);
    if let Err(err) = run(issue_logs, &result_file_path) {
        println!("error running example: {}", err);
    }
}
