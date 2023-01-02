use std::fs;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const COMMAND_SYMBOL: &str = "$";

#[derive(Debug)]
struct File {
    filename: String,
    length: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    // List of references to directories
    directories: Vec<usize>,
    // List of references to files
    files: Vec<usize>,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let mut directories = vec![Directory {
        name: String::from(""),
        directories: Vec::new(),
        files: Vec::new()
    }];
    let mut files: Vec<File> = Vec::new();

    let mut current_directory_index: usize = 0;

    let mut iterator = input.lines().peekable();
    while iterator.peek().is_some() {
        let line_input = iterator.next().unwrap();

        if line_input.starts_with(COMMAND_SYMBOL) {
            let line_input = line_input.replace(COMMAND_SYMBOL, " ");
            let line_input = line_input.trim();

            
            if line_input.starts_with("cd") {
                let path = line_input.replace("cd", "");
                let path = path.trim();

                if path == "/" {
                    current_directory_index = 0;
                }
                else if path == ".." {
                    for (i, dir) in directories.iter().enumerate() {
                        if dir.directories.contains(&current_directory_index) {
                            current_directory_index = i;
                        }
                    }
                }
                else {
                    let mut dir_exists = false;
                    for dir_index in &directories[current_directory_index].directories {
                        if directories[*dir_index].name == path {
                            current_directory_index = *dir_index;
                            dir_exists = true;
                            break;
                        }
                    }
    
                    if !dir_exists {
                        let dir = Directory {
                            name: String::from(path),
                            directories: Vec::new(),
                            files: Vec::new()
                        };
                        directories.push(dir);
                        let new_dir_index = directories.len() - 1;

                        directories[current_directory_index].directories.push(new_dir_index);
                        current_directory_index = new_dir_index;
                    }
                }
            }
        } else if line_input.split(" ").nth(0).unwrap().parse::<u32>().is_ok() {
            let length = line_input.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            let filename = line_input.split(" ").nth(1).unwrap();

            let file = File {
                filename: String::from(filename),
                length
            };

            files.push(file);
            directories[current_directory_index].files.push(files.len() - 1);
        }
    }

    println!("Files: ");
    for file in &files {
        println!("{:?}", file);
    }

    println!("Directories: ");
    for dir in &directories {
        println!("{:?}", dir);
    }

    let mut directory_sizes = vec![0; directories.len()];
    for (i, dir) in directories.iter().enumerate().rev() {
        for child_dir in &dir.directories {
            directory_sizes[i] += directory_sizes[*child_dir];
        }        

        for file_index in &dir.files {
            directory_sizes[i] += files[*file_index].length;
        }
    }


    println!("Directories less than 100 000:");

    let mut combined_small_dir_size: usize = 0;
    for (i, small_dir) in directory_sizes.iter().enumerate() {
        if *small_dir < 100000 {
            println!("{:?}", directories[i]);
            combined_small_dir_size += *small_dir;
        }
    }
    println!("{}", combined_small_dir_size);

    const TOTAL_SYSTEM_SIZE: usize  = 70000000;
    const SPACE_NEEDED_FOR_UPDATE: usize  = 30000000;
    let used_system_space: usize = directory_sizes[0];

    let needed_space: usize = SPACE_NEEDED_FOR_UPDATE - (TOTAL_SYSTEM_SIZE - used_system_space);

    println!("Total directories size is: {}", directory_sizes[0]);

    let mut directory_tuple = Vec::new();
    for (i, dir) in directories.iter().enumerate() {
        directory_tuple.push((i, dir.name.clone(), directory_sizes[i]));
    }

    directory_tuple.sort_by(|a, b| a.2.cmp(&b.2));

    let dir_to_delete = directory_tuple.iter().find(|&t| t.2 >= needed_space);

    match dir_to_delete {
        Some(d) => println!("Directory to delete: {}, size {}", d.1, d.2),
        None => println!("Directory not found")
    };

    return Ok(());
}
