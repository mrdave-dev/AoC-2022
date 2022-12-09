use std::{fs, collections::{HashSet, HashMap}};
use core::fmt;

fn get_file_content() -> String {
    return fs::read_to_string("./input.txt")
        .expect("File './input.txt' should have been readable");
}

// Parse out the directory and files structure from input and output
// input: anything starting with $
// output: directories, files
// need to keep track of size 

// use a dict to keep track of every directory key, size

// cd and ls are the only commands


struct File {
    path: String,
    name: String,
    size: u64,
}

struct Dir {
    /// The key of the file
    path: String,
    /// Keys of sub-Dirs and Files 
    children: Vec<String>,
    /// Key of container
    parent: Option<String>,
    /// Size of all Files and sub-Dirs
    size: u64,
}

impl fmt::Debug for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        return write!(f, "Dir '{}', size {}", self.path, self.size);
    }
}

impl fmt::Debug for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        return write!(f, "File '{}', size {}", self.path, self.size);
    }
}

fn create_dir(path: &str, parent: Option<String>) -> Dir {
    return Dir {
        path: String::from(path),
        children: vec![],
        parent: parent,
        size: 0,
    }
}


fn main() {
    let dollar_char = String::from("$");
    let input = get_file_content();
    let lines: Vec<&str> = input.split("\n").collect();
    let root_dir = create_dir("/", None);
    let mut current_path = vec!["/"];
    let mut is_directory_output: bool = false;
    let mut dirsByPath: HashMap<String, Dir> = HashMap::new();
    let mut filesByPath: HashMap<String, File> = HashMap::new();

    dirsByPath.insert("/".to_string(), create_dir("/", None));

    for (n, line) in lines.into_iter().enumerate() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        // println!("Split line: {:?}", split_line);

        if is_directory_output && !split_line[0].eq("$") {
            if split_line[0].eq("dir") {
                if dirsByPath.get(split_line[1]).is_none() {
                    let parent_path = current_path.join("/");
                    let path = vec![parent_path.clone(), split_line[1].to_string()].join("/");
                    let new_dir = create_dir(&path, Some(parent_path.to_string()));
                    // println!("Creating new path 1: {:?}", &new_dir);
                    dirsByPath.insert(new_dir.path.clone(), new_dir);
                }
            } else {
                if filesByPath.get(split_line[1]).is_none() {
                    let size: u64 = split_line[0].parse().unwrap();
                    let name = split_line[1];
                    // println!("File '{}', size: {}", name, size);
                    // Add File struct to hashmap
                    let parent_path = current_path.join("/");
                    let path = vec![parent_path.clone(), split_line[1].to_string()].join("/");
                    // println!("Creating new file {}", path);
                    let new_file = File {
                        size,
                        path,
                        name: name.to_string(),
                    };
                    filesByPath.insert(new_file.path.clone(), new_file);
                    
                    // Iterate over parents, adding to each size along the way
                    let mut parent = current_path.clone();
                    while parent.len() > 0 {
                        // println!("parent len {} / {}", parent.len(), current_path.len());
                        let parent_string = parent.join("/");
                        // println!("dirs: {:?}", dirsByPath);
                        // println!("parent string: {}", parent_string);
                        // something wrong here
                        let parent_dir = dirsByPath.get_mut(parent_string.as_str()).unwrap();
                        // println!("Adding {} to dir '{:?}'", parent_string, parent_dir);
                        parent_dir.size += size;
                        parent.pop();
                    }
                    
                    

                    


                }

            }


        }

        if split_line[0].eq("$") {
            is_directory_output = false;
            // line is a command
            assert!(split_line.len() > 1, "Expected split line length to be 2 or greater");

            let cmd = split_line[1];
            let param = if split_line.len() > 2 { split_line[2] } else { "" };
            // println!("cmd = '{}', param = '{}'", cmd, param);
            
            if cmd.eq("cd") {
                if param.eq("..") {
                    // println!("poping");
                    current_path.pop();
                } else if param.eq("/") {
                    // println!("clearing");
                    current_path.clear();
                    current_path.push("/");
                } else {
                    let next_path = vec![current_path.clone().join("/"), split_line[2].to_string()].join("/");
                    let parent_path = current_path.clone().join("/");
                    if dirsByPath.get(next_path.as_str()).is_none() {
                        // println!("parent_path {}", parent_path);
                        let current_dir = dirsByPath.get_mut(parent_path.as_str()).unwrap();
                        let mut new_dir = create_dir(&next_path.clone(), Some(parent_path.to_string()));
                        new_dir.size = current_dir.size;
                        // println!("Creating new path 2: {:?}", &new_dir);
                        // println!("Next path {}", next_path);
                        let path = new_dir.path.clone();
                        dirsByPath.insert(path, new_dir);
                    }
                    
                    // println!("pushing {}", param);
                    current_path.push(param);
                }
                // println!("New current dir: {}", current_path.join("/"));
            } else if cmd.eq("ls") {
                is_directory_output = true;
            } else {
                panic!("Command '{}' not recognized", cmd);
            }



        } else {
            // line is output
        }
    }
    println!("Root: {:#?}", root_dir);
    println!("Directories: {:#?}", dirsByPath);
    // println!("Files: {:#?}", filesByPath);

    let mut size_sum: u64 = 0;
    // let mut dirs_in_sum: Vec<Dir> = vec![];
    let root_cap = 70_000_000;
    let space_needed = 30_000_000;
    
    // for (path, dir) in dirsByPath {
    //     if (dir.size <= 100_000) {
    //         size_sum += dir.size;
    //         dirs_in_sum.push(dir);
    //     }
    // }
    // println!("Dirs in sum: {:?}", dirs_in_sum);
    // println!("Size: {}", size_sum);
    
    // for (path, dir) in &dirsByPath {
    //     size_sum += dir.size;
    // }
    let root_size = dirsByPath.get("/").unwrap().size;
    
    let space_ava: i64 = root_cap as i64 - root_size as i64;
    let diff: i64 = space_needed - space_ava;
    println!("{}, {}, {} Diff: {}", root_size, root_cap, space_ava, diff);

    let mut closest_diff = root_cap;
    
    for (path, dir) in &dirsByPath {
        if (dir.size < closest_diff) && (dir.size as i64 >= diff) {
            println!("new closest: {:?}", dir);
            closest_diff = dir.size;
        }
    }

    println!("/ capacity: {}", root_cap);
    println!("needed:     {}", space_needed);
    println!("/ size:     {}", root_size);
    println!("/ space:    {}", space_ava);
    println!("needed:     {}", diff);
    println!("will free:  {}", closest_diff);
    
    
    // / cap:        70000000
    // / size:       43956976
    // / space:      26043024
    // needed:        3956976 // too low


}
