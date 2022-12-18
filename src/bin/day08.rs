use grid::Grid;
use std::path::Path;

#[derive(Default, Clone)]
struct Visibility {
    from_left: bool,
    from_right: bool,
    from_top: bool,
    from_bottom: bool,
}

impl Visibility {
    pub fn is_visible(&self) -> bool {
        self.from_left || self.from_right || self.from_top || self.from_bottom
    }
}

fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let binding = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();
    let lines: Vec<_> = binding.lines().collect();

    let width = lines[0].len();
    let height = lines.len();

    // Parse the grid
    let heights = lines
        .into_iter()
        .flat_map(|line| line.chars())
        .map(|c| c as usize - '0' as usize)
        .collect();
    let height_grid = Grid::from_vec(heights, width);

    // TODO: Determine visibility
    let mut visibility_grid = Grid::init(height, width, Visibility::default());
    for y in 0..height {
        let mut last_height: Option<usize> = None;
        for x in 0..width {
            let tree_height = height_grid[y][x];
            visibility_grid[y][x].from_left = if let Some(lh) = last_height {
                last_height = Some(lh.max(tree_height));
                tree_height > lh
            } else {
                last_height = Some(tree_height);
                true
            };
        }
        last_height = None;
        for x in 0..width {
            let x = width - 1 - x;
            let tree_height = height_grid[y][x];
            visibility_grid[y][x].from_right = if let Some(lh) = last_height {
                last_height = Some(lh.max(tree_height));
                tree_height > lh
            } else {
                last_height = Some(tree_height);
                true
            };
        }
    }
    for x in 0..width {
        let mut last_height: Option<usize> = None;
        for y in 0..height {
            let tree_height = height_grid[y][x];
            visibility_grid[y][x].from_top = if let Some(lh) = last_height {
                last_height = Some(lh.max(tree_height));
                tree_height > lh
            } else {
                last_height = Some(tree_height);
                true
            };
        }
        last_height = None;
        for y in 0..height {
            let y = height - 1 - y;
            let tree_height = height_grid[y][x];
            visibility_grid[y][x].from_bottom = if let Some(lh) = last_height {
                last_height = Some(lh.max(tree_height));
                tree_height > lh
            } else {
                last_height = Some(tree_height);
                true
            };
        }
    }

    let visible_count = visibility_grid
        .iter()
        .filter(|vis| vis.is_visible())
        .count();

    println!("{:?}", height_grid);
    println!("Solution part 1: {}", visible_count);

    let highest_score = (0..height_grid.cols())
        .flat_map(|x| (0..height_grid.rows()).map(move |y| (x, y)))
        .map(|(x, y)| scenic_score(&height_grid, x, y))
        .max()
        .unwrap();

    println!("Solution part 2: {}", highest_score);
}

fn scenic_score(height_grid: &Grid<usize>, x: usize, y: usize) -> usize {
    let height = height_grid[y][x];
    let mut left_distance = 0;
    for nx in 0..x {
        left_distance += 1;
        if height_grid[y][x - nx - 1] >= height {
            break;
        }
    }

    let mut right_distance = 0;
    for nx in x + 1..height_grid.cols() {
        right_distance += 1;
        if height_grid[y][nx] >= height {
            break;
        }
    }
    let mut top_distance = 0;
    for ny in 0..y {
        top_distance += 1;
        if height_grid[y - ny - 1][x] >= height {
            break;
        }
    }
    let mut bottom_distance = 0;
    for ny in y + 1..height_grid.rows() {
        bottom_distance += 1;
        if height_grid[ny][x] >= height {
            break;
        }
    }
    left_distance * right_distance * bottom_distance * top_distance
}
