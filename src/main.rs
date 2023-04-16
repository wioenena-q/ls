use colored::Colorize;
use std::{env, fs, path::Path};

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let len = args.len();
    if len < 2 {
        panic!("Please provide a path to a directory");
    }

    let mut path: Option<&Path> = None;
    let mut i = 1;
    let mut list_hidden = false;
    let mut is_recursive = false;

    loop {
        if i >= len {
            break;
        }
        let arg = args[i].as_str();
        match arg {
            "-P" | "--path" => {
                if i + 1 < len {
                    path = Some(Path::new(args[i + 1].as_str()));
                    i += 2;
                    continue;
                } else {
                    panic!("Please provide a path to a directory");
                }
            }

            "--list-hidden" => {
                list_hidden = true;
                i += 1;
                continue;
            }

            "-R" | "--recursive" => {
                is_recursive = true;
                i += 1;
                continue;
            }

            _ => {
                path = Some(Path::new(arg));
                i += 1;
            }
        }
    }

    if path.is_none() {
        panic!("Please provide a path to a directory");
    }

    list_dir(path.unwrap(), list_hidden, is_recursive);
}

fn list_dir(path: &Path, list_hidden: bool, is_recursive: bool) {
    let mut dir_files = fs::read_dir(path)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<_>>();
    if !list_hidden {
        dir_files = dir_files
            .into_iter()
            .filter(|p| {
                let file_name = p.file_name().unwrap().to_str().unwrap();
                !file_name.starts_with(".")
            })
            .collect::<Vec<_>>();
    }

    for file in dir_files {
        let file_name = file.file_name().unwrap().to_str().unwrap();
        if file.is_dir() {
            println!(
                "{}{}{}",
                path.to_str().unwrap().blue().bold(),
                "/".blue().bold(),
                file_name.blue().bold()
            );
            if is_recursive {
                list_dir(&file, list_hidden, is_recursive);
            }
        } else {
            println!(
                "{}{}{}",
                path.to_str().unwrap().blue().bold(),
                "/".blue().bold(),
                file_name
            );
        }
    }
}
