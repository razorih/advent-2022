static INPUT: &'static str = include_str!("input/day12.txt");

#[derive(Debug, Default)]
struct Grid {
    vertices: Vec<u8>, // range a-z can be represented with u8
    width: usize,
    height: usize,
    start: usize, // Index of starting Node S
    end: usize, // Index of ending Node E
}

#[derive(Debug, Default)]
/// Neighbours of some point
/// Indices are connected to [`Grid`]
struct Neighbours {
    up: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    down: Option<usize>,
}

impl Neighbours {
    fn as_vec(&self) -> Vec<usize> {
        // Easiest way to turn this into something iterable
        // without implementing iterator
        let mut out = Vec::with_capacity(4);

        if let Some(i) = self.up { out.push(i) }
        if let Some(i) = self.left { out.push(i) }
        if let Some(i) = self.down { out.push(i) }
        if let Some(i) = self.right { out.push(i) }

        out
    }
}

impl Grid {
    fn new(s: &str, width: usize, height: usize) -> Self {
        let mut grid = Grid::default();
        grid.vertices = Vec::with_capacity(width * height);
        grid.width = width;
        grid.height = height;

        for (i, c) in s.as_bytes().into_iter().filter(|c| c.is_ascii_alphabetic()).enumerate() {
            grid.vertices.push(match c {
                b'S' => {
                    grid.start = i;
                    b'a'
                },
                b'E' => {
                    grid.end = i;
                    b'z'
                },
                _ => *c,
            } - b'a');
        }

        grid
    }

    fn get(&self, col: usize, row: usize) -> u8 {
        self.vertices[row*self.width + col]
    }

    fn get_i(&self, v: usize) -> u8 {
        self.get(v % self.width, v / self.width)
    }

    /// Returns list of walkable vertex indices
    fn neighbours(&self, col: usize, row: usize) -> Neighbours {
        fn walkable_silver(current: u8, target: u8) -> bool { target <= current + 1 } 
        fn walkable_gold(current: u8, target: u8) -> bool { current <= target + 1}

        let walkable = walkable_gold;

        let mut out = Neighbours::default();
        let cur = self.get(col, row);

        if let Some(up_i) = row.checked_sub(1) {
            let t_i = self.width*up_i + col;
            if walkable(cur, self.vertices[t_i]) {
                out.up = Some(t_i);
            }
        }

        if let Some(left_i) = col.checked_sub(1) {
            let t_i = self.width*row + left_i;
            if walkable(cur, self.vertices[t_i]) {
                out.left = Some(t_i);
            }
        }

        if col + 1 <= self.width - 1 {
            let t_i = self.width*row + col + 1;
            if walkable(cur, self.vertices[t_i]) {
                out.right = Some(t_i);
            }
        }

        if row + 1 <= self.height - 1 {
            let t_i = self.width*(row + 1) + col;
            if walkable(cur, self.vertices[t_i]) {
                out.down = Some(t_i);
            }
        }

        out
    }

    /// Similar to [`Grid::neighbours`] but accepts coordinate as index
    fn neighbours_i(&self, v: usize) -> Neighbours {
        let col = v / self.width;
        let row = v % self.width;
        self.neighbours(row, col)
    }

    /// Form breadth-first search tree
    fn breadth_first_search(&self, root: usize) -> Vec<Option<usize>> {
        use std::collections::VecDeque;

        let mut discovered = vec![false; self.vertices.len()];
        let mut processed = vec![false; self.vertices.len()];
        let mut parents = vec![None; self.vertices.len()];

        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(root);
        discovered[root] = true;

        while let Some(v) = queue.pop_front() {
            if self.get_i(v) == 0 {
                println!("FOUND FIRST a! AT INDEX {v}");
                return parents;
            }
            processed[v] = true;

            let neigh = self.neighbours_i(v).as_vec();
            for y in neigh {
                if !processed[y] {
                    // process edge if needed
                }
                if !discovered[y] {
                    queue.push_back(y);
                    discovered[y] = true;
                    parents[y] = Some(v);
                }
            }
        }

        parents
    }
}

static mut COUNT: usize = 0;
fn path_find(start: usize, end: Option<usize>, tree: &Vec<Option<usize>>, grid: &Grid) {
    if end.is_none() {
        print!("\n|{}|", grid.get_i(start));
    } else {
        let end = end.unwrap();
        path_find(start, tree[end], tree, grid);
        unsafe { COUNT += 1 }; // oh no
        print!("->{}", grid.get_i(end));
    }
}


pub fn silver() {
    //let grid = Grid::new(INPUT, 8, 5);
    let grid = Grid::new(INPUT, 67, 41);
    //println!("{grid:?}");
    for y in 0..grid.height {
        for x in 0..grid.width {
            print!("{:2} ", grid.get(x, y));
        }
        println!();
    }

    let tree = grid.breadth_first_search(grid.start);
    println!("{:?}", tree);
    path_find(grid.start, Some(grid.end), &tree, &grid);
    println!("\ncount: {}", unsafe { COUNT - 1 });

    // Tests
    // let t = 38;
    // let neigh = grid.neighbours_i(t);

    // //println!("{:?}", grid.neighbours(p.0, p.1));
    // println!("   {:2}", grid.vertices[neigh.up.unwrap_or(0)]);
    // println!("{:2}-{:2}-{:2}", grid.vertices[neigh.left.unwrap_or(0)], grid.get_i(t), grid.vertices[neigh.right.unwrap_or(0)]);
    // println!("   {:2}", grid.vertices[neigh.down.unwrap_or(0)]);
}

pub fn gold() {
    //let grid = Grid::new(INPUT, 8, 5);
    let grid = Grid::new(INPUT, 67, 41);
    //println!("{grid:?}");
    // for y in 0..grid.height {
    //     for x in 0..grid.width {
    //         print!("{:2} ", grid.get(x, y));
    //     }
    //     println!();
    // }

    // Tree is now formed starting from the end
    let tree = grid.breadth_first_search(grid.end); /* hardcoded first 'a' index 1742 */
    println!("{:?}", tree);
    path_find(grid.end, Some(1742), &tree, &grid);
    println!("\ncount: {}", unsafe { COUNT - 1 });
}
