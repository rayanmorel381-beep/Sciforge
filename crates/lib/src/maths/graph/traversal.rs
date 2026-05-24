pub fn bfs(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut queue = std::collections::VecDeque::new();
    visited[start] = true;
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &neighbor in &adj[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
    order
}

pub fn dfs(adj: &[Vec<usize>], start: usize) -> Vec<usize> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::new();
    let mut stack = vec![start];
    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        order.push(node);
        for &neighbor in adj[node].iter().rev() {
            if !visited[neighbor] {
                stack.push(neighbor);
            }
        }
    }
    order
}

pub fn topological_sort(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut in_degree = vec![0usize; n];
    for edges in adj {
        for &v in edges {
            in_degree[v] += 1;
        }
    }
    let mut queue: std::collections::VecDeque<usize> = in_degree
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();
    let mut order = Vec::with_capacity(n);
    while let Some(node) = queue.pop_front() {
        order.push(node);
        for &v in &adj[node] {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                queue.push_back(v);
            }
        }
    }
    if order.len() == n { Some(order) } else { None }
}

pub fn connected_components(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut component = vec![usize::MAX; n];
    let mut comp_id = 0;
    for start in 0..n {
        if component[start] != usize::MAX {
            continue;
        }
        let mut stack = vec![start];
        while let Some(node) = stack.pop() {
            if component[node] != usize::MAX {
                continue;
            }
            component[node] = comp_id;
            for &v in &adj[node] {
                if component[v] == usize::MAX {
                    stack.push(v);
                }
            }
        }
        comp_id += 1;
    }
    component
}

pub fn is_bipartite(adj: &[Vec<usize>]) -> bool {
    let n = adj.len();
    let mut color = vec![-1i32; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            for &v in &adj[node] {
                if color[v] == -1 {
                    color[v] = 1 - color[node];
                    queue.push_back(v);
                } else if color[v] == color[node] {
                    return false;
                }
            }
        }
    }
    true
}

pub fn has_cycle_directed(adj: &[Vec<usize>]) -> bool {
    let n = adj.len();
    let mut state = vec![0u8; n];
    for i in 0..n {
        if state[i] == 0 && dfs_cycle(adj, i, &mut state) {
            return true;
        }
    }
    false
}

fn dfs_cycle(adj: &[Vec<usize>], node: usize, state: &mut [u8]) -> bool {
    state[node] = 1;
    for &v in &adj[node] {
        if state[v] == 1 {
            return true;
        }
        if state[v] == 0 && dfs_cycle(adj, v, state) {
            return true;
        }
    }
    state[node] = 2;
    false
}

pub fn bfs_distance(adj: &[Vec<usize>], start: usize) -> Vec<i64> {
    let n = adj.len();
    let mut dist = vec![-1i64; n];
    dist[start] = 0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    while let Some(node) = queue.pop_front() {
        for &v in &adj[node] {
            if dist[v] == -1 {
                dist[v] = dist[node] + 1;
                queue.push_back(v);
            }
        }
    }
    dist
}

pub fn iterative_deepening_dfs(adj: &[Vec<usize>], start: usize, max_depth: usize) -> Vec<usize> {
    let mut result = Vec::new();
    for depth in 0..=max_depth {
        let mut visited = vec![false; adj.len()];
        depth_limited_dfs(adj, start, depth, &mut visited, &mut result);
        if !result.is_empty() {
            return result;
        }
    }
    result
}

fn depth_limited_dfs(
    adj: &[Vec<usize>],
    node: usize,
    limit: usize,
    visited: &mut [bool],
    order: &mut Vec<usize>,
) {
    visited[node] = true;
    order.push(node);
    if limit > 0 {
        for &v in &adj[node] {
            if !visited[v] {
                depth_limited_dfs(adj, v, limit - 1, visited, order);
            }
        }
    }
}

