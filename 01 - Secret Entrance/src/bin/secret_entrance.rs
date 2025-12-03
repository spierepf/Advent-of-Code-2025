fn main() {
    println!(
        "{}",
        secret_entrance::calculate_password(&mut std::io::stdin().lock())
    );
}
