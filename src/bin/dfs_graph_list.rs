use dsa::graph::WeightedAdjacencyList;

#[allow(unused)]
#[derive(Debug)]
struct WalkResult {
    result: bool,
    seen: Vec<bool>,
    path: Vec<usize>,
}

#[allow(unused)]
fn walk(
    graph: &WeightedAdjacencyList,
    curr: usize,
    needle: usize,
    mut seen: Vec<bool>,
    mut path: Vec<usize>,
) -> WalkResult {
    let mut result;

    if seen[curr] == true {
        return WalkResult {
            result: false,
            seen,
            path,
        };
    }

    seen[curr] = true;

    // recurse
    // pre
    path.push(curr);

    if curr == needle {
        return WalkResult {
            result: true,
            seen,
            path,
        };
    }
    // recurse
    let list = &graph[curr];
    for edge in list {
        WalkResult { result, seen, path } = walk(graph, edge.to, needle, seen, path);

        if result {
            return WalkResult { result, path, seen };
        }
    }
    // post
    path.pop();

    return WalkResult {
        result: false,
        seen,
        path,
    };
}

#[allow(unused)]
fn dfs(graph: WeightedAdjacencyList, source: usize, needle: usize) -> Option<Vec<usize>> {
    let seen = vec![false; graph.len()];
    let path = Vec::new();

    let WalkResult { result, path, .. } = walk(&graph, source, needle, seen, path);

    if result {
        return Some(path);
    } else {
        return None;
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::graph::get_list_2;

    #[test]
    fn dfs_works() {
        let list = get_list_2();
        let expected = vec![0, 1, 4, 5, 6];

        assert_eq!(dfs(list, 0, 6), Some(expected));
    }
}
