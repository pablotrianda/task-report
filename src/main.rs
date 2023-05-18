use std::fs;
use regex::Regex;
use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Cli{
    path: std::path::PathBuf,

    #[arg(short, long, default_value_t = false)]
    json_print: bool
}


fn main() {
    let args = Cli::parse();

    // Parameters
    let dir_entries = fs::read_dir(&args.path).unwrap();
    let json_print = &args.json_print;

    // Get all files from directory
    let mut dir_entries_sorted = dir_entries
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
        
    dir_entries_sorted.sort_by_key(|entry| entry.path());

    let last_file: &str = &dir_entries_sorted.last().expect("El vector dir_entries_sorted está vacío.").path().display().to_string();


    if *json_print { println!("{}","[") }
    for entry in dir_entries_sorted {
        let file_name = &entry.path().display().to_string();
        read_a_file(file_name, *json_print);
        if last_file == file_name  {
            println!("{}", r"");
        } else {
            println!("{}", r",");
        }
    }
    if *json_print { println!("{}","]") }

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
fn read_a_file(note_file_name: &str, json_print: bool){
    let content = fs::read_to_string(note_file_name)
        .expect("Error to read the file");

    let v: Vec<&str> = content.split("Today:").collect();

    let today: Vec<&str>= v.last().unwrap().split('*').collect();

    if json_print { print!("{}","{") }
    print_title(note_file_name, json_print);
    print_tasks(today, json_print);
    if json_print { print!("{}","}") }
}

// Print the title, in this case is the file date
// The date is displayed in the following format: 
//          dd-mm-yyyy
// And the file names have the following format:
//          yyyymmdd_notes.md
fn print_title(note_file_name: &str, json_print: bool){
    let re = Regex::new(r"\d{8}").unwrap();
    let title = re.captures(note_file_name).unwrap();
    let full_date = title.get(0).map_or("", |m| m.as_str());
    let year = &full_date[0..4];
    let month = &full_date[4..6];
    let day = &full_date[6..8];

    let show_date = day.to_owned()+"-"+month +"-"+year;

    if json_print {
        print!("{}",format!("\"date\":\"{}\",", &show_date));
    }else{
        println!("{}:", show_date.bold().cyan());
    }
}

// Print the task number, this number correspond to Jira tikect
// All tickets begin with the following code: MBM-1234
fn print_tasks(tasks: Vec<&str>, json_print: bool){
    let re = Regex::new(r"MBM-\d{4}").unwrap();
    let mut ticket_numbers = Vec::new();

    // Get only the codes tickets
    for t in &tasks {
        if re.is_match(t){
            let ticket_number = re.captures(t).unwrap();
            ticket_numbers.push(ticket_number.get(0).map_or("", |m| m.as_str()));
        }
    }

    if json_print {
        let quoted_ticket_numbers: Vec<String> = ticket_numbers.iter().map(|&s| format!("\"{}\"", s)).collect();
        print!("\"tasks\":[{}]",quoted_ticket_numbers.join(","));
    }else{
        let result = ticket_numbers.join(", ");
        print!("{}",result);
    }
}
