use std::fs;
use regex::Regex;
use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Cli{
    path: std::path::PathBuf
}


fn main() {
    let args = Cli::parse();

    let dir_entries = fs::read_dir(&args.path).unwrap();
    
    let mut dir_entries_sorted = dir_entries
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
        
    dir_entries_sorted.sort_by_key(|entry| entry.path());
    
    for entry in dir_entries_sorted {
        read_a_file(&entry.path().display().to_string());
    }

}

// Read the file with the given name.
// only read the part what begins with "Today" tag.
// The file struct is:
//  ```
//  Daily:
//      * Item to chat in the daily meeting
//  Today:
//      * Item was I currently work
//  ````
fn read_a_file(note_file_name: &str){
    let content = fs::read_to_string(note_file_name)
        .expect("Error to read the file");

    let v: Vec<&str> = content.split("Today:").collect();

    let today: Vec<&str>= v.last().unwrap().split('*').collect();

    print_title(note_file_name);
    print_tasks(today)
}

// Print the title, in this case is the file date
// The date is displayed in the following format: 
//          dd-mm-yyyy
// And the file names have the following format:
//          yyyymmdd_notes.md
fn print_title(note_file_name: &str){
    let re = Regex::new(r"\d{8}").unwrap();
    let title = re.captures(note_file_name).unwrap();
    let full_date = title.get(0).map_or("", |m| m.as_str());
    let year = &full_date[0..4];
    let month = &full_date[4..6];
    let day = &full_date[6..8];

    let show_date = day.to_owned()+"-"+month +"-"+year;

    println!("{}:", show_date.bold().cyan());
}

// Print the task number, this number correspond to Jira tikect
// All tickets begin with the following code: MBM-1234
fn print_tasks(tasks: Vec<&str>){
    let re = Regex::new(r"MBM-\d{4}").unwrap();

    for t in &tasks {
        if re.is_match(t){
            let ticket_number = re.captures(t).unwrap();
            println!("\t{:?}",ticket_number.get(0).map_or("", |m| m.as_str()));
        }
    }
}
