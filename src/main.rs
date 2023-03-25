use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let note_dir_name = &args[1];

    let dir_entries = fs::read_dir(note_dir_name).unwrap();
    
    let mut dir_entries_sorted = dir_entries
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
        
    dir_entries_sorted.sort_by_key(|entry| entry.path());
    
    for entry in dir_entries_sorted {
        read_a_file(&entry.path().display().to_string());
    }

}

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

    println!("{}-{}-{}", day, month, year);
}

// Print the task number, this number correspond to Jira tikect
fn print_tasks(tasks: Vec<&str>){
    let re = Regex::new(r"MBM-\d{4}").unwrap();

    for t in &tasks {
        if re.is_match(t){
            let ticket_number = re.captures(t).unwrap();
            println!("\t{:?}",ticket_number.get(0).map_or("", |m| m.as_str()));
        }
    }
}
