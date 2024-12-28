use jump::error::JumpError;
use jump::fs;
use jump::io::{gen_opts, parse_args, Scenario};
use std::io;

fn main() -> Result<(), io::Error> {
    let save_file_path = fs::create_save_file()?;
    let mut data = jump::data::JumpData::build(&save_file_path)?;
    let args: Vec<String> = std::env::args().into_iter().collect();
    let opts = gen_opts();
    let scenario = parse_args(&opts, &args)?;
    match scenario {
        Scenario::Sleep => {}
        Scenario::HelpMe => jump::io::process_help(&opts, &args[0]),
        Scenario::ShowSaved => print!("{}", data),
        Scenario::AddJumpCurrent(name) => {
            let current_path = match std::fs::canonicalize(std::env::current_dir().unwrap()) {
                Ok(v) => v,
                Err(e) => panic!("Adding error: {}", e.to_string()),
            };
            data.insert(&name, current_path.to_str().unwrap())?;
        }
        Scenario::AddJumpWithTarget(name, target) => data.insert(&name, &target)?,
        Scenario::RemoveJump(name) => data.remove(&name)?,
        Scenario::GoJump(name) => {
            let path_to_jump = data
                .get(&name)
                .ok_or_else(|| return JumpError::JumpPathNotExists)?;
            fs::jump_print(&path_to_jump)?
        }
    };
    data.save(&save_file_path)?;
    Ok(())
}
