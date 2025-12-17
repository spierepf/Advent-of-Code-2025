use std::collections::HashMap;
use std::str::FromStr;

pub const SAMPLE_INPUT: &str = r#"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"#;

pub const SAMPLE_INPUT2: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
pub struct Node(pub String);

#[derive(Debug, Default, PartialEq)]
pub struct Graph {
    adjacencies: HashMap<Node, Vec<Node>>,
}

impl<const N: usize> From<[(Node, Vec<Node>); N]> for Graph {
    fn from(adjacencies: [(Node, Vec<Node>); N]) -> Self {
        Graph {
            adjacencies: HashMap::from(adjacencies),
        }
    }
}
#[test]
fn test_the_default_graph_has_no_nodes_or_edges() {
    assert_eq!(
        Graph::default(),
        Graph {
            adjacencies: HashMap::new()
        }
    );
}

#[test]
fn test_we_can_construct_a_graph_from_an_array_of_tuples() {
    let empty_array: [(Node, Vec<Node>); 0] = [];
    assert_eq!(Graph::from(empty_array), Graph::default(),);
}

impl Graph {
    fn cached_count_paths_from_and_to(
        &self,
        from: Node,
        to: Node,
        cache: &mut HashMap<Node, usize>,
    ) -> usize {
        if let Some(cached_count) = cache.get(&from) {
            return *cached_count;
        }

        let count = if from == to {
            1
        } else {
            self.adjacencies
                .get(&from)
                .map(|adj| {
                    adj.clone()
                        .into_iter()
                        .map(|i| self.cached_count_paths_from_and_to(i, to.clone(), cache))
                        .sum()
                })
                .unwrap_or_default()
        };

        cache.insert(from, count);
        count
    }

    pub fn count_paths_from_and_to(&self, from: Node, to: Node) -> usize {
        let mut cache: HashMap<Node, usize> = HashMap::new();
        self.cached_count_paths_from_and_to(from, to, &mut cache)
    }
}
#[test]
fn test_there_is_exactly_one_path_from_an_existing_node_to_itself() {
    let node = Node("out".to_string());
    assert_eq!(
        Graph::from([(node.clone(), vec![])]).count_paths_from_and_to(node.clone(), node.clone()),
        1
    )
}

#[test]
fn test_are_no_paths_between_unconnected_nodes() {
    let from = Node("from".to_string());
    let to = Node("to".to_string());
    assert_eq!(
        Graph::from([(from.clone(), vec![]), (to.clone(), vec![]),])
            .count_paths_from_and_to(from.clone(), to.clone()),
        0
    )
}
#[test]
fn test_is_exactly_one_path_in_a_binodal_graph_from_one_node_to_its_directed_neighbor() {
    let from = Node("from".to_string());
    let to = Node("to".to_string());
    assert_eq!(
        Graph::from([(from.clone(), vec![to.clone()]), (to.clone(), vec![]),])
            .count_paths_from_and_to(from.clone(), to.clone()),
        1
    )
}

#[test]
fn test_a_connection_to_an_irrelevant_node_does_not_a_path_make() {
    let from = Node("from".to_string());
    let to = Node("to".to_string());
    let irrelevant = Node("irrelevant".to_string());
    assert_eq!(
        Graph::from([
            (from.clone(), vec![irrelevant.clone()]),
            (to.clone(), vec![]),
            (irrelevant.clone(), vec![]),
        ])
        .count_paths_from_and_to(from.clone(), to.clone()),
        0
    )
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            adjacencies: s
                .lines()
                .map(|line| -> Result<(Node, Vec<Node>), ()> {
                    let (from, rest) = line.split_once(':').ok_or(())?;
                    let to_list = rest
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| Node(s.to_string()))
                        .collect::<Vec<Node>>();
                    Ok((Node(from.to_string()), to_list))
                })
                .collect::<Result<HashMap<Node, Vec<Node>>, ()>>()?,
        })
    }
}

pub fn sample_graph() -> Graph {
    let you = Node("you".to_string());
    let out = Node("out".to_string());

    let aaa = Node("aaa".to_string());
    let bbb = Node("bbb".to_string());
    let ccc = Node("ccc".to_string());
    let ddd = Node("ddd".to_string());
    let eee = Node("eee".to_string());
    let fff = Node("fff".to_string());
    let ggg = Node("ggg".to_string());
    let hhh = Node("hhh".to_string());
    let iii = Node("iii".to_string());

    Graph::from([
        (aaa.clone(), vec![you.clone(), hhh.clone()]),
        (you.clone(), vec![bbb.clone(), ccc.clone()]),
        (bbb.clone(), vec![ddd.clone(), eee.clone()]),
        (ccc.clone(), vec![ddd.clone(), eee.clone(), fff.clone()]),
        (ddd.clone(), vec![ggg.clone()]),
        (eee.clone(), vec![out.clone()]),
        (fff.clone(), vec![out.clone()]),
        (ggg.clone(), vec![out.clone()]),
        (hhh.clone(), vec![ccc.clone(), fff.clone(), iii.clone()]),
        (iii.clone(), vec![out.clone()]),
    ])
}
#[test]
fn parse_a_graph() {
    assert_eq!(SAMPLE_INPUT.parse::<Graph>(), Ok(sample_graph()))
}

impl Graph {
    pub fn count_paths_including(&self, nodes: Vec<Node>) -> usize {
        nodes
            .windows(2)
            .map(|pair| self.count_paths_from_and_to(pair[0].clone(), pair[1].clone()))
            .product()
    }
}

#[test]
fn we_can_count_paths_following_a_list_of_nodes() {
    let you = Node("you".to_string());
    let out = Node("out".to_string());
    let aaa = Node("aaa".to_string());
    let bbb = Node("bbb".to_string());
    let svr = Node("svr".to_string());
    let fft = Node("fft".to_string());
    let dac = Node("dac".to_string());

    assert_eq!(
        sample_graph().count_paths_including(vec![you.clone(), out.clone()]),
        5
    );

    assert_eq!(
        Graph::from([(you.clone(), vec![out.clone()])])
            .count_paths_including(vec![you.clone(), out.clone()]),
        1
    );

    assert_eq!(
        Graph::from([
            (you.clone(), vec![aaa.clone(), bbb.clone()]),
            (aaa.clone(), vec![out.clone()]),
            (bbb.clone(), vec![out.clone()]),
        ])
        .count_paths_including(vec![you.clone(), aaa.clone(), out.clone()]),
        1
    );

    assert_eq!(
        SAMPLE_INPUT2
            .parse::<Graph>()
            .unwrap()
            .count_paths_including(vec![svr.clone(), fft.clone(), dac.clone(), out.clone()]),
        2
    );
    assert_eq!(
        SAMPLE_INPUT2
            .parse::<Graph>()
            .unwrap()
            .count_paths_including(vec![svr.clone(), dac.clone(), fft.clone(), out.clone()]),
        0
    );
}
