use clap::Parser;
use colored::*;
use regex::Regex;
use std::fs;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[arg(short, long, default_value_t = false)]
    json_print: bool,
}

struct TicketData {
    title: String,
    tickets: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    // Parameters
    let dir_entries = fs::read_dir(&args.path).unwrap();
    let json_print = &args.json_print;

    // Get all files from directory
    let mut dir_entries_sorted = dir_entries
        .map(|entry| entry.unwrap().path().display().to_string())
        .collect::<Vec<_>>();
    dir_entries_sorted.sort();

    let mut tickets: Vec<TicketData> = Vec::new();

    for entry in dir_entries_sorted {
        tickets.push(read_a_file(&entry));
    }

    if *json_print {
        print_json(tickets);
    } else {
        print_data(tickets);
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
//  Return the title and tick by each day
fn read_a_file(note_file_name: &String) -> TicketData {
    let content = fs::read_to_string(note_file_name).expect("Error to read the file");

    let v: Vec<&str> = content.split("Today:").collect();

    let today_tasks: Vec<&str> = v.clone().last().unwrap().split('*').collect();

    // Get data from file
    let title: String = print_title(note_file_name);
    let ticket_numbers: Vec<String> = print_tasks(today_tasks);

    TicketData {
        title,
        tickets: ticket_numbers,
    }
}

// Print the title, in this case is the file date
// The date is displayed in the following format:
//          dd-mm-yyyy
// And the file names have the following format:
//          yyyymmdd_notes.md
fn print_title(note_file_name: &str) -> String {
    let re = Regex::new(r"\d{8}").unwrap();
    let title = re.captures(note_file_name).unwrap();
    let full_date = title.get(0).map_or("", |m| m.as_str());
    let year = &full_date[0..4];
    let month = &full_date[4..6];
    let day = &full_date[6..8];

    let show_date = day.to_owned() + "-" + month + "-" + year;

    show_date
}

// Print the task number, this number correspond to Jira tikect
// All tickets begin with the following code: XXXX-1234
fn print_tasks(tasks: Vec<&str>) -> Vec<String> {
    let re = Regex::new(r"([A-Z]+)-\d{4}").unwrap();
    let mut ticket_numbers = Vec::new();

    // Get only the codes tickets
    for t in &tasks {
        if re.is_match(t) {
            let ticket_number = re.captures(t).unwrap();
            ticket_numbers.push(String::from(
                ticket_number.get(0).map_or("", |m| m.as_str()),
            ));
        }
    }

    return ticket_numbers.clone();
}

fn print_data(tasks: Vec<TicketData>) {
    for day in tasks {
        print!("{}:", day.title.bold().cyan());
        print!("{} \n", day.tickets.join(", "));
    }
}

fn print_json(tasks: Vec<TicketData>) {
    println!("{}", "[");
    let last_element = String::from(&tasks.last().unwrap().title);
    for day in tasks {
        // Element
        print!("{}", "{");
        // Date
        print!("{}", format!("\"date\":\"{}\",", day.title));
        // Ticket numbers
        let quoted_ticket_numbers: Vec<String> =
            day.tickets.iter().map(|s| format!("\"{}\"", s)).collect();
        print!("\"tasks\":[{}]", quoted_ticket_numbers.join(","));
        // End element
        // Check if is the last element to avaoid trailing comma
        if last_element == day.title {
            print!("{}", "}");
        } else {
            print!("{},", "}");
        }
    }
    println!("{}", "]");
}
