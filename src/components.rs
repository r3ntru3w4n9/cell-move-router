use arrayvec::ArrayVec;
use num::Num;
use std::collections::{HashMap, HashSet};

pub trait PrefixParse {
    fn prefix() -> &'static str;
    fn to_numeric(name: &str) -> usize {
        // subtracted by one because of the offset
        let length = Self::prefix().len();
        name[length..].parse::<usize>().expect("Expect usize") - 1
    }
    fn to_string(id: usize) -> String {
        // added by one because of the offset
        format!("{}{}", Self::prefix(), id + 1)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Pair<T>
where
    T: Num,
{
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
pub struct Layer {
    // layer id (starts from 0)
    pub id: usize,
    // horizontal or vertical
    pub direction: Direction,
    // dimensions
    pub dim: Pair<usize>,
    // all grids' capacity
    pub capacity: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MasterPin {
    // id of the pin
    pub id: usize,
    // layer on which the pin is on
    pub layer: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Blockage {
    // id of the blockage
    pub id: usize,
    // layer on which the blockage is on
    pub layer: usize,
    // extra demand the blockage will cost
    pub demand: usize,
}

#[derive(Debug)]
pub struct MasterCell {
    // id of cell
    pub id: usize,
    // number of pins
    pub pins: HashSet<MasterPin>,
    // number of blockages
    pub blkgs: HashSet<Blockage>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConflictType {
    AdjHGGrid,
    SameGGrid,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Conflict {
    // adjHGGrid or sameGGrid
    pub kind: ConflictType,
    // other id
    pub id: usize,
    // on which layer
    pub layer: usize,
    // by how much
    pub demand: usize,
}

#[derive(Debug)]
pub enum CellType {
    Movable,
    Fixed,
}

#[derive(Debug)]
pub struct Cell {
    // if the cell can be moved
    pub movable: CellType,
    // position
    pub position: Pair<usize>,
    // mastercell type
    pub pins: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point<T>
where
    T: Num,
{
    pub row: T,
    pub col: T,
    pub lay: T,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Route<T>
where
    T: Num,
{
    pub source: Point<T>,
    pub target: Point<T>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Towards {
    Up,
    Down,
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug)]
pub struct NetNode {
    // corresponding to pin id
    pub id: Option<usize>,
    // positions
    pub position: Pair<usize>,
    // nearby nodes
    pub up: Option<usize>,
    pub down: Option<usize>,
    pub left: Option<usize>,
    pub right: Option<usize>,
}

#[derive(Debug)]
pub struct NetTree {
    nodes: Vec<NetNode>,
}

#[derive(Debug)]
pub struct Net {
    // min layer id
    pub min_layer: usize,
    // represented as a tree
    pub tree: NetTree,
}

impl<T> Pair<T>
where
    T: Copy + Num,
{
    pub fn size(&self) -> T {
        // x: rows, y: columns
        self.x * self.y
    }
}

impl Layer {
    pub fn get_capacity(&self, row: usize, col: usize) -> Option<&usize> {
        self.capacity.get(row * self.dim.y + col)
    }

    pub fn get_capacity_mut(&mut self, row: usize, col: usize) -> Option<&mut usize> {
        self.capacity.get_mut(row * self.dim.y + col)
    }
}

impl PrefixParse for Layer {
    fn prefix() -> &'static str {
        "M"
    }
}

impl PrefixParse for MasterPin {
    fn prefix() -> &'static str {
        "P"
    }
}

impl PrefixParse for Blockage {
    fn prefix() -> &'static str {
        "B"
    }
}

impl PrefixParse for MasterCell {
    fn prefix() -> &'static str {
        "MC"
    }
}

impl PrefixParse for Cell {
    fn prefix() -> &'static str {
        "C"
    }
}

impl<T> Point<T>
where
    T: Num,
{
    pub fn flatten(self) -> Pair<T> {
        let Point { row, col, .. } = self;
        Pair { x: row, y: col }
    }
}

impl Route<usize> {
    pub fn vector(&self) -> Point<isize> {
        let Route { source, target } = *self;
        Point {
            row: target.row as isize - source.row as isize,
            col: target.col as isize - source.col as isize,
            lay: target.lay as isize - source.lay as isize,
        }
    }

    pub fn towards(&self) -> Towards {
        match self.vector() {
            Point {
                row: 0,
                col: 0,
                lay: 0,
            } => unreachable!(),
            Point {
                row,
                col: 0,
                lay: 0,
            } => {
                if row > 0 {
                    Towards::Up
                } else {
                    Towards::Down
                }
            }
            Point {
                row: 0,
                col,
                lay: 0,
            } => {
                if col > 0 {
                    Towards::Right
                } else {
                    Towards::Left
                }
            }
            Point {
                row: 0,
                col: 0,
                lay,
            } => {
                if lay > 0 {
                    Towards::Top
                } else {
                    Towards::Bottom
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Towards {
    pub fn inv(&self) -> Self {
        match *self {
            Towards::Up => Towards::Down,
            Towards::Down => Towards::Up,
            Towards::Left => Towards::Right,
            Towards::Right => Towards::Left,
            Towards::Top => Towards::Bottom,
            Towards::Bottom => Towards::Top,
        }
    }
}

impl NetTree {
    pub fn new<F>(conn_pins: Vec<usize>, segments: HashSet<Route<usize>>, pin_position: F) -> Self
    where
        F: Fn(usize) -> Pair<usize>,
    {
        let conn_positions: HashMap<Pair<usize>, usize> = conn_pins
            .into_iter()
            .map(pin_position)
            .enumerate()
            .map(|(x, y)| (y, x))
            .collect();

        let end_points: HashSet<Pair<usize>> = segments
            .iter()
            .map(|route| [route.source, route.target])
            .map(ArrayVec::from)
            .map(ArrayVec::into_iter)
            .flatten()
            .map(Point::flatten)
            .collect();

        let mut nodes: Vec<NetNode> = end_points
            .into_iter()
            .map(|position| {
                let id = if let Some(&idx) = conn_positions.get(&position) {
                    Some(idx)
                } else {
                    None
                };

                NetNode {
                    id,
                    position,
                    up: None,
                    down: None,
                    left: None,
                    right: None,
                }
            })
            .collect();

        let position_to_idx: HashMap<Pair<usize>, usize> = nodes
            .iter()
            .enumerate()
            .map(|(idx, node)| (node.position, idx))
            .collect();

        for route in segments.into_iter() {
            let towards = route.towards();

            let Route { source, target } = route;

            let source_pos = source.flatten();
            let target_pos = target.flatten();

            let source_idx = *position_to_idx
                .get(&source_pos)
                .expect("Node does not exist");
            let target_idx = *position_to_idx
                .get(&target_pos)
                .expect("Node does not exist");

            let mut set_node = |self_index: usize, other_index, diff: Towards| {
                let node = nodes.get_mut(self_index).expect("Node does not exist");

                match diff {
                    Towards::Up => {
                        node.up = Some(other_index);
                    }
                    Towards::Down => {
                        node.down = Some(other_index);
                    }
                    Towards::Left => {
                        node.left = Some(other_index);
                    }
                    Towards::Right => {
                        node.right = Some(other_index);
                    }
                    Towards::Top | Towards::Bottom => {}
                }
            };

            set_node(source_idx, target_idx, towards);
            set_node(target_idx, source_idx, towards.inv());
        }

        Self { nodes }
    }
}

impl Net {
    pub fn new<F>(
        min_layer: usize,
        conn_pins: Vec<usize>,
        segments: HashSet<Route<usize>>,
        pin_position: F,
    ) -> Self
    where
        F: Fn(usize) -> Pair<usize>,
    {
        let tree = NetTree::new(conn_pins, segments, pin_position);
        Self { min_layer, tree }
    }
}

impl PrefixParse for Net {
    fn prefix() -> &'static str {
        "N"
    }
}
