use chrono::NaiveDate;
use core::panic;
use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{process::Command, str::FromStr};

mod models;
use crate::models::{CommandOutput, Config, IssueLog};

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

fn get_logs_from_project(file_path: String, user: &String, date: &NaiveDate) -> Vec<IssueLog> {
    let outputs = get_git_logs(file_path);
    let mut logs = filter_logs(outputs, &user, &date);
    logs.sort_by_key(|a| extract_issue_from_msg(&a));

    let mut issue_logs = get_issue_logs(logs);
    issue_logs.sort_by_key(|a| a.date);
    issue_logs
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config_filepath = match args.get(1) {
        Some(v) => v,
        None => panic!("No config file provided"),
    };
    let config = read_user_from_file(config_filepath);
    let content = match config {
        Ok(e) => e,
        Err(e) => panic!("Error reading config file: {}", e),
    };
    println!("Found config: {:?}", content);

    let mut project_logs: Vec<IssueLog> = Vec::new();
    for project in content.repositories {
        let project_date = NaiveDate::from_str(&content.date);
        let date = match project_date {
            Ok(d) => d,
            Err(..) => panic!("Error parsing the date"),
        };
        project_logs = project_logs
            .into_iter()
            .chain(get_logs_from_project(project, &content.author, &date))
            .collect();
    }
    project_logs.sort_by_key(|a| (a.date, a.issue.clone()));
    project_logs.dedup_by_key(|a| (a.date, a.issue.clone()));
    project_logs.sort_by_key(|a| a.date);
    let result = write_to_file(project_logs, &content.save_file_path);
    match result {
        Ok(_) => println!("Worklog written to {}", &content.save_file_path),
        Err(e) => println!("Error writing to file: {}", e),
    }
}
