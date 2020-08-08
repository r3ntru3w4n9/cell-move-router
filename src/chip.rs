use crate::{
    components::{
        Blockage, Cell, CellType, Conflict, ConflictType, Direction, FactoryID, Layer, MasterCell,
        MasterPin, Net, Pair, Point, Route,
    },
    utilities,
};
use anyhow::Result;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Error as FmtError, Formatter, Result as FmtResult},
    fs,
};

#[derive(Default, Debug)]
pub struct Chip {
    /// maximum movement count
    max_move: usize,
    /// dimensions
    dim: Pair<usize>,
    /// organized layers
    layers: Vec<Layer>,
    /// organized mastercells
    mastercells: Vec<MasterCell>,
    /// all cells
    cells: Vec<Cell>,
    /// all nets
    nets: Vec<Net>,
    /// all conflicts
    conflicts: HashMap<usize, HashSet<Conflict>>,
}

impl Chip {
    /// Reads the content of a file into memory
    pub fn read_file(&mut self, filename: &str) -> Result<()> {
        use utilities::{check_eq, parse_numeric, parse_string};

        let content = fs::read_to_string(filename)?;
        let content = &mut content.split_whitespace();

        // MaxCellMove <maxMoveCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "MaxCellMove")?;
        let max_move: usize = parse_numeric(content)?;
        self.max_move = max_move;

        // GGridBoundaryIdx <rowBeginIdx> <colBeginIdx> <rowEndIdx> <colEndIdx>
        let keyword = parse_string(content)?;
        check_eq(keyword, "GGridBoundaryIdx")?;

        let row_beg: usize = parse_numeric(content)?;
        let col_beg: usize = parse_numeric(content)?;

        check_eq(row_beg, 1)?;
        check_eq(col_beg, 1)?;

        let row_end: usize = parse_numeric(content)?;
        let col_end: usize = parse_numeric(content)?;

        let num_rows = row_end;
        let num_cols = col_end;

        self.dim = Pair(num_rows, num_cols);

        // NumLayer <LayerCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumLayer")?;

        let num_layers: usize = parse_numeric(content)?;

        // Lay <layerName> <Idx> <RoutingDirection> <defaultSupplyOfOneGGrid>
        for idx in 0..num_layers {
            let keyword = parse_string(content)?;
            check_eq(keyword, "Lay")?;

            let name = parse_string(content)?;
            let layer_id: usize = parse_numeric(content)?;
            let id: usize = Layer::from_str(name)?;

            check_eq(layer_id, id + 1)?;

            let dir_str = parse_string(content)?;
            let direction = if dir_str == "H" {
                Direction::Horizontal
            } else {
                check_eq(dir_str, "V")?;
                Direction::Vertical
            };

            let supply: usize = parse_numeric(content)?;
            let grid_size = self.dim.size();
            let capacity = vec![supply; grid_size];
            let dim = self.dim;

            self.layers.push(Layer {
                id: idx,
                direction,
                dim,
                capacity,
            });
        }

        // NumNonDefaultSupplyGGrid <nonDefaultSupplyGGridCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumNonDefaultSupplyGGrid")?;
        let num_non_default: usize = parse_numeric(content)?;
        for _ in 0..num_non_default {
            // <rowIdx> <colIdx> <LayIdx> <incrOrDecrValue>
            let r: usize = parse_numeric(content)?;
            let c: usize = parse_numeric(content)?;
            let l: usize = parse_numeric(content)?;
            let val: isize = parse_numeric(content)?;

            let (r, c, l) = (r - 1, c - 1, l - 1);

            let dim = self.dim;
            let layer_mut = self.get_layer_mut(l).expect("Layer index out of bounds");

            debug_assert_eq!(dim, layer_mut.dim);

            let cell_capacity = layer_mut
                .get_capacity_mut(r, c)
                .expect("Cell index out of bounds");

            *cell_capacity = (*cell_capacity as isize + val) as usize;
        }

        // NumMasterCell <masterCellCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumMasterCell")?;
        let num_master_cell: usize = parse_numeric(content)?;
        // MasterCell <masterCellName> <pinCount> <blockageCount>

        for idx in 0..num_master_cell {
            let keyword = parse_string(content)?;
            check_eq(keyword, "MasterCell")?;

            let name = parse_string(content)?;
            check_eq(MasterCell::from_str(name)?, idx)?;

            let num_pins: usize = parse_numeric(content)?;
            let num_blkgs: usize = parse_numeric(content)?;

            let mut pins = HashSet::with_capacity(num_pins);
            // Pin <pinName> <pinLayer>
            for _ in 0..num_pins {
                let keyword = parse_string(content)?;
                check_eq(keyword, "Pin")?;

                let pin_name = parse_string(content)?;
                let pin_layer = parse_string(content)?;

                let pin_id = MasterPin::from_str(pin_name)?;
                let layer_id = Layer::from_str(pin_layer)?;

                let avail = pins.insert(MasterPin {
                    id: pin_id,
                    layer: layer_id,
                });

                debug_assert!(avail);
            }

            let mut blkgs = HashSet::with_capacity(num_blkgs);

            // Blkg <blockageName> <blockageLayer> <demand>
            for _ in 0..num_blkgs {
                let keyword = parse_string(content)?;
                check_eq(keyword, "Blkg")?;

                let blkg_name = parse_string(content)?;
                let blkg_layer = parse_string(content)?;
                let blkg_demand: usize = parse_numeric(content)?;

                let layer_id = Layer::from_str(blkg_layer)?;
                let blkg_id = Blockage::from_str(blkg_name)?;

                let avail = blkgs.insert(Blockage {
                    id: blkg_id,
                    layer: layer_id,
                    demand: blkg_demand,
                });

                debug_assert!(avail);
            }

            self.mastercells.push(MasterCell {
                id: idx,
                pins,
                blkgs,
            })
        }

        // NumNeighborCellExtraDemand <count>
        let keyword = parse_string(content)?;
        assert_eq!(keyword, "NumNeighborCellExtraDemand");
        let extra_count: usize = parse_numeric(content)?;

        self.conflicts.reserve(2 * extra_count);

        let mut is_same: usize = 0;

        // sameGGrid <masterCellName1> <masterCellName2> <layerName> <demand>
        // adjHGGrid <masterCellName1> <masterCellName2> <layerName> <demand>
        for _ in 0..extra_count {
            let grid_type_str = parse_string(content)?;
            let adj_grid = if grid_type_str == "adjHGGrid" {
                ConflictType::AdjHGGrid
            } else {
                assert_eq!(grid_type_str, "sameGGrid");
                ConflictType::SameGGrid
            };

            let master_cell_1 = parse_string(content)?;
            let master_cell_2 = parse_string(content)?;

            let layer_name = parse_string(content)?;
            let layer_demand: usize = parse_numeric(content)?;

            let mc_id_1 = MasterCell::from_str(master_cell_1)?;
            let mc_id_2 = MasterCell::from_str(master_cell_2)?;

            let layer_id = Layer::from_str(layer_name)?;

            self.conflicts
                .entry(mc_id_1)
                .or_insert_with(HashSet::new)
                .insert(Conflict {
                    kind: adj_grid,
                    id: mc_id_2,
                    layer: layer_id,
                    demand: layer_demand,
                });

            if mc_id_1 == mc_id_2 {
                is_same += 1;
            } else {
                self.conflicts
                    .entry(mc_id_2)
                    .or_insert_with(HashSet::new)
                    .insert(Conflict {
                        kind: adj_grid,
                        id: mc_id_1,
                        layer: layer_id,
                        demand: layer_demand,
                    });
            }
        }

        let num_elements: usize = self
            .conflicts
            .iter()
            .map(|(_, set)| set)
            .map(HashSet::len)
            .sum();

        debug_assert_eq!(num_elements + is_same, 2 * extra_count);

        // NumCellInst <cellInstCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumCellInst")?;
        let cell_count: usize = parse_numeric(content)?;

        let mut pin_cell = Vec::new();

        let mut pin_count = 0;
        // CellInst <instName> <masterCellName> <gGridRowIdx> <gGridColIdx> <movableCstr>
        for idx in 0..cell_count {
            let keyword = parse_string(content)?;
            check_eq(keyword, "CellInst")?;

            let cell_name = parse_string(content)?;
            let id = Cell::from_str(cell_name)?;
            check_eq(id, idx)?;

            let master_cell_name = parse_string(content)?;

            let mc_id = MasterCell::from_str(master_cell_name)?;

            let row: usize = parse_numeric(content)?;
            let col: usize = parse_numeric(content)?;
            let position = Pair(row, col);

            let move_str = parse_string(content)?;
            let movable = if move_str == "Movable" {
                CellType::Movable
            } else {
                check_eq(move_str, "Fixed")?;
                CellType::Fixed
            };

            let mc = self.mastercells.get(mc_id).expect("MasterCell not found");
            let length = mc.pins.len();
            let pins: Vec<_> = (pin_count..pin_count + length).collect();
            pin_count += length;

            pin_cell.push(pin_count);

            self.cells.push(Cell {
                id,
                movable,
                position,
                pins,
            });
        }

        let binary_search = |target: usize, mut low: usize, mut high: usize| -> Option<usize> {
            loop {
                debug_assert!(low <= high);

                let middle = (low + high) / 2;
                let value = *pin_cell.get(middle)?;

                if target <= value {
                    high = middle;
                } else {
                    debug_assert!(target > value);
                    // avoids infinite loop since it's possible that low == middle
                    if low == middle {
                        low += 1;
                    } else {
                        low = middle;
                    }
                }

                if low == high {
                    return Some(high);
                }
            }
        };

        let pin_position = |pin_id: usize| -> Option<Pair<usize>> {
            let cells = &self.cells;
            let idx = binary_search(pin_id, 0, cells.len())?;
            let position = cells.get(idx)?.position;
            Some(position)
        };

        // NumNets <netCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumNets")?;
        let net_count: usize = parse_numeric(content)?;

        let mut net_layers = Vec::with_capacity(net_count);
        let mut net_pins = Vec::with_capacity(net_count);
        // Net <netName> <numPins> <minRoutingLayConstraint>
        for idx in 0..net_count {
            let keyword = parse_string(content)?;
            check_eq(keyword, "Net")?;

            let net_name = parse_string(content)?;
            check_eq(Net::from_str(net_name)?, idx)?;

            let num_pins: usize = parse_numeric(content)?;
            let layer = parse_string(content)?;

            let min_layer = if layer == "NoCstr" {
                0
            } else {
                Layer::from_str(layer)?
            };

            let mut pins = Vec::with_capacity(num_pins);
            // Pin <instName>/<masterPinName>
            for _ in 0..num_pins {
                let keyword = parse_string(content)?;
                check_eq(keyword, "Pin")?;

                let next = parse_string(content)?;
                let pin_info = &mut next.split('/');
                let cell_name = parse_string(pin_info)?;
                let pin_name = parse_string(pin_info)?;
                check_eq(pin_info.next(), None)?;

                let cell_id = Cell::from_str(cell_name)?;
                let pin_id = MasterPin::from_str(pin_name)?;

                pins.push(
                    *self
                        .cells
                        .get(cell_id)
                        .expect("Cell not found")
                        .pins
                        .get(pin_id)
                        .expect("Pin not found"),
                );
            }

            net_layers.push(min_layer);
            net_pins.push(pins);
        }
        // NumRoutes <routeSegmentCount>
        let keyword = parse_string(content)?;
        check_eq(keyword, "NumRoutes")?;
        let num_segments: usize = parse_numeric(content)?;

        let mut routes = vec![HashSet::new(); net_count];

        // <sRowIdx> <sColIdx> <sLayIdx> <eRowIdx> <eColIdx> <eLayIdx> <netName>
        for _ in 0..num_segments {
            let srow: usize = parse_numeric(content)?;
            let scol: usize = parse_numeric(content)?;
            let slay: usize = parse_numeric(content)?;
            let erow: usize = parse_numeric(content)?;
            let ecol: usize = parse_numeric(content)?;
            let elay: usize = parse_numeric(content)?;
            let net_name = parse_string(content)?;
            let net_id = Net::from_str(net_name)?;

            let source = Point(srow, scol, slay);
            let target = Point(erow, ecol, elay);

            let route = Route(source, target);
            routes
                .get_mut(net_id)
                .expect("Index out of bounds")
                .insert(route);
        }

        check_eq(routes.len(), net_count)?;

        self.nets = (0..net_count, net_layers, net_pins, routes)
            .into_par_iter()
            .map(|(id, m_layer, conn_pins, segments)| {
                Net::new(id, m_layer, conn_pins, segments, pin_position)
            })
            .collect();

        // parse ends here
        check_eq(content.next(), None)?;
        Ok(())
    }

    /// Write the content stored in memory to a file
    pub fn write_file(&mut self, filename: &str) -> Result<()> {
        fs::write(filename, format!("{}\n", self))?;

        Ok(())
    }

    /// Returns a reference to a layer
    pub fn get_layer(&self, idx: usize) -> Option<&Layer> {
        self.layers.get(idx)
    }

    /// Returns a mutable reference to a layer
    pub fn get_layer_mut(&mut self, idx: usize) -> Option<&mut Layer> {
        self.layers.get_mut(idx)
    }
}

impl Display for Chip {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let accumulate = |mut acc: String, s: String| {
            acc.push_str(&s);
            acc
        };

        let names: String = self
            .nets
            .par_iter()
            .map(ToString::to_string)
            .fold_with(String::new(), accumulate)
            .reduce_with(accumulate)
            .ok_or(FmtError)?;

        write!(f, "{}", names)
    }
}
