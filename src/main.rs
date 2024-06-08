use chrono::NaiveDate;
use core::panic;
use itertools::Itertools;
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{process::Command, str::FromStr};

mod models;
use crate::models::{CommandOutput, Config, IssueLog};

fn get_git_logs(file_path: String) -> Vec<CommandOutput> {
    let _git_pull = Command::new(String::from("git")).args(["pull"]);
    let output = Command::new(String::from("git"))
        .current_dir(file_path)
        .args(["log","--all","--pretty='%as,%s,%an'"])
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

fn extract_issue_from_msg(
    command_log: &CommandOutput,
    regex_pattern: &String,
) -> (String, NaiveDate) {
    let re = Regex::new(regex_pattern);
    let regex_object = match re {
        Ok(r) => r,
        Err(..) => panic!("Invalid regex pattern"),
    };
    let regex_operation: Option<regex::Match<'_>> = regex_object.find(&command_log.message);
    let result = match regex_operation {
        Some(res) => res.as_str().to_string(),
        None => String::from(""),
    };
    (result, command_log.date)
}

fn get_issue_logs(mut command_logs: Vec<CommandOutput>, regex_pattern: &String) -> Vec<IssueLog> {
    let mut issue_logs = Vec::new();
    command_logs.sort_by_key(|x| extract_issue_from_msg(&x, regex_pattern));
    let grouped_logs = command_logs
        .into_iter()
        .group_by(|x| extract_issue_from_msg(&x, regex_pattern));
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

fn get_logs_from_project(
    file_path: String,
    user: &String,
    date: &NaiveDate,
    regex_pattern: &String,
) -> Vec<IssueLog> {
    let outputs = get_git_logs(file_path);
    let mut logs = filter_logs(outputs, &user, &date);
    logs.sort_by_key(|a| extract_issue_from_msg(&a, regex_pattern));

    let mut issue_logs = get_issue_logs(logs, regex_pattern);
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
            .chain(get_logs_from_project(
                project,
                &content.author,
                &date,
                &content.regex_for_ticket_option,
            ))
            .collect();
    }
    project_logs.sort_by_key(|a| a.date);
    println!("{:?}", &project_logs);
    project_logs.sort_by_key(|a| (a.date, a.issue.clone()));
    project_logs.dedup_by_key(|a| (a.date, a.issue.clone()));
    project_logs.sort_by_key(|a| a.date);

    let filtered_logs = project_logs
        .into_iter()
        .filter(|x| x.issue.len() > 0)
        .collect::<Vec<IssueLog>>();
    let grouped_logs = filtered_logs.into_iter().group_by(|x| x.date);
    let stacked_logs: Vec<IssueLog> = grouped_logs
        .into_iter()
        .map(|x| IssueLog::collaps_logs(x.1.collect()))
        .collect();

    let result = write_to_file(stacked_logs, &content.save_file_path);
    match result {
        Ok(_) => println!("Worklog written to {}", &content.save_file_path),
        Err(e) => println!("Error writing to file: {}", e),
    }
}
