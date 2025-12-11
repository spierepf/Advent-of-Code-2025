use hashbrown::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Debug, Hash, Eq)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
}

impl Cell {
    pub fn adjacent_cells(&self) -> [Cell; 8] {
        [
            Cell {
                row: self.row + -1,
                col: self.col,
            },
            Cell {
                row: self.row + -1,
                col: self.col + 1,
            },
            Cell {
                row: self.row,
                col: self.col + 1,
            },
            Cell {
                row: self.row + 1,
                col: self.col + 1,
            },
            Cell {
                row: self.row + 1,
                col: self.col,
            },
            Cell {
                row: self.row + 1,
                col: self.col + -1,
            },
            Cell {
                row: self.row,
                col: self.col + -1,
            },
            Cell {
                row: self.row + -1,
                col: self.col + -1,
            },
        ]
    }
}
#[test]
fn test_a_cell_can_get_its_adjacent_cells() {
    assert_eq!(Cell { row: 0, col: 0 }.adjacent_cells().len(), 8);
    assert!(
        [
            Cell { row: -1, col: 0 },
            Cell { row: -1, col: 1 },
            Cell { row: 0, col: 1 },
            Cell { row: 1, col: 1 },
            Cell { row: 1, col: 0 },
            Cell { row: 1, col: -1 },
            Cell { row: 0, col: -1 },
            Cell { row: -1, col: -1 },
        ]
        .into_iter()
        .all(|c| Cell { row: 0, col: 0 }.adjacent_cells().contains(&c))
    );

    assert!(
        [
            Cell { row: 9, col: 10 },
            Cell { row: 9, col: 11 },
            Cell { row: 10, col: 11 },
            Cell { row: 11, col: 11 },
            Cell { row: 11, col: 10 },
            Cell { row: 11, col: 9 },
            Cell { row: 10, col: 9 },
            Cell { row: 9, col: 9 },
        ]
        .into_iter()
        .all(|c| Cell { row: 10, col: 10 }.adjacent_cells().contains(&c))
    );

    assert!(
        [
            Cell { row: 2, col: 5 },
            Cell { row: 2, col: 6 },
            Cell { row: 3, col: 6 },
            Cell { row: 4, col: 6 },
            Cell { row: 4, col: 5 },
            Cell { row: 4, col: 4 },
            Cell { row: 3, col: 4 },
            Cell { row: 2, col: 4 },
        ]
        .into_iter()
        .all(|c| Cell { row: 3, col: 5 }.adjacent_cells().contains(&c))
    );
}

#[derive(Clone, Default, PartialEq, Debug)]
pub struct CellSet {
    cells: HashSet<Cell>,
}

impl CellSet {
    pub fn count_occupied_adjacent_cells(&self, cell: Cell) -> usize {
        cell.adjacent_cells()
            .into_iter()
            .collect::<CellSet>()
            .cells
            .intersection(&self.cells)
            .count()
    }

    pub fn count_accessible_rolls(&self) -> usize {
        self.cells
            .iter()
            .filter(|cell| self.count_occupied_adjacent_cells(**cell) < 4)
            .count()
    }
    pub fn accessible_rolls(&self) -> impl Iterator<Item = Cell> {
        self.cells
            .iter()
            .copied()
            .filter(|cell| self.count_occupied_adjacent_cells(*cell) < 4)
    }

    pub fn subtract_rolls(&self, rolls: impl IntoIterator<Item = Cell>) -> CellSet {
        CellSet {
            cells: self
                .cells
                .difference(&rolls.into_iter().collect())
                .copied()
                .collect(),
        }
    }
}

impl FromIterator<Cell> for CellSet {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        let mut set = CellSet::default();

        for c in iter {
            set.cells.insert(c);
        }

        set
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseCellSetError;

impl FromStr for CellSet {
    type Err = ParseCellSetError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(col, c)| match c {
                        '.' => None,
                        '@' => Some(Ok(Cell {
                            row: row as i32,
                            col: col as i32,
                        })),
                        _ => Some(Err(ParseCellSetError)),
                    })
            })
            .collect()
    }
}

impl IntoIterator for CellSet {
    type IntoIter = hashbrown::hash_set::IntoIter<Cell>;
    type Item = Cell;

    fn into_iter(self) -> Self::IntoIter {
        self.cells.into_iter()
    }
}

#[test]
fn test_cell_set_into_iter_returns_an_iterator_containing_the_elements_of_the_vector() {
    assert_eq!(CellSet::from_iter([]).into_iter().count(), 0);
    assert_eq!(
        CellSet::from_iter([Cell { row: 0, col: 0 }])
            .into_iter()
            .count(),
        1
    );
    assert_eq!(
        CellSet::from_iter([Cell { row: 0, col: 0 }, Cell { row: 0, col: 2 }])
            .into_iter()
            .collect::<CellSet>(),
        CellSet::from_iter([Cell { row: 0, col: 0 }, Cell { row: 0, col: 2 }])
    );
}

#[test]
fn test_the_cell_set_can_count_how_many_adjacent_cells_are_occupied() {
    assert_eq!(
        [].into_iter()
            .collect::<CellSet>()
            .count_occupied_adjacent_cells(Cell { row: 0, col: 0 }),
        0
    );
    assert_eq!(
        [Cell { row: -1, col: 0 }]
            .into_iter()
            .collect::<CellSet>()
            .count_occupied_adjacent_cells(Cell { row: 0, col: 0 }),
        1
    );
    assert_eq!(
        [Cell { row: -2, col: 0 }]
            .into_iter()
            .collect::<CellSet>()
            .count_occupied_adjacent_cells(Cell { row: 0, col: 0 }),
        0
    );
}

