use chrono::NaiveDate;
use core::panic;
use itertools::Itertools;
use std::error::Error;
use std::{process::Command, str::FromStr};

mod models;
use crate::models::{CommandOutput, IssueLog};

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

fn write_to_file(
    issue_logs: Vec<IssueLog>,
    write_file_path: &String,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_file_path)?;
    for issue_log in issue_logs {
        wtr.serialize(issue_log)?;
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let example_file_path = String::from("/Users/tomislav/Projects/xarvio/hrl-config");
    let outputs = get_git_logs(example_file_path);
    let date_result = NaiveDate::from_str("2023-11-01");
    let date = match date_result {
        Ok(date_result) => date_result,
        Err(..) => panic!("Error with the date"),
    };
    let user = String::from("IgnjatT");
    let result_file_path = String::from("worklog.csv");
    let mut logs = filter_logs(outputs, &user, &date);
    logs.sort_by_key(|a| extract_issue_from_msg(&a));

    let mut issue_logs = get_issue_logs(logs);
    issue_logs.sort_by_key(|a| a.date);
    if let Err(err) = write_to_file(issue_logs, &result_file_path) {
        println!("error running example: {}", err);
    }
}
