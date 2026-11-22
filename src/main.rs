use std::env;

fn main() {
  display_sys_infos();
}

fn display_sys_infos() {
  println!("Current directory: {}", current_dir());
  println!("Current exe: {}", current_exe());
}

fn current_dir() -> String {
      match env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(e) => {
          eprintln!("Error getting current directory: {}", e);
          return String::new();
        },
    }
}

fn current_exe() -> String {
      match env::current_exe() {
        Ok(path) => path.display().to_string(),
        Err(e) => {
          eprintln!("Error getting current directory: {}", e);
          return String::new();
        },
    }
}