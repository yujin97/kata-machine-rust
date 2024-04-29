pub type WeightedAdjacencyMatrix = Vec<Vec<usize>>;

pub struct GraphEdge {
    pub to: usize,
    pub weight: usize,
}

pub type WeightedAdjacencyList = Vec<Vec<GraphEdge>>;

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

pub fn get_list_1() -> WeightedAdjacencyList {
    let mut list = Vec::new();

    list.push(vec![
        GraphEdge { to: 1, weight: 3 },
        GraphEdge { to: 2, weight: 1 },
    ]);

    list.push(vec![
        GraphEdge { to: 0, weight: 3 },
        GraphEdge { to: 2, weight: 4 },
        GraphEdge { to: 4, weight: 1 },
    ]);

    list.push(vec![
        GraphEdge { to: 1, weight: 4 },
        GraphEdge { to: 3, weight: 7 },
        GraphEdge { to: 0, weight: 1 },
    ]);

    list.push(vec![
        GraphEdge { to: 2, weight: 7 },
        GraphEdge { to: 4, weight: 5 },
        GraphEdge { to: 6, weight: 1 },
    ]);

    list.push(vec![
        GraphEdge { to: 1, weight: 1 },
        GraphEdge { to: 3, weight: 5 },
        GraphEdge { to: 5, weight: 2 },
    ]);

    list.push(vec![
        GraphEdge { to: 6, weight: 1 },
        GraphEdge { to: 4, weight: 2 },
        GraphEdge { to: 2, weight: 10 },
    ]);

    list.push(vec![
        GraphEdge { to: 3, weight: 1 },
        GraphEdge { to: 5, weight: 1 },
    ]);

    list
}

pub fn get_list_2() -> WeightedAdjacencyList {
    let mut list = Vec::new();

    list.push(vec![
        GraphEdge { to: 1, weight: 3 },
        GraphEdge { to: 2, weight: 1 },
    ]);

    list.push(vec![GraphEdge { to: 4, weight: 1 }]);

    list.push(vec![GraphEdge { to: 3, weight: 7 }]);

    list.push(vec![]);

    list.push(vec![
        GraphEdge { to: 1, weight: 1 },
        GraphEdge { to: 3, weight: 5 },
        GraphEdge { to: 5, weight: 2 },
    ]);

    list.push(vec![
        GraphEdge { to: 2, weight: 10 },
        GraphEdge { to: 6, weight: 1 },
    ]);

    list.push(vec![GraphEdge { to: 3, weight: 1 }]);

    list
}
