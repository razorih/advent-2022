static INPUT: &'static str = include_str!("input/day12.txt");

#[derive(Debug, Default)]
struct Grid {
    vertices: Vec<u8>, // range a-z can be represented with u8
    width: usize,
    height: usize,
    start: usize, // Index of starting vertex S
    end: usize, // Index of ending vertex E
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
    /// Consumes self and returns neighbouring vertex indices in random order.
    fn into_iter(self) -> impl Iterator<Item = usize> {
        [self.up, self.left, self.down, self.right]
            .into_iter()
            .filter_map(|dir| dir)
    }
}

impl Grid {
    fn new(s: &str, width: usize, height: usize) -> Self {
        let mut grid = Grid::default();
        grid.vertices.reserve_exact(width * height);
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
            });
        }

        grid
    }

    fn get(&self, col: usize, row: usize) -> u8 {
        self.vertices[row*self.width + col]
    }

    fn get_i(&self, v: usize) -> u8 {
        self.vertices[v]
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

    /// Similar to [`Grid::neighbours`] but accepts coordinate as an index
    fn neighbours_i(&self, v: usize) -> Neighbours {
        let col = v / self.width;
        let row = v % self.width;
        self.neighbours(row, col)
    }

    /// Returns breadth-first tree starting from given root vertice index.
    fn breadth_first_search(&self, root: usize) -> BreadthFirstSearchTree {
        use std::collections::VecDeque;

        // List of discovered vertices
        let mut discovered = vec![false; self.vertices.len()];
        // Vertices' parent nodes.
        // Each vertex `v` (except root) has exactly one parent vertex `y`.
        // Parent vertex can be found with `y = parents[v]`
        // `y` is None if it's the root.
        let mut parents = vec![None; self.vertices.len()];

        // Queue of vertices to be visited
        let mut queue: VecDeque<usize> = VecDeque::new();

        queue.push_back(root);
        discovered[root] = true;

        while let Some(v) = queue.pop_front() {
            // When solving Gold stop when first 'a' has been found.
            // Write down found index on a piece of paper.
            // if self.get_i(v) == b'a' {
            //     println!("=== First `a` at index: {v} ===");
            //     break;
            // }

            for y in self.neighbours_i(v).into_iter() {
                if !discovered[y] {
                    queue.push_back(y);
                    discovered[y] = true;
                    parents[y] = Some(v);
                }
            }
        }

        BreadthFirstSearchTree {
            tree: parents,
            root,
            grid: &self,
        }
    }
}

/// Represents result of Breadth First Search.
/// Result is closely related to [`Grid`] and thus only lives as long the
/// grid it was sourced from lives.
struct BreadthFirstSearchTree<'a> {
    tree: Vec<Option<usize>>,
    root: usize,
    grid: &'a Grid,
}

impl<'a> BreadthFirstSearchTree<'a> {
    /// Returns shortest path to given index.
    fn find_path(&self, end: Option<usize>) -> Vec<usize> {
        let mut path: Vec<usize> = Vec::new();
        let mut cursor = end;

        while !(cursor.is_none() || cursor.unwrap() == self.root) {
            path.push(cursor.unwrap());
            cursor = self.tree[cursor.unwrap()];
        }

        path.push(self.root);
        path
    }
}

pub fn silver() {
    let grid = Grid::new(INPUT, 67, 41);

    let tree = grid.breadth_first_search(grid.start);
    let path = tree.find_path(Some(grid.end));
    
    println!("Silver: {}", path.len() - 1); // Answer is the number of edges, not vertices
}

pub fn gold() {
    let grid = Grid::new(INPUT, 67, 41);
    // Found during tree construction in `Grid::breadth_first_search`
    const MAGIC: usize = 1742;

    // Tree is now formed starting from the end
    let tree = grid.breadth_first_search(grid.end);
    let path = tree.find_path(Some(MAGIC));

    println!("Gold: {}", path.len() - 1);
}
