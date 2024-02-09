use chrono::NaiveDate;
use core::panic;
use std::process::Command;

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

fn get_git_logs(file_path: String) -> Vec<CommandOutput> {
    let output = Command::new(String::from("git"))
        .current_dir(file_path)
        .args(["log", "-2", "--pretty='%as,%f,%an'"])
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

fn main() {
    let example_file_path = String::from("/Users/tomislav/Projects/xarvio/hrl-config");
    let outputs = get_git_logs(example_file_path);
    println!("{:?}", outputs)
}
