use chrono::{DateTime, TimeZone, Local};
use colored::Colorize;
use regex::Regex;
use std::{fs::File, path::PathBuf, process::Command, str};

enum Script {
    Path(PathBuf),
    Name(File),
    Ext(String),
    Flag(String),
    Running(Status),
    Date,
}

struct Status {
    code: u8,
    date: String,
}

struct Output;

trait cleaner {
    fn clean_whitespace<'a>(text: &'a str) -> Vec<&'a str>;
    fn clean_newline<'b>(output: &'b str) -> Vec<&'b str>;
}

impl cleaner for Output {
    fn clean_whitespace<'a>(text: &'a str) -> Vec<&'a str> {
        let white_space = Regex::new(r"([ ]{2,})").expect("Regex incorrect");

        let mut result = Vec::new();
        let mut last = 0;
        for (index, matched) in text.match_indices(&white_space) {
            if last != index {
                result.push(&text[last..index]);
            }
            last = index + matched.len();
        }
        if last < text.len() {
            result.push(&text[last..]);
        }
        result
    }

    fn clean_newline<'b>(output: &'b str) -> Vec<&'b str> {
        output.split("\n").collect()
    }
}

struct DockerInfo<'a>
{
    container_id: String,
    image: String,
    command: Commands<'a>,
    created: DateTime<Local>,
    status: Status,
    ports: u16,
    name: String,
}

impl<'a> DockerInfo<'a> {
    fn new(
        container: String,
        image: String,
        command: Commands<'a>,
        date: DateTime<Local>,
        status: Status,
        port: u16,
        name: String,
    ) -> DockerInfo<'a> {
        DockerInfo {
            container_id: container,
            image: image,
            command: command,
            created: date,
            status: status,
            ports: port,
            name: name,
        }
    }
}

struct Docker;

impl Docker {
    fn comm(flags: Vec<&str>) {
        for f in flags {
            match f {
                "l" | "-l" | "--list" => {
                    
                    let mut info = Command::new("docker")
                        .arg("ps")
                        .arg("--all")
                        .output()
                        .expect("ls command failed to start");

                    let date = chrono::offset::Local::now();

                    let output = match str::from_utf8(&info.stdout) {
                        Ok(x) => x,
                        Err(e) => panic!(""),
                    };

                    let images: Vec<&str> = Output::clean_newline(&output);

                    for i in images {
                        let image = Output::clean_whitespace(&i);

                        
                    }
                }
                _ => panic!(),
            }
        }
    }
}

pub struct Commands<'a> {
    Cmd: &'a str,
    Flags: Vec<&'a str>,
}

impl<'a> Commands<'a> {
    pub fn find_command(comm: &str, flags: Vec<&str>) {
        let comm = Commands {
            Cmd: comm,
            Flags: flags,
        };

        match comm.Cmd {
            "docker" => {
                Docker::comm(comm.Flags);
            }
            "v" => {}
            _ => panic!(),
        }
    }
}
