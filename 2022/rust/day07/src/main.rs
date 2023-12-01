#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    parent: Option<usize>,
    children: Option<Vec<usize>>
}

#[derive(Debug)]
struct Filesystem {
    entries: Vec<Node>
}

fn parse_fs(input: &String) -> Result<Filesystem, String> {
    let mut cursor: usize = 0;
    let mut fs = Filesystem{entries: vec![]};
    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    match fs.entries[cursor].parent {
                        Some(parent) => cursor = parent,
                        None => return Err(String::from("trying to go up to nowhere")),
                    };
                } else if fs.entries.len() == 0 {
                    fs.entries.push(Node{name: String::from("/"), size: 0, parent: None, children: Some(vec![])});
                    // cursor can stay 0
                    // maybe more correct for cursor to be None before system boots up though
                } else {
                    // look for the filename in children
                    let name = String::from(parts[2]);
                    let child_cursor: usize = match &fs.entries[cursor].children {
                        Some(children) => match children.iter().find(|&index| fs.entries[*index].name == name) {
                            Some(entry) => *entry,
                            None => return Err(format!("fs entry not found for subfolder {name}"))
                        },
                        None => {
                            let new_index = fs.entries.len();
                            fs.entries.push(Node{ name, size: 0, parent: Some(cursor), children: None });
                            new_index
                        }
                    };
                    cursor = child_cursor;
                }
                continue;
            } else if parts[1] == "ls" {
                continue;
            }
        } else if &line[..3] == "dir" {
            let dir_name = match line.split(" ").skip(1).next() {
                Some(name) => String::from(name),
                None => return Err(String::from("recv'd directory with no name"))
            };
            fs.entries.push(Node{name: dir_name, size: 0, parent: Some(cursor), children: Some(vec![])});
        } else {
            let parts: Vec<&str> = line.split(" ").collect();
            let size = parts[0].parse::<usize>().unwrap();
            let name = String::from(parts[1]);
            let new_index = fs.entries.len();
            fs.entries.push(Node{ name, size, parent: Some(cursor), children: None });
            match &mut fs.entries[cursor].children {
                Some(children) => children.push(new_index),
                None => return Err(String::from("in a directory with no children"))
            }
        }
    }
    return Ok(fs)
}

pub fn part_one(input: &String) -> usize {
    let fs = parse_fs(input);
    // let result = dirs.iter().fold(0, |total, dir_info| {
    //     let amount = dir_info.1;
    //     if amount <= 100000 {
    //         return total + amount;
    //     }
    //     total
    // });
    println!("{:?}", fs);
    0
}
// pub fn part_two(input: &String) -> usize {
//     let dirs = read_dir_stats(input);
//     let target = 30000000;
//     let free = 70000000 - dirs[0].1;
//     let min = target - free;
//     let amounts: Vec<usize> = dirs.iter().map(|d| d.1).collect();

//     *amounts
//         .iter()
//         .reduce(|smallest, current| {
//             if current > &min {
//                 if current < smallest {
//                     return current;
//                 }
//             }
//             smallest
//         })
//         .unwrap()
// }

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
    // let result = part_two(&contents);
    // println!("part 2");
    // println!("{:?}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = String::from(
            "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
        );
        assert_eq!(part_one(&example), 95437);
    }
//     #[test]
//     fn it_works_v2() {
//         let example = String::from(
//             "$ cd /
// $ ls
// dir a
// 14848514 b.txt
// 8504156 c.dat
// dir d
// $ cd a
// $ ls
// dir e
// 29116 f
// 2557 g
// 62596 h.lst
// $ cd e
// $ ls
// 584 i
// $ cd ..
// $ cd ..
// $ cd d
// $ ls
// 4060174 j
// 8033020 d.log
// 5626152 d.ext
// 7214296 k",
//         );
//         assert_eq!(part_two(&example), 24933642);
//     }
}
