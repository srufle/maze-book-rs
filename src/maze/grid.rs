extern crate image;
extern crate imageproc;
extern crate slog_async;
extern crate slog_term;

use super::cell::Cell;
use super::cell::Cells;
use radix_fmt::*;
use slog::Drain;
use slog::Logger;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use self::image::{Rgb, RgbImage};
use self::imageproc::drawing;
use self::imageproc::rect::Rect;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Hash)]
pub struct Pos2d {
    col: usize,
    row: usize,
}

impl Pos2d {
    pub fn p(col: usize, row: usize) -> Pos2d {
        Pos2d { col, row }
    }
}
impl Eq for Pos2d {}

#[derive(Debug, Copy, Clone)]
pub struct Distance2d {
    pos: Pos2d,
    dist: usize,
}
impl Distance2d {
    pub fn d(pos: Pos2d, dist: usize) -> Distance2d {
        Distance2d { pos, dist }
    }
    pub fn p(col: usize, row: usize, dist: usize) -> Distance2d {
        Distance2d {
            pos: Pos2d::p(col, row),
            dist,
        }
    }
    pub fn pos(&self) -> Pos2d {
        self.pos
    }
    pub fn dist(&self) -> usize {
        self.dist
    }
    pub fn col(&self) -> usize {
        self.pos.col
    }
    pub fn row(&self) -> usize {
        self.pos.row
    }
}

pub type DistanceMap = HashMap<Pos2d, usize>;
pub type Pos2dVec = Vec<Pos2d>;

pub struct Grid {
    width: usize,
    length: usize,
    cells: Cells,
    visited: HashSet<Pos2d>,
    distances: DistanceMap,
    frontier: Pos2dVec,
}

impl Grid {
    pub fn new(width: usize, length: usize) -> Grid {
        Grid {
            width,
            length,
            cells: Vec::with_capacity(width * length),
            visited: HashSet::with_capacity(width * length),
            distances: HashMap::with_capacity(width * length),
            frontier: Vec::with_capacity(width * length),
        }
    }

    pub fn init(&mut self) {
        for row in 0..self.length() {
            for col in 0..self.width() {
                let cell = Cell::new(col, row);
                self.cells.push(cell);
            }
        }
        self.insert_distance(Distance2d::p(0, 0, 0));
    }

    pub fn reset_path(&mut self) {
        self.frontier.clear();
        self.visited.clear();
    }

    pub fn entrance(&mut self, entrance: Pos2d) {
        self.frontier.push(entrance);
        self.visit_pos(entrance);
    }

    pub fn visit_pos(&mut self, pos: Pos2d) -> bool {
        self.visited.insert(pos)
    }

    pub fn is_cell_visited(&self, pos: Pos2d) -> bool {
        self.visited.contains(&pos)
    }

    pub fn display(&self) {
        let mut col = 1;
        let mut cells = self.cells();
        debug!(Grid::logger(), "{:?}", cells);
        cells.sort_by_key(|c| self.cells_len() - c.row());
        debug!(Grid::logger(), "{:?}", cells);
        for cell in cells {
            print!("{}|", cell.to_string());
            if col % self.width() == 0 {
                println!();
            }
            col += 1;
        }
    }

    pub fn display_distances(&self) {
        let mut col = 1;
        let mut cells = self.cells();
        debug!(Grid::logger(), "{:?}", cells);
        cells.sort_by_key(|c| self.cells_len() - c.row());
        debug!(Grid::logger(), "{:?}", cells);
        println!("distances =");
        for cell in cells {
            let d2d = match self.distance_of_cell(Pos2d::p(cell.col(), cell.row())) {
                Some(&dist) => Distance2d::p(cell.col(), cell.row(), dist),
                None => Distance2d::p(0, 0, 0),
            };
            print!("{},{}-{}|", d2d.col(), d2d.row(), radix_36(d2d.dist()));
            if col % self.width() == 0 {
                println!();
            }
            col += 1;
        }
    }

