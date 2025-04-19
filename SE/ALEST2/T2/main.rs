use std::{
    fs::File,
    path::Path,
    path::PathBuf,
    collections::VecDeque,
    io::{BufRead, BufReader, Lines, Result},
};

const FILES_NUM: usize = 7;
const FILES_PATH: [&str; FILES_NUM] = [
    "data/caso1_cohen.txt",
    "data/caso2_cohen.txt",
    "data/caso3_cohen.txt",
    "data/caso4_cohen.txt",
    "data/caso5_cohen.txt",
    "data/caso6_cohen.txt",
    "data/caso7_cohen.txt",
];
const FILES_NAME: [&str; FILES_NUM] = [
    "caso1_cohen.txt",
    "caso2_cohen.txt",
    "caso3_cohen.txt",
    "caso4_cohen.txt",
    "caso5_cohen.txt",
    "caso6_cohen.txt",
    "caso7_cohen.txt",
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
        if chosen_file < FILES_NUM {
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
    let (mut maze, hero, villain, maze_size) = map_maze(FILES_PATH[chosen_file]);
    let answer = shortest_path(&mut maze, maze_size, hero, villain);
    println!();
    println!(">>> {:?} => {}", FILES_PATH[chosen_file], answer);
}

fn map_maze(file_path: &str) -> (Vec<(char, usize)>, usize, usize, usize) {
    let mut maze = Vec::<(char, usize)>::new();
    let mut hero: usize = 0;
    let mut villain: usize = 0;
    let file_path = Path::new(file_path);
    for (index, character) in read_lines(file_path)
        .unwrap()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .enumerate()
    {
        maze.push((character, 0));
        if character == 'A' {
            hero = index;
        }
        if character == 'B' {
            villain = index;
        }
    }
    let maze_size: usize = ((maze.len() + 1) as f64).sqrt() as usize;
    (maze, hero, villain, maze_size)
}

fn read_lines(filename: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(filename).expect("Error opening file.");
    Ok(BufReader::new(file).lines())
}

fn shortest_path(
    maze: &mut Vec<(char, usize)>,
    offset: usize,
    source: usize,
    destiny: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(source);
    let mut aux: usize = 0;
    while aux != destiny {
        let curr_pos = queue.pop_front().unwrap();
        for next_pos in vec![
            curr_pos - offset,
            curr_pos - 1,
            curr_pos + offset,
            curr_pos + 1,
        ] {
            if maze[next_pos].0 != '#' && maze[next_pos].1 == 0 {
                maze[next_pos].1 = curr_pos + 1;
                queue.push_back(next_pos);
            }
            aux = next_pos;
        }
    }
    maze[destiny].1
}

fn run_all_files() {
    let start = std::time::Instant::now();
    for chosen_file in 0..FILES_NUM {
        process_single_file(chosen_file);
    }
    let duration = start.elapsed();
    println!();
    println!("Time to process all files: {:?}", duration);
}
