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

fn main() {
    let example_file_path = "/Users/tomislav/Projects/xarvio/hrl-config";
    let git_command = String::from("git");
    let output = Command::new(git_command)
        .current_dir(example_file_path)
        .args(["log", "-2", "--pretty='%as,%f,%an'"])
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(output.stdout);
    let real_output = match output {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let output = real_output
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| CommandOutput::new(x.to_string()))
        .collect::<Vec<CommandOutput>>();
    println!("{:?}", output);
    let dx = NaiveDate::parse_from_str("2024-02-08", "%Y-%m-%d");

    let date_x = match dx {
        Ok(dx) => dx,
        Err(..) => panic!("Wrong date format"),
    };
    println!("{:?}", date_x)
}