    fn do_render_ascii(&self, dist_map: Option<&DistanceMap>) {
        let mut col = 1;
        let mut cells = self.cells();
        debug!(Grid::logger(), "{:?}", cells);
        cells.sort_by_key(|c| self.cells_len() - c.row());
        debug!(Grid::logger(), "{:?}", cells);

        let mut output = format!("{}{}\n", "+", "---+".repeat(self.width()));

        let mut top = "|".to_string();
        let mut bottom = "+".to_string();

        for cell in cells {
            let body = match dist_map {
                Some(crumbs) => match crumbs.get(&Pos2d::p(cell.col(), cell.row())) {
                    Some(&dist) => format!(" {} ", radix_36(dist)),
                    None => "   ".to_string(),
                },
                None => match self.distance_of_cell(Pos2d::p(cell.col(), cell.row())) {
                    Some(&dist) => format!(" {} ", radix_36(dist)),
                    None => "   ".to_string(),
                },
            };

            let east_boundary = if cell.east {
                " ".to_string()
            } else {
                "|".to_string()
            };
            top.push_str(&format!("{}{}", body, east_boundary));

            let south_boundary = if cell.south { "   " } else { "---" };
            let corner = "+";
            bottom.push_str(&format!("{}{}", south_boundary, corner));
            if col % self.width() == 0 {
                output.push_str(&format!("{}\n", top));
                output.push_str(&format!("{}\n", bottom));
                top = "|".to_string();
                bottom = "+".to_string();
            }
            col += 1;
        }
        print!("{}", output);
    }
    pub fn render_ascii(&self) {
        self.do_render_ascii(None);
    }

    pub fn render_ascii_path(&self, dist_map: DistanceMap) {
        self.do_render_ascii(Some(&dist_map));
    }

    fn background_for_cell(&self, dist: usize) -> Rgb<u8> {
        let max_path = self.max_path_from(Pos2d::p(0, 0));
        let max_dist = max_path.dist();

        let intensity = (max_dist - dist) as f32 / max_dist as f32;
        let dark = ((255.0 * intensity).round()) as u8;
        let bright = (128.0 + (127.0 * intensity).round()) as u8;
        Rgb([dark, bright, dark])
    }
    fn do_render_png(&self, name: &str, dist_map: Option<&DistanceMap>) {
        let filename = Path::new(&name);

        let img_width = 800u32;
        let img_height = 800u32;

        let grid_size = if self.width > std::i32::MAX as usize {
            None
        } else {
            Some(self.width as i32)
        };
        let cell_size = 700i32 / grid_size.unwrap();
        let left_margin = 50i32;
        let bottom_margin = 50i32 + (grid_size.unwrap() * cell_size);

        let black = Rgb([0u8, 0u8, 0u8]);
        // Colors werere used for debugging drawing
        // let red = Rgb([128u8, 0u8, 0u8]);
        // let green = Rgb([0u8, 128u8, 0u8]);
        // let blue = Rgb([0u8, 0u8, 128u8]);
        // let gray = Rgb([128u8, 128u8, 128u8]);
        let white = Rgb([255u8, 255u8, 255u8]);

        let mut image = RgbImage::from_pixel(img_width, img_height, white);

        let cells = self.cells();
        let line_width = 2u32;

        for cell in &cells {
            let bg_color = match dist_map {
                Some(dist_map) => match dist_map.get(&Pos2d::p(cell.col(), cell.row())) {
                    Some(&dist) => self.background_for_cell(dist),
                    None => self.background_for_cell(0),
                },
                None => match self.distance_of_cell(Pos2d::p(cell.col(), cell.row())) {
                    Some(&dist) => self.background_for_cell(dist),
                    None => self.background_for_cell(0),
                },
            };

            let inner_x = cell.inner_x(left_margin, cell_size, line_width);
            let inner_y = cell.inner_y(bottom_margin, cell_size, line_width);
            let inner_w = cell.inner_w(cell_size, line_width);
            let inner_h = cell.inner_h(cell_size, line_width);
            drawing::draw_filled_rect_mut(
                &mut image,
                Rect::at(inner_x, inner_y).of_size(inner_w, inner_h),
                bg_color,
            );

            if !cell.north {
                let x = left_margin + (cell.col() as i32 * cell_size);
                let y = bottom_margin - (cell.row() as i32 * cell_size) - cell_size;
                let width = cell_size as u32;
                let height = line_width;
                debug!(
                    Grid::logger(),
                    "x={:?}, y={:?}, width={:?}, height={:?}, black, {}, north==false",
                    x,
                    y,
                    width,
                    height,
                    cell.to_string()
                );

                drawing::draw_filled_rect_mut(
                    &mut image,
                    Rect::at(x, y).of_size(width, height),
                    black,
                );
            }
            if !cell.east {
                let x = left_margin + (cell.col() as i32 * cell_size) + cell_size;
                let y = bottom_margin - (cell.row() as i32 * cell_size) - cell_size;
                let width = line_width;
                let height = cell_size as u32;
                debug!(
                    Grid::logger(),
                    "x={:?}, y={:?}, width={:?}, height={:?}, green, {}, east==false",
                    x,
                    y,
                    width,
                    height,
                    cell.to_string()
                );

                drawing::draw_filled_rect_mut(
                    &mut image,
                    Rect::at(x, y).of_size(width, height),
                    black,
                );
            }
            if !cell.south {
                let x = left_margin + (cell.col() as i32 * cell_size);
                let y = bottom_margin - (cell.row() as i32 * cell_size);
                let width = cell_size as u32;
                let height = line_width;
                debug!(
                    Grid::logger(),
                    "x={:?}, y={:?}, width={:?}, height={:?}, red, {}, south==false",
                    x,
                    y,
                    width,
                    height,
                    cell.to_string()
                );

                drawing::draw_filled_rect_mut(
                    &mut image,
                    Rect::at(x, y).of_size(width, height),
                    black,
                );
            }
            if !cell.west {
                let x = left_margin + (cell.col() as i32 * cell_size);
                let y = bottom_margin - (cell.row() as i32 * cell_size) - cell_size;
                let width = line_width;
                let height = cell_size as u32;
                debug!(
                    Grid::logger(),
                    "x={:?}, y={:?}, width={:?}, height={:?}, blue, {}, west==false",
                    x,
                    y,
                    width,
                    height,
                    cell.to_string()
                );

                drawing::draw_filled_rect_mut(
                    &mut image,
                    Rect::at(x, y).of_size(width, height),
                    black,
                );
            }
        }

        image.save(filename).unwrap();
    }

