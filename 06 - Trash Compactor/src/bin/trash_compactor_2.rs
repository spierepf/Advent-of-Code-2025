fn main() -> anyhow::Result<()> {
    println!(
        "{}",
        trash_compactor::Homework::parse_v2(&toolbox::read_to_string(
            &mut std::io::stdin().lock()
        )?)?
        .sum_of_problems()
    );
    Ok(())
}
