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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Pair {
    pub x: usize,
    pub y: usize,
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
    pub dim: Pair,
    // all grids' capacity
    pub capacity: Vec<usize>,
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct MasterPin {
    // id of the pin
    pub id: usize,
    // layer on which the pin is on
    pub layer: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
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
    pub position: Pair,
    // mastercell type
    pub pins: Vec<usize>,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
    pub lay: usize,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Route {
    pub source: Point,
    pub target: Point,
}

#[derive(Debug)]
pub struct NetNode {
    // corresponding to pin id
    pub id: Option<usize>,
    // positions
    pub positions: Pair,
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

impl Pair {
    pub fn size(&self) -> usize {
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

impl NetTree {
    pub fn new() -> Self {
        unimplemented!()
    }
}

impl Net {
    pub fn new(min_layer: usize, conn_pins: Vec<usize>, segments: HashSet<Route>) -> Self {
        let tree = NetTree::new();
        Self { min_layer, tree }
    }
}

impl PrefixParse for Net {
    fn prefix() -> &'static str {
        "N"
    }
}
