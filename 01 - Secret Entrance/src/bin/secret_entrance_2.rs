fn main() {
    println!(
        "{}",
        secret_entrance::calculate_password_2(&mut std::io::stdin().lock())
    );
}
