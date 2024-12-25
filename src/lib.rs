extern crate home;
extern crate json;
use std::{
    collections::HashMap,
    fs,
    io::{self},
    path,
};

pub fn create_save_folder() -> Result<path::PathBuf, io::Error> {
    let home_dir_path = home::home_dir().ok_or_else(|| {
        return io::Error::new(io::ErrorKind::NotFound, "Could find home directory");
    })?;
    let initial_abs = home_dir_path.join(path::PathBuf::from(".jump"));
    match fs::create_dir(&initial_abs) {
        Ok(_) => Ok(initial_abs),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => Ok(initial_abs),
            _ => Err(e),
        },
    }
}

pub fn create_save_file() -> Result<path::PathBuf, io::Error> {
    let save_file_path = create_save_folder()?.join("jumps.json");
    match fs::File::create_new(&save_file_path) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => {}
            _ => return Err(e),
        },
    };
    Ok(save_file_path)
}

pub fn read_save_file(
    save_file_path: &path::PathBuf,
) -> Result<HashMap<String, String>, io::Error> {
    let file_str = fs::read_to_string(save_file_path)?;
    let json_object = match json::parse(&file_str) {
        Ok(v) => v,
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Can't parse save file: {}", e),
            ))
        }
    };
    let mut data = HashMap::new();
    for (key, value) in json_object.entries().into_iter() {
        data.insert(key.to_string(), value.to_string());
    }
    Ok(data)
}
