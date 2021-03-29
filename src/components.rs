use anyhow::{Error, Result};
use num::Num;
use std::{
    cmp,
    collections::HashSet,
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    ops,
    str::FromStr,
    usize,
};

/// FactoryID provides three methods.
/// Users only need to implement `prefix`
/// which is a `&'static str` and unique to the type.
/// `from_str` generates an instance's id,
/// `from_numeric` generates an instance's name.
/// Both are automatically implemented provided that `prefix` is implemented.
pub trait FactoryID {
    /// The prefix a type has in input file.
    /// The name of an instance is uniquely determined by its prefix and id
    fn prefix() -> &'static str;

    /// Converts from &str to usize.
    fn from_str(name: &str) -> Result<usize>
    where
        Error: From<<usize as FromStr>::Err>,
    {
        // subtracted by one because of the offset
        let length = Self::prefix().len();

        let parsednum = name[length..].parse::<usize>().map_err(Error::from)?;
        let id = parsednum - 1;
        Ok(id)
    }

    /// Converts from usize to String.
    fn from_num(id: usize) -> Result<String> {
        // added by one because of the offset
        let strnum = id + 1;
        Ok(format!("{}{}", Self::prefix(), strnum))
    }
}

/// Directions of a layer
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// There are different conflict types
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConflictType {
    AdjHGGrid,
    SameGGrid,
}

/// Whether a cell is movable
#[derive(Debug)]
pub enum CellType {
    Movable,
    Fixed,
}

/// Towards a direction
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Towards {
    Up,
    Down,
    Left,
    Right,
    Top,
    Bottom,
}

/// A 2-dimension tuple representing a Pair.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Pair<T>(pub T, pub T)
where
    T: Copy + Num;

/// A 3-dimension tuple representing a Point.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point<T>(pub T, pub T, pub T)
where
    T: Copy + Num;

/// A source point and a target point representing a Route.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Route<T>(pub Point<T>, pub Point<T>)
where
    T: Copy + Num;

/// Some information about a Layer.
#[derive(Debug)]
pub struct Layer {
    /// layer id (starts from 0)
    pub id: usize,
    /// horizontal or vertical
    pub direction: Direction,
    /// dimensions
    pub dim: Pair<usize>,
    /// all grids' capacity
    pub capacity: Vec<usize>,
}

/// Some information about a MasterPin.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct MasterPin {
    /// id of the pin
    pub id: usize,
    /// layer on which the pin is on
    pub layer: usize,
}

/// Some information about a Blockage.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Blockage {
    /// id of the blockage
    pub id: usize,
    /// layer on which the blockage is on
    pub layer: usize,
    /// extra demand the blockage will cost
    pub demand: usize,
}

/// Some information about a MasterCell.
#[derive(Debug)]
pub struct MasterCell {
    /// id of cell
    pub id: usize,
    /// number of pins
    pub pins: HashSet<MasterPin>,
    /// number of blockages
    pub blkgs: HashSet<Blockage>,
}

/// Some information about a Conflict,
/// which happens when certain types of MasterCells are too close for confort.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Conflict {
    /// adjHGGrid or sameGGrid
    pub kind: ConflictType,
    /// other id
    pub id: usize,
    /// on which layer
    pub layer: usize,
    /// by how much
    pub demand: usize,
}

/// Some information about a Cell
#[derive(Debug)]
pub struct Cell {
    /// id of the cell
    pub id: usize,
    /// if the cell can be moved
    pub movable: CellType,
    /// whether the cell has moved
    pub moved: bool,
    /// position
    pub position: Pair<usize>,
    /// mastercell type
    pub pins: Vec<usize>,
}

/// Pointer points to the nearby node.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Pointer {
    /// nearby node index
    index: usize,
    /// nearby node height
    height: usize,
}

/// A node representing a position that's either an endpoint, an intersection, or a turningpoint.
#[derive(Clone, Copy, Debug)]
pub struct PosNode {
    /// corresponding to pin id, None represents a virtual node.
    pub id: Option<usize>,
    /// positions
    pub position: Point<usize>,
    /// -x direction
    pub left: Option<Pointer>,
    /// +x direction
    pub right: Option<Pointer>,
    /// -y direction
    pub down: Option<Pointer>,
    /// +y direction
    pub up: Option<Pointer>,
    /// -z direction
    pub bottom: Option<Pointer>,
    /// +z direction
    pub top: Option<Pointer>,
}

/// Some information about a Net.
#[derive(Debug, Default)]
pub struct Net {
    /// id of the net
    pub id: usize,
    /// min layer id
    pub min_layer: usize,
    // TODO: fields that backs the actual implementation
}

