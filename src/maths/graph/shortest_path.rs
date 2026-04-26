pub fn dijkstra(adj: &[Vec<(usize, f64)>], start: usize) -> Vec<f64> {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    let mut visited = vec![false; n];
    dist[start] = 0.0;
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut min_d = f64::INFINITY;
        for i in 0..n {
            if !visited[i] && dist[i] < min_d {
                min_d = dist[i];
                u = i;
            }
        }
        if u == usize::MAX {
            break;
        }
        visited[u] = true;
        for &(v, w) in &adj[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    dist
}

pub fn bellman_ford(edges: &[(usize, usize, f64)], n: usize, start: usize) -> Option<Vec<f64>> {
    let mut dist = vec![f64::INFINITY; n];
    dist[start] = 0.0;
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    for &(u, v, w) in edges {
        if dist[u] + w < dist[v] {
            return None;
        }
    }
    Some(dist)
}

pub fn floyd_warshall(dist: &mut [Vec<f64>]) {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] + dist[k][j] < dist[i][j] {
                    dist[i][j] = dist[i][k] + dist[k][j];
                }
            }
        }
    }
}

pub fn dijkstra_path(adj: &[Vec<(usize, f64)>], start: usize, end: usize) -> (f64, Vec<usize>) {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    let mut prev = vec![usize::MAX; n];
    let mut visited = vec![false; n];
    dist[start] = 0.0;
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut min_d = f64::INFINITY;
        for i in 0..n {
            if !visited[i] && dist[i] < min_d {
                min_d = dist[i];
                u = i;
            }
        }
        if u == usize::MAX || u == end {
            break;
        }
        visited[u] = true;
        for &(v, w) in &adj[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                prev[v] = u;
            }
        }
    }
    let mut path = Vec::new();
    let mut cur = end;
    while cur != usize::MAX {
        path.push(cur);
        cur = prev[cur];
    }
    path.reverse();
    (dist[end], path)
}

pub fn a_star(
    adj: &[Vec<(usize, f64)>],
    start: usize,
    goal: usize,
    heuristic: fn(usize, usize) -> f64,
) -> (f64, Vec<usize>) {
    let n = adj.len();
    let mut g_score = vec![f64::INFINITY; n];
    let mut f_score = vec![f64::INFINITY; n];
    let mut prev = vec![usize::MAX; n];
    let mut closed = vec![false; n];
    g_score[start] = 0.0;
    f_score[start] = heuristic(start, goal);
    loop {
        let mut current = usize::MAX;
        let mut min_f = f64::INFINITY;
        for i in 0..n {
            if !closed[i] && f_score[i] < min_f {
                min_f = f_score[i];
                current = i;
            }
        }
        if current == usize::MAX || current == goal {
            break;
        }
        closed[current] = true;
        for &(neighbor, w) in &adj[current] {
            let tentative = g_score[current] + w;
            if tentative < g_score[neighbor] {
                prev[neighbor] = current;
                g_score[neighbor] = tentative;
                f_score[neighbor] = tentative + heuristic(neighbor, goal);
            }
        }
    }
    let mut path = Vec::new();
    let mut cur = goal;
    while cur != usize::MAX {
        path.push(cur);
        cur = prev[cur];
    }
    path.reverse();
    (g_score[goal], path)
}

pub fn johnson(adj: &[Vec<(usize, f64)>]) -> Option<Vec<Vec<f64>>> {
    let n = adj.len();
    let mut edges = Vec::new();
    for (u, neighbors) in adj.iter().enumerate() {
        for &(v, w) in neighbors {
            edges.push((u, v, w));
        }
    }
    for v in 0..n {
        edges.push((n, v, 0.0));
    }
    let h = bellman_ford_internal(&edges, n + 1, n)?;
    let mut result = vec![vec![f64::INFINITY; n]; n];
    for u in 0..n {
        let reweighted: Vec<Vec<(usize, f64)>> = (0..n)
            .map(|i| adj[i].iter().map(|&(v, w)| (v, w + h[i] - h[v])).collect())
            .collect();
        let d = dijkstra_internal(&reweighted, u);
        for v in 0..n {
            result[u][v] = d[v] - h[u] + h[v];
        }
    }
    Some(result)
}

fn bellman_ford_internal(
    edges: &[(usize, usize, f64)],
    n: usize,
    start: usize,
) -> Option<Vec<f64>> {
    let mut dist = vec![f64::INFINITY; n];
    dist[start] = 0.0;
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    for &(u, v, w) in edges {
        if dist[u] + w < dist[v] {
            return None;
        }
    }
    Some(dist)
}

fn dijkstra_internal(adj: &[Vec<(usize, f64)>], start: usize) -> Vec<f64> {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    let mut visited = vec![false; n];
    dist[start] = 0.0;
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut min_d = f64::INFINITY;
        for i in 0..n {
            if !visited[i] && dist[i] < min_d {
                min_d = dist[i];
                u = i;
            }
        }
        if u == usize::MAX {
            break;
        }
        visited[u] = true;
        for &(v, w) in &adj[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    dist
}

pub fn spfa(adj: &[Vec<(usize, f64)>], start: usize) -> Option<Vec<f64>> {
    let n = adj.len();
    let mut dist = vec![f64::INFINITY; n];
    let mut in_queue = vec![false; n];
    let mut count = vec![0usize; n];
    dist[start] = 0.0;
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);
    in_queue[start] = true;
    while let Some(u) = queue.pop_front() {
        in_queue[u] = false;
        for &(v, w) in &adj[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                count[v] += 1;
                if count[v] >= n {
                    return None;
                }
                if !in_queue[v] {
                    queue.push_back(v);
                    in_queue[v] = true;
                }
            }
        }
    }
    Some(dist)
}

