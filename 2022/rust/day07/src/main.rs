pub fn part_one(input: &String) -> usize {
    let mut cwd: Vec<String> = vec![String::from("/")];
    let mut dirs: Vec<(String, usize)> = vec![(String::from("/"), 0)];
    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    cwd.pop();
                } else {
                    cwd.push(String::from(parts[2]));
                }
                continue;
            } else if parts[1] == "ls" {
                continue;
            }
        } else if &line[..3] == "dir" {
            let parts: Vec<&str> = line.split(" ").collect();
            let dir_name = String::from(parts[1]);
            let resolved = if cwd.len() == 1 {
                format!("/{dir_name}")
            } else {
                let mut copy = cwd[1..].to_vec();
                copy.push(dir_name);
                let joined = copy.join("/");
                format!("{joined}")
            };
            let dir_info = dirs.iter().find(|(name, _)| name == &resolved);
            if dir_info.is_none() {
                dirs.push((resolved, 0));
            }
        } else /* a file */ {
            let parts: Vec<&str> = line.split(" ").collect();
            let size = parts[0].parse::<usize>().unwrap();
            dirs = dirs.iter().map(|dir_stats| {

                if cwd.join("/").contains(&dir_stats.0) {
                    return (dir_stats.0.clone(), dir_stats.1 + size);
                }
                (dir_stats.0.clone(), dir_stats.1)
            }).collect();
        }
    }
    let result = dirs.iter().fold(0, |total, dir_info| {
        let amount = dir_info.1;
        if amount <= 100000 {
            return total + amount
        }
        total
    });
    result
}
pub fn part_two(input: &String) -> usize {
    let mut cwd: Vec<String> = vec![String::from("/")];
    let mut dirs: Vec<(String, usize)> = vec![(String::from("/"), 0)];
    for line in input.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    cwd.pop();
                } else {
                    cwd.push(String::from(parts[2]));
                }
                continue;
            } else if parts[1] == "ls" {
                continue;
            }
        } else if &line[..3] == "dir" {
            let parts: Vec<&str> = line.split(" ").collect();
            let dir_name = String::from(parts[1]);
            let resolved = if cwd.len() == 1 {
                format!("/{dir_name}")
            } else {
                let mut copy = cwd[1..].to_vec();
                copy.push(dir_name);
                let joined = copy.join("/");
                format!("{joined}")
            };
            let dir_info = dirs.iter().find(|(name, _)| name == &resolved);
            if dir_info.is_none() {
                dirs.push((resolved, 0));
            }
        } else /* a file */ {
            let parts: Vec<&str> = line.split(" ").collect();
            let size = parts[0].parse::<usize>().unwrap();
            dirs = dirs.iter().map(|dir_stats| {

                if cwd.join("/").contains(&dir_stats.0) {
                    return (dir_stats.0.clone(), dir_stats.1 + size);
                }
                (dir_stats.0.clone(), dir_stats.1)
            }).collect();
        }
    }
    let target = 30000000;
    let free = 70000000 - dirs[0].1;
    let min = target - free;
    let amounts: Vec<usize> = dirs.iter().map(|d| d.1).collect();

    *amounts.iter().reduce(|smallest, current| {
        if current > &min {
            if current < smallest {
                return current
            }
        }
        smallest
    }).unwrap()
}

fn main() {
    let contents = std::fs::read_to_string("input").unwrap();
    let result = part_one(&contents);
    println!("part 1");
    println!("{:?}", result);
    let result = part_two(&contents);
    println!("part 2");
    println!("{:?}", result);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = String::from("$ cd /
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
7214296 k");
        assert_eq!(part_one(&example), 95437);
    }
    #[test]
    fn it_works_v2() {
        let example = String::from("$ cd /
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
7214296 k");
        assert_eq!(part_two(&example), 24933642);
    }
}