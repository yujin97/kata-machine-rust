use dsa::graph::WeightedAdjacencyList;

#[allow(unused)]
fn has_unvisited(seen: &Vec<bool>, dists: &Vec<f64>) -> bool {
    for (i, s) in seen.iter().enumerate() {
        if !s && dists[i] < f64::INFINITY {
            return true;
        }
    }

    false
}

#[allow(unused)]
fn get_lowest_unvisited(seen: &Vec<bool>, dists: &Vec<f64>) -> Option<usize> {
    let mut idx = None;
    let mut lowest_distance = f64::INFINITY;

    for (i, d) in dists.iter().enumerate() {
        if seen[i] {
            continue;
        }

        if lowest_distance > *d {
            lowest_distance = *d;
            idx = Some(i);
        }
    }

    idx
}

#[allow(unused)]
fn dijkstra_list(source: usize, sink: usize, arr: WeightedAdjacencyList) -> Vec<usize> {
    let mut seen = vec![false; arr.len()];
    let mut prev = vec![None; arr.len()];
    let mut dists = vec![f64::INFINITY; arr.len()];
    dists[source] = 0.0;

    while has_unvisited(&seen, &dists) {
        let curr = get_lowest_unvisited(&seen, &dists).unwrap();

        seen[curr] = true;

        for edge in &arr[curr] {
            if seen[edge.to] {
                continue;
            }

            let dist = dists[curr] + edge.weight as f64;
            if dist < dists[edge.to] {
                dists[edge.to] = dist;
                prev[edge.to] = Some(curr);
            }
        }
    }

    let mut out = vec![];
    let mut curr = sink;

    while prev[curr].is_some() {
        out.push(curr);
        curr = prev[curr].unwrap();
    }

    out.push(source);

    out.reverse();
    out
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use dsa::graph::get_list_1;

    #[test]
    fn dijkstra_works_on_adj_list() {
        let list = get_list_1();

        assert_eq!(dijkstra_list(0, 6, list), vec![0, 1, 4, 5, 6]);
    }
}