pub fn dag_shortest_path(adj: &[Vec<(usize, f64)>], start: usize) -> Vec<f64> {
    let n = adj.len();
    let plain: Vec<Vec<usize>> = adj
        .iter()
        .map(|neighbors| neighbors.iter().map(|&(v, _)| v).collect())
        .collect();
    let topo = topo_sort_internal(&plain);
    let mut dist = vec![f64::INFINITY; n];
    dist[start] = 0.0;
    for &u in &topo {
        if dist[u] == f64::INFINITY {
            continue;
        }
        for &(v, w) in &adj[u] {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    dist
}

fn topo_sort_internal(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut in_deg = vec![0usize; n];
    for edges in adj {
        for &v in edges {
            in_deg[v] += 1;
        }
    }
    let mut q: std::collections::VecDeque<usize> = in_deg
        .iter()
        .enumerate()
        .filter(|&(_, &d)| d == 0)
        .map(|(i, _)| i)
        .collect();
    let mut order = Vec::with_capacity(n);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    order
}

pub fn k_shortest_paths(
    adj: &[Vec<(usize, f64)>],
    start: usize,
    end: usize,
    k: usize,
) -> Vec<(f64, Vec<usize>)> {
    let mut results = Vec::new();
    let mut counts = vec![0usize; adj.len()];
    let mut heap: Vec<(f64, usize, Vec<usize>)> = vec![(0.0, start, vec![start])];
    while let Some(idx) = heap
        .iter()
        .enumerate()
        .min_by(|a, b| {
            a.1.0
                .partial_cmp(&b.1.0)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
    {
        let (cost, u, path) = heap.swap_remove(idx);
        counts[u] += 1;
        if u == end {
            results.push((cost, path.clone()));
            if results.len() == k {
                return results;
            }
        }
        if counts[u] > k {
            continue;
        }
        for &(v, w) in &adj[u] {
            let mut new_path = path.clone();
            new_path.push(v);
            heap.push((cost + w, v, new_path));
        }
    }
    results
}

pub fn closeness_centrality(adj: &[Vec<(usize, f64)>], node: usize) -> f64 {
    let dist = dijkstra_internal(adj, node);
    let n = adj.len();
    let mut total = 0.0;
    let mut reachable = 0usize;
    for (i, &d) in dist.iter().enumerate() {
        if i != node && d < f64::INFINITY {
            total += d;
            reachable += 1;
        }
    }
    if reachable == 0 || total == 0.0 || n <= 1 {
        return 0.0;
    }
    let norm = reachable as f64 / (n - 1) as f64;
    norm * reachable as f64 / total
}

pub fn betweenness_centrality(adj: &[Vec<(usize, f64)>]) -> Vec<f64> {
    let n = adj.len();
    let mut cb = vec![0.0; n];
    for s in 0..n {
        let mut stack = Vec::new();
        let mut pred: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut sigma = vec![0.0f64; n];
        let mut dist = vec![f64::INFINITY; n];
        sigma[s] = 1.0;
        dist[s] = 0.0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            stack.push(u);
            for &(v, w) in &adj[u] {
                let new_d = dist[u] + w;
                if new_d < dist[v] {
                    dist[v] = new_d;
                    sigma[v] = 0.0;
                    pred[v].clear();
                    queue.push_back(v);
                }
                if (new_d - dist[v]).abs() < 1e-12 {
                    sigma[v] += sigma[u];
                    pred[v].push(u);
                }
            }
        }
        let mut delta = vec![0.0; n];
        while let Some(w) = stack.pop() {
            for &v in &pred[w] {
                delta[v] += (sigma[v] / sigma[w]) * (1.0 + delta[w]);
            }
            if w != s {
                cb[w] += delta[w];
            }
        }
    }
    for val in cb.iter_mut() {
        *val /= 2.0;
    }
    cb
}

pub fn all_pairs_unweighted(adj: &[Vec<usize>]) -> Vec<Vec<i64>> {
    let n = adj.len();
    (0..n)
        .map(|i| {
            let mut dist = vec![-1i64; n];
            dist[i] = 0;
            let mut q = std::collections::VecDeque::new();
            q.push_back(i);
            while let Some(u) = q.pop_front() {
                for &v in &adj[u] {
                    if dist[v] == -1 {
                        dist[v] = dist[u] + 1;
                        q.push_back(v);
                    }
                }
            }
            dist
        })
        .collect()
}

pub fn diameter_weighted(adj: &[Vec<(usize, f64)>]) -> f64 {
    let n = adj.len();
    let mut diam = 0.0f64;
    for i in 0..n {
        let d = dijkstra_internal(adj, i);
        for &val in &d {
            if val < f64::INFINITY && val > diam {
                diam = val;
            }
        }
    }
    diam
}

pub fn negative_cycle_exists(edges: &[(usize, usize, f64)], n: usize) -> bool {
    let mut dist = vec![0.0; n];
    for _ in 0..n - 1 {
        for &(u, v, w) in edges {
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }
    for &(u, v, w) in edges {
        if dist[u] + w < dist[v] {
            return true;
        }
    }
    false
}
