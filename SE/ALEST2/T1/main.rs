use std::{
    fs::File,
    path::Path,
    time::Instant,
    collections::VecDeque,
    io::{BufRead, BufReader, Lines, Result},
};

const HI: usize = 1;
const LO: usize = 0;
const FILES_PATH: [&str; 13] = [
    "data/cohen00.txt",
    "data/cohen01.txt",
    "data/cohen02.txt",
    "data/cohen03.txt",
    "data/cohen04.txt",
    "data/cohen05.txt",
    "data/cohen06.txt",
    "data/cohen07.txt",
    "data/cohen08.txt",
    "data/cohen09.txt",
    "data/cohen10.txt",
    "data/cohen11.txt",
    "data/cohen12.txt",
];
const FILES_NAME: [&str; 13] = [
    "casoenunciado.txt",
    "cohen01.txt",
    "cohen02.txt",
    "cohen03.txt",
    "cohen04.txt",
    "cohen05.txt",
    "cohen06.txt",
    "cohen07.txt",
    "cohen08.txt",
    "cohen09.txt",
    "cohen10.txt",
    "cohen11.txt",
    "cohen12.txt",
];

fn main() {
    loop {
        let option = menu();
        if option == 3 {
            break;
        }
    }
}

fn menu() -> usize {
    show_menu_options();
    let option = read_user_input();
    match option {
        1 => {
            choose_one_file();
        }
        2 => {
            run_all_files();
        }
        3 => {
            println!();
            println!(">>> Bye ^_~");
        }
        _ => {
            println!();
            println!("Invalid option. Try again.");
        }
    }
    option
}

fn show_menu_options() {
    println!("Hi! What do you want to do?");
    println!("1 - Run one file");
    println!("2 - Run all files");
    println!("3 - Quit");
}

fn choose_one_file() {
    loop {
        show_file_options();
        let chosen_file = read_user_input();
        if chosen_file < 13 {
            process_single_file(chosen_file);
            break;
        }
        println!("Please, select a valid number.");
    }
}

fn show_file_options() {
    println!();
    println!("Available files:");
    for (index, file_name) in FILES_NAME.iter().enumerate() {
        println!(">>> {} - {}", index, file_name)
    }
}

fn read_user_input() -> usize {
    println!();
    println!("Input selected file number: ");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<usize>() {
        Ok(parsed_string) => parsed_string,
        Err(_) => 13,
    }
}

fn process_single_file(chosen_file: usize) {
    let start = Instant::now();
    let answer = least_possible_list_size(get_sorted_ip_list(FILES_PATH[chosen_file]));
    let duration = start.elapsed();
    println!();
    println!("Time to process file: {:?}", duration);
    println!(">>> {} => {}", FILES_PATH[chosen_file], answer);
}

fn get_sorted_ip_list(file_path: &str) -> Vec<Vec<i32>> {
    let mut ip_list = Vec::<Vec<i32>>::new();
    let file_path = Path::new(file_path);
    for line in read_lines(file_path).unwrap() {
        ip_list.push(
            line.unwrap()
                .split('-')
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|string| string.parse::<i32>().unwrap())
                .collect(),
        );
    }
    ip_list.sort();
    ip_list
}

fn read_lines(filename: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename).expect("Error opening file.");
    Ok(BufReader::new(file).lines())
}

fn least_possible_list_size(ip_list: Vec<Vec<i32>>) -> usize {
    let mut blocked_ranges = VecDeque::<Vec<i32>>::new();

    for ip_pair in ip_list {
        if blocked_ranges.is_empty() {
            blocked_ranges.push_back(ip_pair);
        } else if change_higher_value(&ip_pair, &blocked_ranges.back().unwrap()) {
            blocked_ranges.back_mut().unwrap()[HI] = ip_pair[HI];
        } else if !contained(&ip_pair, &blocked_ranges.back().unwrap()) {
            blocked_ranges.push_back(ip_pair);
        }
    }
    blocked_ranges.len()
}

fn change_higher_value(interval_1: &Vec<i32>, interval_2: &Vec<i32>) -> bool {
    return interval_2[LO] < interval_1[LO]
        && interval_1[LO] < interval_2[HI]
        && interval_2[HI] <= interval_1[HI];
}

fn contained(interval_1: &Vec<i32>, interval_2: &Vec<i32>) -> bool {
    return interval_2[LO] < interval_1[LO] && interval_1[HI] < interval_2[HI];
}

fn run_all_files() {
    let start = Instant::now();
    for chosen_file in 0..13 {
        process_single_file(chosen_file);
    }
    let duration = start.elapsed();
    println!();
    println!("Time to process all files: {:?}", duration);
}
