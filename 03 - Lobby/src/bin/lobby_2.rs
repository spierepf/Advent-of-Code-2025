fn main() {
    println!(
        "{}",
        lobby::calculate_total_joltage(&mut std::io::stdin().lock(), 12)
    );
}
