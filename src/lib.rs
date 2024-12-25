pub mod error {
    use std::io;

    pub enum JumpError {
        JumpPathAlreadyExists,
        JumpPathNotExists,
        NoHomeDirectory,
        HomeDirectoryAlreadyExists,
        SaveFileAlreadyExists,
        CantParseSaveFile(String),
        Other(String),
    }
    /*
     2. `JumpError` doesn't implement `std::fmt::Display`
        the trait `std::fmt::Display` is not implemented for `JumpError`
        in format strings you may be able to use `{:?}`
        (or {:#?} for pretty-print) instead [E0277]
    */
    impl std::fmt::Display for JumpError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                JumpError::JumpPathAlreadyExists => write!(f, "Jump path already exists!"),
                JumpError::JumpPathNotExists => write!(f, "Cant modify unexisting path!"),
                JumpError::NoHomeDirectory => write!(f, "No home directory was found!"),
                JumpError::HomeDirectoryAlreadyExists => {
                    write!(f, "Home directory already exists!")
                }
                JumpError::SaveFileAlreadyExists => write!(f, "Save file already exists!"),
                JumpError::CantParseSaveFile(error) => {
                    write!(f, "Cant parse save file, jso parser error: {}", error)
                }
                JumpError::Other(error) => write!(f, "Other error: {}", error),
            }
        }
    }

    impl From<JumpError> for std::io::Error {
        fn from(value: JumpError) -> Self {
            io::Error::new(io::ErrorKind::Other, format!("JumpError: {}", value))
        }
    }
}

pub mod fs {

    extern crate home;
    extern crate json;

    use crate::error::JumpError;
    use std::{collections::HashMap, fs, io, path};

    pub fn create_save_folder() -> Result<path::PathBuf, JumpError> {
        let home_dir_path = match home::home_dir() {
            Some(v) => v,
            None => return Err(JumpError::NoHomeDirectory),
        };

        let initial_abs = home_dir_path.join(path::PathBuf::from(".jump"));
        match fs::create_dir(&initial_abs) {
            Ok(_) => Ok(initial_abs),
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists => Ok(initial_abs),
                _ => Err(JumpError::HomeDirectoryAlreadyExists),
            },
        }
    }

    pub fn create_save_file() -> Result<path::PathBuf, JumpError> {
        let save_file_path = create_save_folder()?.join("jumps.json");
        match fs::File::create_new(&save_file_path) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists => {}
                _ => return Err(JumpError::Other(e.to_string())),
            },
        };

        Ok(save_file_path)
    }

    pub fn read_save_file(
        save_file_path: &path::PathBuf,
    ) -> Result<HashMap<String, String>, JumpError> {
        let file_str = match fs::read_to_string(save_file_path) {
            Ok(v) => v,
            Err(e) => return Err(JumpError::Other(e.to_string())),
        };
        let json_object = match json::parse(&file_str) {
            Ok(v) => v,
            Err(e) => return Err(JumpError::CantParseSaveFile(e.to_string())),
        };
        let mut data = HashMap::new();
        for (key, value) in json_object.entries().into_iter() {
            data.insert(key.to_string(), value.to_string());
        }
        Ok(data)
    }
}

pub mod data {
    use crate::error::JumpError;
    use core::fmt;
    use std::{
        collections::HashMap,
        io::{self, Write},
        path,
    };
    pub struct JumpData {
        paths: HashMap<String, String>,
    }

    impl JumpData {
        pub fn build(save_file_path: &path::PathBuf) -> Result<JumpData, JumpError> {
            Ok(JumpData {
                paths: crate::fs::read_save_file(&save_file_path)?,
            })
        }
        pub fn new() -> JumpData {
            JumpData {
                paths: HashMap::new(),
            }
        }
        pub fn get(&self, key: &str) -> Option<&String> {
            self.paths.get(key)
        }
        pub fn insert(&mut self, key: &str, path: &str) -> Result<(), JumpError> {
            if self.paths.contains_key(key) {
                return Err(JumpError::JumpPathAlreadyExists);
            }
            self.paths.insert(String::from(key), String::from(path));
            Ok(())
        }
        pub fn change(&mut self, key: &String, path: String) -> Result<(), JumpError> {
            if !self.paths.contains_key(key) {
                return Err(JumpError::JumpPathNotExists);
            }
            self.paths.insert(key.clone(), path.clone()).unwrap();
            Ok(())
        }
        pub fn save(&self, save_file_path: &path::PathBuf) -> Result<(), JumpError> {
            let mut json_object = json::object::Object::new();
            for (key, value) in self.paths.iter() {
                json_object.insert(&key, json::JsonValue::from(value.clone()));
            }
            let json_object_str = json::stringify_pretty(json_object, 4);
            let mut file = std::fs::File::create(&save_file_path)
                .map_err(|e| JumpError::Other(e.to_string()))?;
            write!(file, "{}", json_object_str).map_err(|e| JumpError::Other(e.to_string()))?;
            Ok(())
        }
        pub fn remove(&mut self, key: &str) -> Result<(), JumpError> {
            if !self.paths.contains_key(key) {
                return Err(JumpError::JumpPathNotExists);
            }
            self.paths.remove(key);
            Ok(())
        }
    }
    impl fmt::Debug for JumpData {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("JumpData")
                .field("paths", &self.paths)
                .finish()
        }
    }
}