    pub fn render_png(&self, name: &str) {
        self.do_render_png(name, None);
    }

    pub fn render_png_path(&self, name: &str, dist_map: DistanceMap) {
        self.do_render_png(name, Some(&dist_map));
    }

    pub fn calculate_distances(&mut self) {
        let cur_front = &self.frontier;
        self.do_calculate_distances(cur_front.to_vec());
    }

    pub fn distance_map(&self) -> DistanceMap {
        self.distances.clone()
    }

    pub fn distance_of_cell(&self, pos: Pos2d) -> Option<&usize> {
        self.distances.get(&pos)
    }

    pub fn insert_distance(&mut self, distance: Distance2d) -> Option<usize> {
        self.distances.insert(distance.pos(), distance.dist())
    }

    pub fn max_path_from(&self, start: Pos2d) -> Distance2d {
        let mut max_dist: usize = 0;
        let mut max_pos = start;
        let dist_map = self.distance_map();
        for (pos, dist) in dist_map.iter() {
            if dist > &max_dist {
                max_pos = Pos2d::p(pos.col, pos.row);
                max_dist = *dist;
            }
        }
        Distance2d::d(max_pos, max_dist)
    }
    pub fn plot_path_between(&mut self, start: Pos2d, end: Pos2d) -> DistanceMap {
        let mut dist_map: DistanceMap = HashMap::with_capacity(self.width * self.length);
        let mut cur_pos = end;
        let mut cur_dist = match self.distance_of_cell(cur_pos) {
            Some(&dist) => dist,
            None => 0,
        };

        dist_map.insert(cur_pos, cur_dist);

        while cur_pos != start {
            let cell = self.cell_at(cur_pos.col, cur_pos.row).unwrap();
            if cell.north {
                let next_pos = self.next_north(cell.col(), cell.row());
                let next_dist = match self.distance_of_cell(next_pos) {
                    Some(&dist) => dist,
                    None => 0,
                };
                if next_dist < cur_dist {
                    dist_map.insert(next_pos, next_dist);
                    cur_pos = next_pos;
                    cur_dist = next_dist;
                }
            }
            if cell.east {
                let next_pos = self.next_east(cell.col(), cell.row());
                let next_dist = match self.distance_of_cell(next_pos) {
                    Some(&dist) => dist,
                    None => 0,
                };
                if next_dist < cur_dist {
                    dist_map.insert(next_pos, next_dist);
                    cur_pos = next_pos;
                    cur_dist = next_dist;
                }
            }
            if cell.south {
                let next_pos = self.next_south(cell.col(), cell.row());
                let next_dist = match self.distance_of_cell(next_pos) {
                    Some(&dist) => dist,
                    None => 0,
                };
                if next_dist < cur_dist {
                    dist_map.insert(next_pos, next_dist);
                    cur_pos = next_pos;
                    cur_dist = next_dist;
                }
            }
            if cell.west {
                let next_pos = self.next_west(cell.col(), cell.row());
                let next_dist = match self.distance_of_cell(next_pos) {
                    Some(&dist) => dist,
                    None => 0,
                };
                if next_dist < cur_dist {
                    dist_map.insert(next_pos, next_dist);
                    cur_pos = next_pos;
                    cur_dist = next_dist;
                }
            }
        }

        dist_map
    }

