use std::collections::VecDeque;

type WeightedAdjacentMatrix = Vec<Vec<usize>>;

fn bfs(graph: WeightedAdjacentMatrix, source: usize, needle: usize) -> Option<Vec<usize>> {
    let mut seen = vec![false; graph.len()];
    let mut prev = vec![None; graph.len()];

    seen[source] = true;

    let mut q = VecDeque::new();
    q.push_back(source);

    while q.len() > 0 {
        let curr = q.pop_front().expect("Failed to pop front");

        if curr == needle {
            break;
        }

        for (idx, adj) in graph[curr].iter().enumerate() {
            if adj == &0 {
                continue;
            }

            if seen[idx] == true {
                continue;
            }

            seen[idx] = true;
            prev[idx] = Some(curr);
            q.push_back(idx);
        }
    }

    if prev[needle].is_none() {
        return None;
    }

    let mut curr = needle;
    let mut out = Vec::new();

    while prev[curr].is_some() {
        out.push(curr);
        curr = prev[curr].expect("Faield to access prev of current");
    }

    if out.len() > 0 {
        out.reverse();
        return Some(vec![vec![source], out].concat());
    }

    None
}

fn main() {}