pub fn strongly_connected_components(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();
    let mut visited = vec![false; n];
    let mut finish_order = Vec::new();
    for i in 0..n {
        if !visited[i] {
            scc_dfs1(adj, i, &mut visited, &mut finish_order);
        }
    }
    let mut radj = vec![Vec::new(); n];
    for (u, neighbors) in adj.iter().enumerate() {
        for &v in neighbors {
            radj[v].push(u);
        }
    }
    let mut comp = Vec::new();
    let mut visited2 = vec![false; n];
    for &node in finish_order.iter().rev() {
        if !visited2[node] {
            let mut c = Vec::new();
            scc_dfs2(&radj, node, &mut visited2, &mut c);
            comp.push(c);
        }
    }
    comp
}

fn scc_dfs1(adj: &[Vec<usize>], node: usize, visited: &mut [bool], order: &mut Vec<usize>) {
    visited[node] = true;
    for &v in &adj[node] {
        if !visited[v] {
            scc_dfs1(adj, v, visited, order);
        }
    }
    order.push(node);
}

fn scc_dfs2(adj: &[Vec<usize>], node: usize, visited: &mut [bool], comp: &mut Vec<usize>) {
    visited[node] = true;
    comp.push(node);
    for &v in &adj[node] {
        if !visited[v] {
            scc_dfs2(adj, v, visited, comp);
        }
    }
}

pub fn articulation_points(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut disc = vec![0u64; n];
    let mut low = vec![0u64; n];
    let mut parent = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut ap = vec![false; n];
    let mut timer = 1u64;
    for i in 0..n {
        if !visited[i] {
            ap_dfs(
                adj,
                i,
                &mut disc,
                &mut low,
                &mut parent,
                &mut visited,
                &mut ap,
                &mut timer,
            );
        }
    }
    (0..n).filter(|&i| ap[i]).collect()
}

fn ap_dfs(
    adj: &[Vec<usize>],
    u: usize,
    disc: &mut [u64],
    low: &mut [u64],
    parent: &mut [usize],
    visited: &mut [bool],
    ap: &mut [bool],
    timer: &mut u64,
) {
    visited[u] = true;
    disc[u] = *timer;
    low[u] = *timer;
    *timer += 1;
    let mut children = 0u32;
    for &v in &adj[u] {
        if !visited[v] {
            children += 1;
            parent[v] = u;
            ap_dfs(adj, v, disc, low, parent, visited, ap, timer);
            if low[v] < low[u] {
                low[u] = low[v];
            }
            if parent[u] == usize::MAX && children > 1 {
                ap[u] = true;
            }
            if parent[u] != usize::MAX && low[v] >= disc[u] {
                ap[u] = true;
            }
        } else if v != parent[u] && disc[v] < low[u] {
            low[u] = disc[v];
        }
    }
}

pub fn bridges(adj: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let n = adj.len();
    let mut disc = vec![0u64; n];
    let mut low = vec![0u64; n];
    let mut parent = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut result = Vec::new();
    let mut timer = 1u64;
    for i in 0..n {
        if !visited[i] {
            bridge_dfs(
                adj,
                i,
                &mut disc,
                &mut low,
                &mut parent,
                &mut visited,
                &mut result,
                &mut timer,
            );
        }
    }
    result
}

fn bridge_dfs(
    adj: &[Vec<usize>],
    u: usize,
    disc: &mut [u64],
    low: &mut [u64],
    parent: &mut [usize],
    visited: &mut [bool],
    br: &mut Vec<(usize, usize)>,
    timer: &mut u64,
) {
    visited[u] = true;
    disc[u] = *timer;
    low[u] = *timer;
    *timer += 1;
    for &v in &adj[u] {
        if !visited[v] {
            parent[v] = u;
            bridge_dfs(adj, v, disc, low, parent, visited, br, timer);
            if low[v] < low[u] {
                low[u] = low[v];
            }
            if low[v] > disc[u] {
                br.push((u, v));
            }
        } else if v != parent[u] && disc[v] < low[u] {
            low[u] = disc[v];
        }
    }
}

pub fn graph_diameter(adj: &[Vec<usize>]) -> i64 {
    let n = adj.len();
    let mut diam = 0i64;
    for i in 0..n {
        let d = bfs_distance(adj, i);
        for &val in &d {
            if val > diam {
                diam = val;
            }
        }
    }
    diam
}

