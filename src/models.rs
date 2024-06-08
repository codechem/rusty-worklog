use chrono::NaiveDate;
use core::panic;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct CommandOutput {
    pub date: NaiveDate,
    pub message: String,
    pub author: String,
}
impl CommandOutput {
    pub fn new(output: String) -> CommandOutput {
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
pub struct IssueLog {
    pub date: NaiveDate,
    pub issue: String,
}
impl IssueLog {
    pub fn new((issue, date): (String, NaiveDate)) -> IssueLog {
        IssueLog { issue, date }
    }
    pub fn collaps_logs(issues: Vec<IssueLog>) -> IssueLog {
        let date = match issues.get(0) {
            Some(issue) => issue.date,
            None => panic!("Empty list"),
        };

        let raw_issue = issues
            .into_iter()
            .map(|x| x.issue)
            .collect::<Vec<String>>()
            .join(";");
        IssueLog {
            issue: raw_issue,
            date,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub author: String,
    pub project_name: String,
    pub repositories: Vec<String>,
    pub date: String,
    pub save_file_path: String,
    pub regex_for_ticket_option: String,
}
