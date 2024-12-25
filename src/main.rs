use jump::{create_save_file, read_save_file};
use std::{collections::HashMap, io};

fn main() -> Result<(), io::Error> {
    let save_file_path = create_save_file()?;

    let jumps: HashMap<String, String> = read_save_file(&save_file_path)?;
    // jumps.insert("1".to_string(), "2".to_string());
    // jumps.insert("3".to_string(), "4".to_string());
    // jumps.insert("5".to_string(), "6".to_string());

    let ostr = json::stringify_pretty(jumps, 3);
    println!("{}", &ostr);

    Ok(())
}
