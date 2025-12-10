fn main() {
    let input = toolbox::read_to_string(&mut std::io::stdin().lock()).unwrap();
    let input: trash_compactor::Input = input.parse().unwrap();
    println!("{}", input.do_homework_1());
}
