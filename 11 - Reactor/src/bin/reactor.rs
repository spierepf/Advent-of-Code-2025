use reactor::{Graph, Node};
use toolbox::read_stdin_to_string;

fn main() {
    println!(
        "{}",
        read_stdin_to_string()
            .parse::<Graph>()
            .unwrap()
            .count_paths_from_and_to(Node("you".to_string()), Node("out".to_string()))
    );
}
