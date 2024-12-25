use jump::fs;
use std::{collections::HashMap, io};

fn main() -> Result<(), io::Error> {
    let save_file_path = fs::create_save_file()?;
    let mut data = jump::data::JumpData::build(&save_file_path)?;
    println!("{:?}", data);
    // data.insert("1000", "1000")?;
    match data.remove("1000") {
        Ok(_) => {}
        Err(_) => {}
    }
    println!("{:?}", data);
    data.save(&save_file_path)?;

    Ok(())
}