    fn do_calculate_distances(&mut self, frontier: Pos2dVec) {
        let mut new_frontier: Pos2dVec = vec![];
        debug!(Grid::logger(), "frontier = {:?}", frontier);
        if !frontier.is_empty() {
            for pos in frontier.iter() {
                let cell = self.cell_at(pos.col, pos.row).unwrap();
                let cur_pos = *pos; //pos.clone();

                let cur_dist = match self.distance_of_cell(cur_pos) {
                    Some(&dist) => dist,
                    None => 0,
                };
                debug!(
                    Grid::logger(),
                    "cur_pos = {:?}, cur_dist={}", cur_pos, cur_dist
                );
                if cell.north {
                    let next_pos = self.next_north(cell.col(), cell.row());
                    if !self.is_cell_visited(next_pos) {
                        self.insert_distance(Distance2d::d(next_pos, cur_dist + 1));
                        new_frontier.push(next_pos);
                        self.visit_pos(cur_pos);
                    }
                }
                if cell.east {
                    let next_pos = self.next_east(cell.col(), cell.row());
                    if !self.is_cell_visited(next_pos) {
                        self.insert_distance(Distance2d::d(next_pos, cur_dist + 1));
                        new_frontier.push(next_pos);
                        self.visit_pos(cur_pos);
                    }
                }
                if cell.south {
                    let next_pos = self.next_south(cell.col(), cell.row());
                    if !self.is_cell_visited(next_pos) {
                        self.insert_distance(Distance2d::d(next_pos, cur_dist + 1));
                        new_frontier.push(next_pos);
                        self.visit_pos(cur_pos);
                    }
                }
                if cell.west {
                    let next_pos = self.next_west(cell.col(), cell.row());
                    if !self.is_cell_visited(next_pos) {
                        self.insert_distance(Distance2d::d(next_pos, cur_dist + 1));
                        new_frontier.push(next_pos);
                        self.visit_pos(cur_pos);
                    }
                }
            }
            self.do_calculate_distances(new_frontier);
        }
    }

    pub fn cell_at(&self, col: usize, row: usize) -> Option<Cell> {
        match (col, row) {
            (col, _) if col >= self.width() => None,
            (_, row) if row >= self.length() => None,
            (col, row) => {
                let index = (self.width() * row) + col;
                if index < (self.width() * self.length()) {
                    Some(self.cells[index])
                } else {
                    None
                }
            }
        }
    }

    pub fn update_cell_at(&mut self, col: usize, row: usize, cell: Cell) {
        match (col, row) {
            (col, _) if col >= self.width() => panic!("Tried to update ({},{})", col, row),
            (_, row) if row >= self.length() => panic!("Tried to update ({},{})", col, row),
            (col, row) => {
                let index = (self.width() * row) + col;
                if index < (self.width() * self.length()) {
                    self.cells[index] = cell
                } else {
                    panic!("Tried to update ({},{})", col, row)
                }
            }
        }
    }

    pub fn cells(&self) -> Cells {
        self.cells.clone()
    }

    pub fn cells_len(&self) -> usize {
        self.cells.len()
    }

    pub fn link_north(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_north();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col, row + 1) {
            cell.link_south();
            self.update_cell_at(col, row + 1, cell);
        }
    }

    pub fn next_north(&mut self, col: usize, row: usize) -> Pos2d {
        if !self.at_upper(row) {
            Pos2d::p(col, row + 1)
        } else {
            Pos2d::p(col, row)
        }
    }

    pub fn link_east(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_east();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col + 1, row) {
            cell.link_west();
            self.update_cell_at(col + 1, row, cell);
        }
    }

    pub fn next_east(&mut self, col: usize, row: usize) -> Pos2d {
        if !self.at_right(col) {
            Pos2d::p(col + 1, row)
        } else {
            Pos2d::p(col, row)
        }
    }

    pub fn link_south(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_south();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col, row - 1) {
            cell.link_north();
            self.update_cell_at(col, row - 1, cell);
        }
    }

    pub fn next_south(&mut self, col: usize, row: usize) -> Pos2d {
        if !self.at_lower(row) {
            Pos2d::p(col, row - 1)
        } else {
            Pos2d::p(col, row)
        }
    }

    pub fn link_west(&mut self, col: usize, row: usize) {
        if let Some(mut cell) = self.cell_at(col, row) {
            cell.link_west();
            self.update_cell_at(col, row, cell);
        }
        if let Some(mut cell) = self.cell_at(col - 1, row) {
            cell.link_east();
            self.update_cell_at(col - 1, row, cell);
        }
    }

    pub fn next_west(&mut self, col: usize, row: usize) -> Pos2d {
        if !self.at_left(col) {
            Pos2d::p(col - 1, row)
        } else {
            Pos2d::p(col, row)
        }
    }

    pub fn at_upper(&self, row: usize) -> bool {
        (self.length == row + 1)
    }

    pub fn at_lower(&self, row: usize) -> bool {
        (1 == row + 1)
    }

    pub fn at_right(&self, col: usize) -> bool {
        (self.width == col + 1)
    }

    pub fn at_left(&self, col: usize) -> bool {
        (1 == col + 1)
    }

    pub fn at_upper_left(&self, col: usize, row: usize) -> bool {
        self.at_upper(row) && self.at_left(col)
    }

    pub fn at_upper_right(&self, col: usize, row: usize) -> bool {
        self.at_upper(row) && self.at_right(col)
    }

    pub fn at_lower_left(&self, col: usize, row: usize) -> bool {
        self.at_lower(row) && self.at_left(col)
    }

    pub fn at_lower_right(&self, col: usize, row: usize) -> bool {
        self.at_lower(row) && self.at_right(col)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn length(&self) -> usize {
        self.length
    }

    fn logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();

        slog::Logger::root(drain, o!())
    }
}

