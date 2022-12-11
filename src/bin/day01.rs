fn main() {
    // Read input file
    let input = std::fs::read_to_string("inputs/day01.txt").unwrap();

    let mut elf_list = Vec::new();
    let mut current_sum = 0;
    let lines = input.lines();
    for line in lines {
        if line.is_empty() {
            elf_list.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<i32>().unwrap();
        }
    }
    elf_list.sort();
    elf_list.reverse();

    println!("Solution 01: {}", elf_list[0]);
    println!("Solution 02: {}", elf_list[0] + elf_list[1] + elf_list[2]);
}