pub fn eccentricity(adj: &[Vec<usize>], node: usize) -> i64 {
    let d = bfs_distance(adj, node);
    let mut ecc = 0i64;
    for &val in &d {
        if val > ecc {
            ecc = val;
        }
    }
    ecc
}

pub fn graph_center(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let eccs: Vec<i64> = (0..n).map(|i| eccentricity(adj, i)).collect();
    let min_ecc = eccs.iter().copied().min().unwrap_or(0);
    eccs.iter()
        .enumerate()
        .filter(|&(_, &e)| e == min_ecc)
        .map(|(i, _)| i)
        .collect()
}

pub fn degree_sequence(adj: &[Vec<usize>]) -> Vec<usize> {
    let mut degrees: Vec<usize> = adj.iter().map(|neighbors| neighbors.len()).collect();
    degrees.sort_unstable();
    degrees.reverse();
    degrees
}

pub fn clustering_coefficient(adj: &[Vec<usize>], node: usize) -> f64 {
    let neighbors = &adj[node];
    let k = neighbors.len();
    if k < 2 {
        return 0.0;
    }
    let mut triangles = 0u64;
    let neighbor_set: std::collections::HashSet<usize> = neighbors.iter().copied().collect();
    for i in 0..k {
        for j in (i + 1)..k {
            if neighbor_set.contains(&neighbors[j]) && adj[neighbors[i]].contains(&neighbors[j]) {
                triangles += 1;
            }
        }
    }
    2.0 * triangles as f64 / (k * (k - 1)) as f64
}

pub fn pagerank(adj: &[Vec<usize>], damping: f64, iterations: usize) -> Vec<f64> {
    let n = adj.len();
    let mut rank = vec![1.0 / n as f64; n];
    let mut new_rank = vec![0.0; n];
    for _ in 0..iterations {
        for r in new_rank.iter_mut() {
            *r = (1.0 - damping) / n as f64;
        }
        for (u, neighbors) in adj.iter().enumerate() {
            if neighbors.is_empty() {
                continue;
            }
            let contrib = damping * rank[u] / neighbors.len() as f64;
            for &v in neighbors {
                new_rank[v] += contrib;
            }
        }
        std::mem::swap(&mut rank, &mut new_rank);
    }
    rank
}

pub fn bidirectional_bfs(adj: &[Vec<usize>], start: usize, end: usize) -> i64 {
    if start == end {
        return 0;
    }
    let n = adj.len();
    let mut dist_s = vec![-1i64; n];
    let mut dist_e = vec![-1i64; n];
    dist_s[start] = 0;
    dist_e[end] = 0;
    let mut q_s = std::collections::VecDeque::new();
    let mut q_e = std::collections::VecDeque::new();
    q_s.push_back(start);
    q_e.push_back(end);
    loop {
        if q_s.is_empty() && q_e.is_empty() {
            return -1;
        }
        if let Some(u) = q_s.pop_front() {
            for &v in &adj[u] {
                if dist_s[v] == -1 {
                    dist_s[v] = dist_s[u] + 1;
                    q_s.push_back(v);
                }
                if dist_e[v] >= 0 {
                    return dist_s[v] + dist_e[v];
                }
            }
        }
        if let Some(u) = q_e.pop_front() {
            for &v in &adj[u] {
                if dist_e[v] == -1 {
                    dist_e[v] = dist_e[u] + 1;
                    q_e.push_back(v);
                }
                if dist_s[v] >= 0 {
                    return dist_s[v] + dist_e[v];
                }
            }
        }
    }
}

pub fn has_cycle_undirected(adj: &[Vec<usize>]) -> bool {
    let n = adj.len();
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = vec![(start, usize::MAX)];
        while let Some((node, par)) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for &v in &adj[node] {
                if !visited[v] {
                    stack.push((v, node));
                } else if v != par {
                    return true;
                }
            }
        }
    }
    false
}
