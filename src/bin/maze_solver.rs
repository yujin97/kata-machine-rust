#[derive(PartialEq, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(x: isize, y: isize) -> Option<Self> {
        if x >= 0 && y >= 0 {
            let x = usize::try_from(x).expect("Failed to parse to Point");
            let y = usize::try_from(y).expect("Failed to parse to Point");

            Some(Point { x, y })
        } else {
            None
        }
    }

    fn move_from_point(&self, direction: Direction) -> Option<Point> {
        let curr_x = isize::try_from(self.x).expect("Failed to parse when moving from Point");
        let curr_y = isize::try_from(self.y).expect("Failed to parse when moving from Point");

        match direction {
            Direction::Up => Self::parse(curr_x, curr_y - 1),
            Direction::Right => Self::parse(curr_x + 1, curr_y),
            Direction::Down => Self::parse(curr_x, curr_y + 1),
            Direction::Left => Self::parse(curr_x - 1, curr_y),
        }
    }
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct WalkResult {
    result: bool,
    seen: Vec<Vec<bool>>,
    path: Vec<Point>,
}

fn walk(
    maze: &[String],
    wall: char,
    curr: Point,
    end: Point,
    mut seen: Vec<Vec<bool>>,
    mut path: Vec<Point>,
) -> WalkResult {
    // Base cases
    if curr.y >= maze.len() || curr.x >= maze[0].len() {
        return WalkResult {
            result: false,
            seen,
            path,
        };
    }

    let current_point_value = maze[curr.y]
        .chars()
        .nth(curr.x)
        .expect("Failed to access current point value");

    if current_point_value == wall {
        return WalkResult {
            result: false,
            seen,
            path,
        };
    }

    if curr == end {
        seen[curr.y][curr.x] = true;
        path.push(curr);

        return WalkResult {
            result: true,
            seen,
            path,
        };
    }

    if seen[curr.y][curr.x] == true {
        return WalkResult {
            result: false,
            seen,
            path,
        };
    }

    // recurse
    // pre
    seen[curr.y][curr.x] = true;
    path.push(curr.clone());
    let mut result = false;

    // recurse body
    let directions = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    for direction in directions {
        let next_point = curr.move_from_point(direction);

        if let Some(next_point) = next_point {
            WalkResult { result, seen, path } =
                walk(maze, wall, next_point.clone(), end.clone(), seen, path);

            if result {
                return WalkResult { result, seen, path };
            }
        }
    }

    // post
    path.pop();

    return WalkResult { result, seen, path };
}

fn main() {
    let maze = [
        "xxxxxxxxxx x".to_string(),
        "x        x x".to_string(),
        "x        x x".to_string(),
        "x xxxxxxxx x".to_string(),
        "x          x".to_string(),
        "x xxxxxxxxxx".to_string(),
    ];

    let current = Point::parse(10, 0).unwrap();
    let end = Point::parse(1, 5).unwrap();

    let seen = vec![false; 12];
    let seen = vec![seen; 6];

    let result = walk(&maze, 'x', current, end, seen, Vec::new());

    println!("{:?}", result);
}