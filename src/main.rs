use std::env;

mod lib;

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
    let matchcount = lib::find_files_committed_together(target_filename.as_str());
    lib::print_matchcount(&matchcount);
}
