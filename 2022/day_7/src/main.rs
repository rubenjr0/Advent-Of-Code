mod computer;
mod filesystem;

use computer::Computer;

fn main() {
    let input = include_str!("../input.txt");
    /*  let test = "$ cd /
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
    7214296 k"; */

    let mut computer = Computer::new();
    input
        .lines()
        .filter(|l| l != &"$ ls")
        .map(|line| line.replace("$ ", ""))
        .for_each(|i| {
            computer.interpret_entry(&i);
        });

    // dbg!(&computer);
    let solution = computer.small_directories_size();
    assert_eq!(solution, 1118405);
    println!("Small files in the filesystem occupy {} bytes", solution);

    println!("Available space: {} bytes", computer.free_space());
}
