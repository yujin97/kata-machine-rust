pub type WeightedAdjacencyMatrix = Vec<Vec<usize>>;

pub fn get_matrix_2() -> WeightedAdjacencyMatrix {
    vec![
        vec![0, 3, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 7, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 5, 0, 2, 0],
        vec![0, 0, 18, 0, 0, 0, 1],
        vec![0, 0, 0, 1, 0, 0, 1],
    ]
}
