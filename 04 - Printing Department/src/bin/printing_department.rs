use std::io::Read;

fn main() {
    let mut child_output = String::new();
    std::io::stdin().lock().read_to_string(&mut child_output).expect("failed to read stdin");
    println!("{}", child_output.parse::<printing_department::CellSet>().unwrap().count_accessible_rolls());
}