impl<T> Pair<T>
where
    T: Copy + Num,
{
    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }

    pub fn size(&self) -> T {
        // x: rows, y: columns
        self.x() * self.y()
    }

    pub fn with(&self, lay: T) -> Point<T> {
        let &Pair(row, col) = self;
        Point(row, col, lay)
    }
}

impl<T> Point<T>
where
    T: Copy + Num,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    pub fn row(&self) -> T {
        self.0
    }

    pub fn col(&self) -> T {
        self.1
    }

    pub fn lay(&self) -> T {
        self.2
    }

    pub fn flatten(&self) -> Pair<T> {
        let &Point(row, col, _) = self;
        Pair(row, col)
    }
}

impl<T> Route<T>
where
    T: Copy + Num,
{
    pub fn new(a: Point<T>, b: Point<T>) -> Self {
        Self(a, b)
    }

    pub fn raw(ax: T, ay: T, az: T, bx: T, by: T, bz: T) -> Self {
        let pa = Point(ax, ay, az);
        let pb = Point(bx, by, bz);
        Self(pa, pb)
    }

    pub fn source(&self) -> Point<T> {
        self.0
    }

    pub fn target(&self) -> Point<T> {
        self.1
    }
}

impl Layer {
    pub fn get_capacity(&self, row: usize, col: usize) -> Option<&usize> {
        self.capacity.get(row * self.dim.y() + col)
    }

    pub fn get_capacity_mut(&mut self, row: usize, col: usize) -> Option<&mut usize> {
        self.capacity.get_mut(row * self.dim.y() + col)
    }
}

impl FactoryID for Layer {
    fn prefix() -> &'static str {
        "M"
    }
}

impl FactoryID for MasterPin {
    fn prefix() -> &'static str {
        "P"
    }
}

impl FactoryID for Blockage {
    fn prefix() -> &'static str {
        "B"
    }
}

impl FactoryID for MasterCell {
    fn prefix() -> &'static str {
        "MC"
    }
}

impl FactoryID for Cell {
    fn prefix() -> &'static str {
        "C"
    }
}

impl<T> Display for Pair<T>
where
    T: Copy + Display + Num,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {}", self.x(), self.y())
    }
}

impl<T> Display for Point<T>
where
    T: Copy + Display + Num,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {} {}", self.row(), self.col(), self.lay())
    }
}

impl Route<usize> {
    /// Calculates the difference between `source` and `target`
    fn vector(&self) -> Point<isize> {
        let Route(source, target) = self;
        Point(
            target.row() as isize - source.row() as isize,
            target.col() as isize - source.col() as isize,
            target.lay() as isize - source.lay() as isize,
        )
    }

    /// Categorizes the result of `vector`.
    pub fn towards(&self) -> Towards {
        match self.vector() {
            // A vector can only be of (a, 0, 0), (0, b, 0), (0, 0, c) with a, b, c != 0
            // Hence the unreachable arm.
            Point(0, 0, 0) => unreachable!(),
            Point(row, 0, 0) => {
                if row > 0 {
                    Towards::Right
                } else {
                    Towards::Left
                }
            }
            Point(0, col, 0) => {
                if col > 0 {
                    Towards::Up
                } else {
                    Towards::Down
                }
            }
            Point(0, 0, lay) => {
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
        write!(f, "{} {}", self.source(), self.target())
    }
}

impl ops::Neg for Towards {
    type Output = Self;

    /// Get the opposite direction.
    fn neg(self) -> Self::Output {
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

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "CellInst {} {}",
            Self::from_num(self.id).map_err(|_| FmtError)?,
            self.position
        )
    }
}

impl PosNode {
    /// List the neighboring nodes.
    pub fn neightbors(&self) -> [Option<Pointer>; 4] {
        [self.up, self.down, self.left, self.right]
    }

    /// Find the highest and lowest height of a cell.
    pub fn span(&self) -> (usize, usize) {
        self.neightbors()
            .iter()
            .filter_map(|opt| *opt)
            .map(|ptr| ptr.height)
            .fold((usize::MAX, usize::MIN), |(min, max), height| {
                (cmp::min(min, height), cmp::max(max, height))
            })
    }

    /// Get the index of a neighboring node.
    /// As the tree is 2D, `Top` and `Bottom` are not allowed as input.
    pub fn index(&self, towards: Towards) -> Option<Pointer> {
        match towards {
            Towards::Up => self.up,
            Towards::Down => self.down,
            Towards::Left => self.left,
            Towards::Right => self.right,
            Towards::Top | Towards::Bottom => unreachable!(),
        }
    }
}

impl Net {}

impl Display for Net {
    /// Converts `Net` to `String`
    fn fmt(&self, _f: &mut Formatter) -> FmtResult {
        unimplemented!()
    }
}

impl FactoryID for Net {
    fn prefix() -> &'static str {
        "N"
    }
}
