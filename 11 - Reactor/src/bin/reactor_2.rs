use reactor::{Graph, Node};
use toolbox::read_stdin_to_string;

fn main() {
    let graph = read_stdin_to_string().parse::<Graph>().unwrap();

    println!(
        "{}",
        graph.count_paths_including(vec![
            Node("svr".to_string()),
            Node("fft".to_string()),
            Node("dac".to_string()),
            Node("out".to_string()),
        ]) + graph.count_paths_including(vec![
            Node("svr".to_string()),
            Node("dac".to_string()),
            Node("fft".to_string()),
            Node("out".to_string()),
        ])
    );
}
