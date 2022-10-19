// use std::{fs::File,io::{self, BufRead, BufReader},path::Path};
use std::path::Path;

pub fn directory_from_file(file: &str) -> String {
    match file.rfind('/') {
        Some(i) => file.split_at(i).0,
        None => ""
    }.to_string()
}


pub fn find_other_todo_lists(current_todo_list: &Path) -> Vec<String> {
    let current_todo_filename = current_todo_list.file_name().unwrap().to_str().unwrap();
    // println!("{:?}", current_todo_list);
    // let file_prefix = 
    let directory = directory_from_file(current_todo_list.display().to_string().as_str());
    Path::new(&directory).read_dir().unwrap()
        .flat_map(|f| f)
        .map(|f| f.file_name().to_str().unwrap().to_string())
        .filter(|f| f.starts_with("todo-") && f.ends_with(".md"))
        .filter(|f| !f.eq(current_todo_filename))
        .collect()
}
