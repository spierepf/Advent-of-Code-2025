use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("failed to read stdin");
    println!(
        "{}",
        input
            .parse::<printing_department::CellSet>()
            .unwrap()
            .count_accessible_rolls()
    );
}
