use std::process::*;
use std::env;
use std::io::BufRead;
use std::collections::HashMap;

fn get_filename(args: &Vec<String>) -> Result<String, String> {
  let argument = args.iter().skip(1).next();
  match argument {
    Some(arg) => Ok(arg.clone()),
    None      => Err(String::from("missing command"))
  }
}

fn main() {
    let args = env::args().collect();
    let target_filename = get_filename(&args).unwrap_or_else(|e| { panic!("cannot read arguments '{}'", e) });
    let log_output = Command::new("git")
        .arg("log").arg("--name-only").arg("--pretty=format:")
        .output().unwrap_or_else(|e| { panic!("failed to execute process '{}'", e) });

    let mut matchcount: HashMap<String, i32> = HashMap::new();

    let mut contains_target = false;
    let mut commit: Vec<String> = Vec::new();

    for line in log_output.stdout.lines() {
        let filename = line.unwrap();
        if filename.len() == 0 {
            if contains_target {
                for file in commit.iter() {
                    let count = matchcount.entry(file.clone()).or_insert(0);
                    *count += 1;
                }
                contains_target = false;
            }
            commit.clear();
        } else {
            if target_filename == filename {
                contains_target = true;
            } else {
                commit.push(filename.clone());
            }
        }
    }
    let mut entries: Vec<(&String, &i32)> = matchcount.iter().collect();
    entries.sort_by(|e1, e2| e1.1.cmp(e2.1).reverse());
    for entry in entries.iter().take(10) {
        println!("{} {:>5}", entry.1, entry.0);
    }
}