#[cfg(test)]
mod grid_tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1 == 0, false);
    }

    #[test]
    fn new() {
        let mut grid = Grid::new(4, 4);
        assert_eq!(grid.length(), 4);
        assert_eq!(grid.width(), 4);

        grid.init();
        assert_eq!(grid.cells_len(), 16);

        assert_eq!(grid.at_left(0), true);
        assert_eq!(grid.at_right(3), true);
        assert_eq!(grid.at_lower(0), true);
        assert_eq!(grid.at_upper(3), true);

        assert_eq!(grid.at_lower_left(0, 0), true);
        assert_eq!(grid.at_upper_left(0, 3), true);

        assert_eq!(grid.at_lower_right(3, 0), true);
        assert_eq!(grid.at_upper_right(3, 3), true);
    }

    #[test]
    fn cell_at_0_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 0);
        assert_eq!(test, Some(Cell::new(0, 0)));
    }

    #[test]
    fn cell_at_1_1() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(1, 1);
        assert_eq!(test, Some(Cell::new(1, 1)));
    }

    #[test]
    fn cell_at_2_2() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(2, 2);
        assert_eq!(test, Some(Cell::new(2, 2)));
    }

    #[test]
    fn cell_at_1_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(1, 0);
        assert_eq!(test, Some(Cell::new(1, 0)));
    }

    #[test]
    fn cell_at_3_0() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(3, 0);
        assert_eq!(test, None);
        // assert_eq!(grid.cell_at(0, 0).east, false);
    }

    #[test]
    fn cell_at_0_1() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 1);
        assert_eq!(test, Some(Cell::new(0, 1)));
    }

    #[test]
    fn cell_at_0_3() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        let test = grid.cell_at(0, 3);
        assert_eq!(test, None);
    }

    #[test]
    fn link_north() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_north(0, 0);

        let mut cell_0 = Cell::new(0, 0);
        let mut cell_1 = Cell::new(0, 1);
        cell_0.link_north();
        cell_1.link_south();

        let test_0 = grid.cell_at(0, 0);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(0, 1);
        assert_eq!(test_1, Some(cell_1));
    }

    #[test]
    fn link_east() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_east(0, 0);

        let mut cell_0 = Cell::new(0, 0);
        let mut cell_1 = Cell::new(1, 0);
        cell_0.link_east();
        cell_1.link_west();

        let test_0 = grid.cell_at(0, 0);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(1, 0);
        assert_eq!(test_1, Some(cell_1));
    }

    #[test]
    fn link_south() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_south(2, 2);

        let mut cell_0 = Cell::new(2, 2);
        let mut cell_1 = Cell::new(2, 1);
        cell_0.link_south();
        cell_1.link_north();

        let test_0 = grid.cell_at(2, 2);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(2, 1);
        assert_eq!(test_1, Some(cell_1));
    }
    #[test]
    fn link_west() {
        let mut grid = Grid::new(3, 3);
        grid.init();
        grid.link_west(1, 1);

        let mut cell_0 = Cell::new(1, 1);
        let mut cell_1 = Cell::new(0, 1);
        cell_0.link_west();
        cell_1.link_east();

        let test_0 = grid.cell_at(1, 1);
        assert_eq!(test_0, Some(cell_0));

        let test_1 = grid.cell_at(0, 1);
        assert_eq!(test_1, Some(cell_1));
    }
}
