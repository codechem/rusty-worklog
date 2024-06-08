# Rusty Worklog

This is a simple worklog tool written in Rust. It is designed to be a simple.

## Usage

The usage is simple. You can run the program with a configuration file as an argument.There is a sample configuration file in the repository under the name example_config.json. You can use this file as a template for your own configuration file.

To run the program, you can use the following command:
```bash
cargo run config.json
```

## Configuration
The idea is to have multiple project per configuration file. The configuration file is a json file with the following structure:
```json
{
  "projectName": "The name of the overall project",
  "author": "YourGitUsername",
  "date": "yyyy-mm-dd",
  "repositories": [
    "filepath/to/project",
    "filepath/to/project1",
    "filepath/to/project2"
  ],
  "saveFilePath": "filepath/to/save/sheet.csv",
  "regexForTicketOption": "This is a regex pattern to match the ticket number and name",
}
```

## Output
The output is a csv file with the following structure:
```csv
date,issue
2024-05-01,DFWM-0002
2024-05-21,DFWM-0003
2024-05-27,DFWM-0004
2024-05-28,DFWM-0005
```

## Development
To develop the project and to debug this project with vscode you can use this template to setup rust debugger.
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Rusty Worklog",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/target/debug/rusty_worklog",
            "args": ["${workspaceFolder}/example_config.json"],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```