#[test]
fn test_we_can_translate_text_into_a_cell_set() {
    assert_eq!("".parse::<CellSet>(), Ok(CellSet::default()));
    assert_eq!(".".parse::<CellSet>(), Ok(CellSet::default()));
    assert_eq!(
        "@".parse::<CellSet>(),
        Ok([Cell { row: 0, col: 0 }].into_iter().collect::<CellSet>())
    );
    assert_eq!("X".parse::<CellSet>(), Err(ParseCellSetError));
    assert_eq!(
        ".@".parse::<CellSet>(),
        Ok([Cell { row: 0, col: 1 }].into_iter().collect::<CellSet>())
    );
    assert_eq!(
        "..@@.@@@@.".parse::<CellSet>(),
        Ok([
            Cell { row: 0, col: 2 },
            Cell { row: 0, col: 3 },
            Cell { row: 0, col: 5 },
            Cell { row: 0, col: 6 },
            Cell { row: 0, col: 7 },
            Cell { row: 0, col: 8 },
        ]
        .into_iter()
        .collect::<CellSet>())
    );
    assert_eq!(
        "..@@.@@@@.\n@@@.@.@.@@".parse::<CellSet>(),
        Ok([
            Cell { row: 0, col: 2 },
            Cell { row: 0, col: 3 },
            Cell { row: 0, col: 5 },
            Cell { row: 0, col: 6 },
            Cell { row: 0, col: 7 },
            Cell { row: 0, col: 8 },
            Cell { row: 1, col: 0 },
            Cell { row: 1, col: 1 },
            Cell { row: 1, col: 2 },
            Cell { row: 1, col: 4 },
            Cell { row: 1, col: 6 },
            Cell { row: 1, col: 8 },
            Cell { row: 1, col: 9 },
        ]
        .into_iter()
        .collect::<CellSet>())
    );
}

#[test]
fn test_cell_set_can_count_accessible_rolls() {
    assert_eq!(CellSet::default().count_accessible_rolls(), 0);
    assert_eq!(".".parse::<CellSet>().unwrap().count_accessible_rolls(), 0);
    assert_eq!("@".parse::<CellSet>().unwrap().count_accessible_rolls(), 1);
    assert_eq!(
        "..@@.@@@@."
            .parse::<CellSet>()
            .unwrap()
            .count_accessible_rolls(),
        6
    );
    assert_eq!( "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".parse::<CellSet>().unwrap().count_accessible_rolls() , 13);
}

#[test]
fn test_cell_set_can_iterate_over_accessible_cells() {
    assert_eq!(
        CellSet::default().accessible_rolls().collect::<CellSet>(),
        CellSet::default()
    );
    assert_eq!(
        ".".parse::<CellSet>().unwrap().accessible_rolls().count(),
        0
    );
    assert_eq!(
        "@".parse::<CellSet>().unwrap().accessible_rolls().count(),
        1
    );
    assert_eq!(
        "@".parse::<CellSet>()
            .unwrap()
            .accessible_rolls()
            .collect::<CellSet>(),
        CellSet::from_iter([Cell { row: 0, col: 0 }])
    );
    assert_eq!(
        "..@@.@@@@."
            .parse::<CellSet>()
            .unwrap()
            .accessible_rolls()
            .count(),
        6
    );
    assert_eq!(
        "..@@.@@@@."
            .parse::<CellSet>()
            .unwrap()
            .accessible_rolls()
            .collect::<CellSet>(),
        CellSet::from_iter([
            Cell { row: 0, col: 2 },
            Cell { row: 0, col: 3 },
            Cell { row: 0, col: 5 },
            Cell { row: 0, col: 6 },
            Cell { row: 0, col: 7 },
            Cell { row: 0, col: 8 },
        ])
    );
    assert_eq!( "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".parse::<CellSet>().unwrap().accessible_rolls().count(), 13);
}

#[test]
fn test_cell_set_can_subtract_rolls() {
    assert_eq!(
        CellSet::default().subtract_rolls(CellSet::default()),
        CellSet::default()
    );
    assert_eq!(
        "@".parse::<CellSet>()
            .unwrap()
            .subtract_rolls(CellSet::default()),
        "@".parse::<CellSet>().unwrap()
    );
    assert_eq!(
        "@".parse::<CellSet>()
            .unwrap()
            .subtract_rolls("@".parse::<CellSet>().unwrap()),
        CellSet::default()
    );
    assert_eq!(
        "@.".parse::<CellSet>()
            .unwrap()
            .subtract_rolls(".@".parse::<CellSet>().unwrap()),
        "@.".parse::<CellSet>().unwrap()
    );
}

pub fn subtract_rolls_until_complete(mut rolls: CellSet) -> usize {
    let mut total_removed: usize = 0;

    loop {
        let accessible_rolls = rolls.accessible_rolls().collect::<CellSet>();

        let count_before = rolls.cells.len();
        rolls = rolls.subtract_rolls(accessible_rolls);
        let count_after = rolls.cells.len();

        let removed_count = count_before.abs_diff(count_after);
        if removed_count == 0 {
            break;
        }
        total_removed += removed_count;
    }

    total_removed
}

#[test]
fn test_subtract_rolls_until_complete() {
    assert_eq!(subtract_rolls_until_complete("..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.".parse::<CellSet>().unwrap()), 43);
    assert_eq!(
        subtract_rolls_until_complete("..@@.@@@@.".parse::<CellSet>().unwrap()),
        6
    );
}
