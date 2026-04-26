pub fn ford_fulkerson(capacity: &[Vec<f64>], source: usize, sink: usize) -> f64 {
    let n = capacity.len();
    let mut residual: Vec<Vec<f64>> = capacity.to_vec();
    let mut max_flow = 0.0;
    loop {
        let mut parent = vec![usize::MAX; n];
        let mut visited = vec![false; n];
        visited[source] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            if u == sink {
                break;
            }
            for (v, vis) in visited.iter_mut().enumerate() {
                if !*vis && residual[u][v] > 1e-10 {
                    *vis = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        if !visited[sink] {
            break;
        }
        let mut path_flow = f64::INFINITY;
        let mut v = sink;
        while v != source {
            let u = parent[v];
            if residual[u][v] < path_flow {
                path_flow = residual[u][v];
            }
            v = u;
        }
        v = sink;
        while v != source {
            let u = parent[v];
            residual[u][v] -= path_flow;
            residual[v][u] += path_flow;
            v = u;
        }
        max_flow += path_flow;
    }
    max_flow
}

pub fn min_cut(capacity: &[Vec<f64>], source: usize, sink: usize) -> Vec<(usize, usize)> {
    let n = capacity.len();
    let mut residual: Vec<Vec<f64>> = capacity.to_vec();
    loop {
        let mut parent = vec![usize::MAX; n];
        let mut visited = vec![false; n];
        visited[source] = true;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        while let Some(u) = queue.pop_front() {
            for (v, vis) in visited.iter_mut().enumerate() {
                if !*vis && residual[u][v] > 1e-10 {
                    *vis = true;
                    parent[v] = u;
                    queue.push_back(v);
                }
            }
        }
        if !visited[sink] {
            let mut cuts = Vec::new();
            for (u, &vis_u) in visited.iter().enumerate() {
                if !vis_u {
                    continue;
                }
                for (v, &vis_v) in visited.iter().enumerate() {
                    if !vis_v && capacity[u][v] > 0.0 {
                        cuts.push((u, v));
                    }
                }
            }
            return cuts;
        }
        let mut path_flow = f64::INFINITY;
        let mut v = sink;
        while v != source {
            let u = parent[v];
            if residual[u][v] < path_flow {
                path_flow = residual[u][v];
            }
            v = u;
        }
        v = sink;
        while v != source {
            let u = parent[v];
            residual[u][v] -= path_flow;
            residual[v][u] += path_flow;
            v = u;
        }
    }
}

pub fn bipartite_matching(
    adj: &[Vec<usize>],
    n_left: usize,
    n_right: usize,
) -> Vec<(usize, usize)> {
    let mut match_right = vec![usize::MAX; n_right];
    let mut result = Vec::new();
    for u in 0..n_left {
        let mut visited = vec![false; n_right];
        augment(adj, u, &mut match_right, &mut visited);
    }
    for (v, &u) in match_right.iter().enumerate() {
        if u != usize::MAX {
            result.push((u, v));
        }
    }
    result
}

fn augment(adj: &[Vec<usize>], u: usize, match_right: &mut [usize], visited: &mut [bool]) -> bool {
    for &v in &adj[u] {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        if match_right[v] == usize::MAX || augment(adj, match_right[v], match_right, visited) {
            match_right[v] = u;
            return true;
        }
    }
    false
}

pub fn min_cost_flow(
    capacity: &[Vec<f64>],
    cost: &[Vec<f64>],
    source: usize,
    sink: usize,
    max_flow_limit: f64,
) -> (f64, f64) {
    let n = capacity.len();
    let mut res_cap: Vec<Vec<f64>> = capacity.to_vec();
    let mut res_cost: Vec<Vec<f64>> = cost.to_vec();
    let mut total_flow = 0.0;
    let mut total_cost = 0.0;
    while total_flow < max_flow_limit {
        let mut dist = vec![f64::INFINITY; n];
        let mut parent = vec![usize::MAX; n];
        let mut in_queue = vec![false; n];
        dist[source] = 0.0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(source);
        in_queue[source] = true;
        while let Some(u) = queue.pop_front() {
            in_queue[u] = false;
            for v in 0..n {
                if res_cap[u][v] > 1e-10 && dist[u] + res_cost[u][v] < dist[v] {
                    dist[v] = dist[u] + res_cost[u][v];
                    parent[v] = u;
                    if !in_queue[v] {
                        queue.push_back(v);
                        in_queue[v] = true;
                    }
                }
            }
        }
        if dist[sink] == f64::INFINITY {
            break;
        }
        let mut path_flow = max_flow_limit - total_flow;
        let mut v = sink;
        while v != source {
            let u = parent[v];
            if res_cap[u][v] < path_flow {
                path_flow = res_cap[u][v];
            }
            v = u;
        }
        v = sink;
        while v != source {
            let u = parent[v];
            res_cap[u][v] -= path_flow;
            res_cap[v][u] += path_flow;
            res_cost[v][u] = -res_cost[u][v];
            v = u;
        }
        total_flow += path_flow;
        total_cost += path_flow * dist[sink];
    }
    (total_flow, total_cost)
}

pub fn edmonds_karp(capacity: &[Vec<f64>], source: usize, sink: usize) -> f64 {
    ford_fulkerson(capacity, source, sink)
}

pub fn edge_connectivity(adj: &[Vec<usize>]) -> f64 {
    let n = adj.len();
    if n <= 1 {
        return 0.0;
    }
    let mut cap = vec![vec![0.0; n]; n];
    for (u, neighbors) in adj.iter().enumerate() {
        for &v in neighbors {
            cap[u][v] = 1.0;
        }
    }
    let mut min_flow = f64::INFINITY;
    for t in 1..n {
        let f = ford_fulkerson(&cap, 0, t);
        if f < min_flow {
            min_flow = f;
        }
    }
    min_flow
}

pub fn vertex_connectivity(adj: &[Vec<usize>]) -> f64 {
    let n = adj.len();
    if n <= 1 {
        return 0.0;
    }
    let nn = 2 * n;
    let mut cap = vec![vec![0.0; nn]; nn];
    for i in 0..n {
        cap[2 * i][2 * i + 1] = 1.0;
    }
    for (u, neighbors) in adj.iter().enumerate() {
        for &v in neighbors {
            cap[2 * u + 1][2 * v] = f64::INFINITY;
        }
    }
    let mut min_flow = f64::INFINITY;
    for t in 1..n {
        let f = ford_fulkerson(&cap, 1, 2 * t);
        if f < min_flow {
            min_flow = f;
        }
    }
    min_flow
}

pub fn max_flow_scaling(capacity: &[Vec<f64>], source: usize, sink: usize) -> f64 {
    let n = capacity.len();
    let mut residual: Vec<Vec<f64>> = capacity.to_vec();
    let max_cap = capacity
        .iter()
        .flat_map(|row| row.iter())
        .cloned()
        .fold(0.0f64, f64::max);
    if max_cap == 0.0 {
        return 0.0;
    }
    let mut delta = 1.0;
    while delta * 2.0 <= max_cap {
        delta *= 2.0;
    }
    let mut max_flow = 0.0;
    while delta >= 1.0 {
        loop {
            let mut parent = vec![usize::MAX; n];
            let mut visited = vec![false; n];
            visited[source] = true;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(source);
            while let Some(u) = queue.pop_front() {
                if u == sink {
                    break;
                }
                for v in 0..n {
                    if !visited[v] && residual[u][v] >= delta {
                        visited[v] = true;
                        parent[v] = u;
                        queue.push_back(v);
                    }
                }
            }
            if !visited[sink] {
                break;
            }
            let mut path_flow = f64::INFINITY;
            let mut v = sink;
            while v != source {
                let u = parent[v];
                if residual[u][v] < path_flow {
                    path_flow = residual[u][v];
                }
                v = u;
            }
            v = sink;
            while v != source {
                let u = parent[v];
                residual[u][v] -= path_flow;
                residual[v][u] += path_flow;
                v = u;
            }
            max_flow += path_flow;
        }
        delta /= 2.0;
    }
    max_flow
}

pub fn multi_source_multi_sink_flow(
    capacity: &[Vec<f64>],
    sources: &[usize],
    sinks: &[usize],
) -> f64 {
    let n = capacity.len();
    let nn = n + 2;
    let super_s = n;
    let super_t = n + 1;
    let mut cap = vec![vec![0.0; nn]; nn];
    for i in 0..n {
        for j in 0..n {
            cap[i][j] = capacity[i][j];
        }
    }
    for &s in sources {
        cap[super_s][s] = f64::INFINITY;
    }
    for &t in sinks {
        cap[t][super_t] = f64::INFINITY;
    }
    ford_fulkerson(&cap, super_s, super_t)
}

pub fn circulation_demand(capacity: &[Vec<f64>], demand: &[f64]) -> bool {
    let n = capacity.len();
    let nn = n + 2;
    let super_s = n;
    let super_t = n + 1;
    let mut cap = vec![vec![0.0; nn]; nn];
    let mut total_demand = 0.0;
    for i in 0..n {
        for j in 0..n {
            cap[i][j] = capacity[i][j];
        }
    }
    for i in 0..n {
        if demand[i] > 0.0 {
            cap[super_s][i] = demand[i];
            total_demand += demand[i];
        } else if demand[i] < 0.0 {
            cap[i][super_t] = -demand[i];
        }
    }
    let f = ford_fulkerson(&cap, super_s, super_t);
    (f - total_demand).abs() < 1e-9
}

pub fn residual_graph(capacity: &[Vec<f64>], flow: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let n = capacity.len();
    let mut residual = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            residual[i][j] = capacity[i][j] - flow[i][j] + flow[j][i];
        }
    }
    residual
}

pub fn matching_size(adj: &[Vec<usize>], n_left: usize, n_right: usize) -> usize {
    bipartite_matching(adj, n_left, n_right).len()
}

pub fn is_perfect_matching(adj: &[Vec<usize>], n_left: usize, n_right: usize) -> bool {
    let m = bipartite_matching(adj, n_left, n_right);
    m.len() == n_left.min(n_right)
}
