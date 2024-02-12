use chrono::NaiveDate;
use core::panic;
use serde::Serialize;

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
    pub issue: String,
    pub date: NaiveDate,
}
impl IssueLog {
    pub fn new((issue, date): (String, NaiveDate)) -> IssueLog {
        IssueLog { issue, date }
    }
}
