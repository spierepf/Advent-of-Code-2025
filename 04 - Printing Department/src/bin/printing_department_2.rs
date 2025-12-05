use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("failed to read stdin");
    let rolls: printing_department::CellSet = input.parse().expect("failed to parse input");
    println!(
        "{}",
        printing_department::subtract_rolls_until_complete(rolls)
    );
}
