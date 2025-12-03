fn main() {
    println!(
        "{}",
        gift_shop::sum_all_invalid_product_ids_from_input::<gift_shop::LevelTwoProductIdValidator>(
            &mut std::io::stdin().lock()
        )
    );
}
