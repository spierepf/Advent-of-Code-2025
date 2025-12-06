fn main() {
    println!(
        "{}",
        toolbox::read_to_string(&mut std::io::stdin().lock())
            .unwrap()
            .parse::<cafeteria::Input>()
            .unwrap()
            .count_total_fresh_product_ids()
    );
}
