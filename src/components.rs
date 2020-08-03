use arrayvec::ArrayVec;
use num::Num;
use std::{
    cmp,
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
    usize,
};

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pointer {
    // nearby node index
    index: usize,
    // nearby node height
    height: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct NetNode {
    // corresponding to pin id
    pub id: Option<usize>,
    // positions
    pub position: Pair<usize>,
    // nearby nodes
    pub up: Option<Pointer>,
    pub down: Option<Pointer>,
    pub left: Option<Pointer>,
    pub right: Option<Pointer>,
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
    pub fn size(self) -> T {
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

impl<T> Display for Point<T>
where
    T: Copy + Display + Num,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {} {}", self.row, self.col, self.lay)
    }
}

impl Route<usize> {
    pub fn vector(self) -> Point<isize> {
        let Route { source, target } = self;
        Point {
            row: target.row as isize - source.row as isize,
            col: target.col as isize - source.col as isize,
            lay: target.lay as isize - source.lay as isize,
        }
    }

    pub fn towards(self) -> Towards {
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

impl<T> Display for Route<T>
where
    T: Copy + Display + Num,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {}", self.source, self.target)
    }
}

impl Towards {
    pub fn inv(self) -> Self {
        match self {
            Towards::Up => Towards::Down,
            Towards::Down => Towards::Up,
            Towards::Left => Towards::Right,
            Towards::Right => Towards::Left,
            Towards::Top => Towards::Bottom,
            Towards::Bottom => Towards::Top,
        }
    }
}

impl NetNode {
    pub fn neightbors(self) -> [Option<Pointer>; 4] {
        [self.up, self.down, self.left, self.right]
    }

    pub fn span(self) -> Pair<usize> {
        self.neightbors()
            .iter()
            .filter_map(|opt| *opt)
            .map(|ptr| ptr.height)
            .fold(
                Pair {
                    x: usize::MAX,
                    y: usize::MIN,
                },
                |Pair { x: min, y: max }, height| Pair {
                    x: cmp::min(min, height),
                    y: cmp::max(max, height),
                },
            )
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
            .map(|&Route { source, target }| [source, target])
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
            let height = source.lay;
            match towards {
                Towards::Up | Towards::Down | Towards::Left | Towards::Right => {
                    debug_assert_eq!(height, target.lay);
                }
                Towards::Top | Towards::Bottom => {
                    debug_assert_ne!(height, target.lay);
                }
            }

            let source_pos = source.flatten();
            let target_pos = target.flatten();

            let source_idx = *position_to_idx
                .get(&source_pos)
                .expect("Node does not exist");
            let target_idx = *position_to_idx
                .get(&target_pos)
                .expect("Node does not exist");

            let mut set_node = |sindex: usize, oindex: usize, height: usize, diff: Towards| {
                let node = nodes.get_mut(sindex).expect("Node does not exist");
                let some_ptr = Some(Pointer {
                    index: oindex,
                    height,
                });

                match diff {
                    Towards::Up => node.up = some_ptr,
                    Towards::Down => node.down = some_ptr,
                    Towards::Left => node.left = some_ptr,
                    Towards::Right => node.right = some_ptr,
                    Towards::Top | Towards::Bottom => debug_assert_eq!(sindex, oindex),
                }
            };

            let mut set_both = |sindex: usize, oindex: usize, height: usize, diff: Towards| {
                set_node(sindex, oindex, height, diff);
                set_node(oindex, sindex, height, diff.inv());
            };

            set_both(source_idx, target_idx, height, towards);
        }

        Self { nodes }
    }
}

impl Display for NetTree {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        unimplemented!()
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

impl ToString for Net {
    fn to_string(&self) -> String {
        self.tree.to_string()
    }
}

impl PrefixParse for Net {
    fn prefix() -> &'static str {
        "N"
    }
}
