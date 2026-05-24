pub fn kruskal(n: usize, edges: &mut [(usize, usize, f64)]) -> (f64, Vec<(usize, usize, f64)>) {
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut total = 0.0;
    for &(u, v, w) in edges.iter() {
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            mst.push((u, v, w));
            total += w;
        }
    }
    (total, mst)
}

pub fn prim(adj: &[Vec<(usize, f64)>]) -> (f64, Vec<(usize, usize)>) {
    let n = adj.len();
    let mut in_mst = vec![false; n];
    let mut key = vec![f64::INFINITY; n];
    let mut parent = vec![usize::MAX; n];
    key[0] = 0.0;
    let mut total = 0.0;
    let mut edges = Vec::new();
    for _ in 0..n {
        let mut u = usize::MAX;
        let mut min_k = f64::INFINITY;
        for i in 0..n {
            if !in_mst[i] && key[i] < min_k {
                min_k = key[i];
                u = i;
            }
        }
        if u == usize::MAX {
            break;
        }
        in_mst[u] = true;
        total += key[u];
        if parent[u] != usize::MAX {
            edges.push((parent[u], u));
        }
        for &(v, w) in &adj[u] {
            if !in_mst[v] && w < key[v] {
                key[v] = w;
                parent[v] = u;
            }
        }
    }
    (total, edges)
}

pub fn boruvka(n: usize, edges: &[(usize, usize, f64)]) -> (f64, Vec<(usize, usize, f64)>) {
    let mut uf = UnionFind::new(n);
    let mut mst = Vec::new();
    let mut total = 0.0;
    let mut components = n;
    while components > 1 {
        let mut cheapest = vec![usize::MAX; n];
        for (idx, &(u, v, w)) in edges.iter().enumerate() {
            let cu = uf.find(u);
            let cv = uf.find(v);
            if cu == cv {
                continue;
            }
            if cheapest[cu] == usize::MAX || w < edges[cheapest[cu]].2 {
                cheapest[cu] = idx;
            }
            if cheapest[cv] == usize::MAX || w < edges[cheapest[cv]].2 {
                cheapest[cv] = idx;
            }
        }
        let mut added = false;
        for i in 0..n {
            if cheapest[i] == usize::MAX {
                continue;
            }
            let (u, v, w) = edges[cheapest[i]];
            if uf.find(u) != uf.find(v) {
                uf.union(u, v);
                mst.push((u, v, w));
                total += w;
                components -= 1;
                added = true;
            }
        }
        if !added {
            break;
        }
    }
    (total, mst)
}

pub fn is_tree(adj: &[Vec<usize>]) -> bool {
    let n = adj.len();
    if n == 0 {
        return true;
    }
    let mut edge_count = 0;
    for neighbors in adj {
        edge_count += neighbors.len();
    }
    if edge_count / 2 != n - 1 {
        return false;
    }
    let mut visited = vec![false; n];
    let mut stack = vec![0];
    let mut count = 0;
    while let Some(node) = stack.pop() {
        if visited[node] {
            continue;
        }
        visited[node] = true;
        count += 1;
        for &v in &adj[node] {
            if !visited[v] {
                stack.push(v);
            }
        }
    }
    count == n
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry {
            return;
        }
        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }
    }
    fn connected(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

pub fn mst_weight(n: usize, edges: &mut [(usize, usize, f64)]) -> f64 {
    let (w, _) = kruskal(n, edges);
    w
}

pub fn is_connected_weighted(n: usize, edges: &[(usize, usize, f64)]) -> bool {
    let mut uf = UnionFind::new(n);
    for &(u, v, _) in edges {
        uf.union(u, v);
    }
    let root = uf.find(0);
    (1..n).all(|i| uf.find(i) == root)
}

pub fn bottleneck_mst_edge(n: usize, edges: &mut [(usize, usize, f64)]) -> f64 {
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    let mut uf = UnionFind::new(n);
    let mut max_edge = 0.0f64;
    for &(u, v, w) in edges.iter() {
        if !uf.connected(u, v) {
            uf.union(u, v);
            if w > max_edge {
                max_edge = w;
            }
        }
    }
    max_edge
}

pub fn second_best_mst(n: usize, edges: &mut [(usize, usize, f64)]) -> f64 {
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
    let mut uf = UnionFind::new(n);
    let mut mst_edges = Vec::new();
    let mut mst_set = std::collections::HashSet::new();
    for (idx, &(u, v, w)) in edges.iter().enumerate() {
        if uf.find(u) != uf.find(v) {
            uf.union(u, v);
            mst_edges.push((u, v, w));
            mst_set.insert(idx);
        }
    }
    let mst_weight: f64 = mst_edges.iter().map(|e| e.2).sum();
    let mut best = f64::INFINITY;
    for &skip_idx in &mst_set {
        let mut uf2 = UnionFind::new(n);
        let mut total = 0.0;
        let mut count = 0;
        for (idx, &(u, v, w)) in edges.iter().enumerate() {
            if idx == skip_idx {
                continue;
            }
            if uf2.find(u) != uf2.find(v) {
                uf2.union(u, v);
                total += w;
                count += 1;
            }
        }
        if count == n - 1 && total < best {
            best = total;
        }
    }
    let _ = mst_weight;
    best
}

pub fn mst_edges_count(n: usize, edges: &mut [(usize, usize, f64)]) -> usize {
    let (_, mst) = kruskal(n, edges);
    mst.len()
}

pub fn steiner_tree_approx(adj: &[Vec<(usize, f64)>], terminals: &[usize]) -> f64 {
    if terminals.len() <= 1 {
        return 0.0;
    }
    let n = adj.len();
    let mut in_tree = vec![false; n];
    in_tree[terminals[0]] = true;
    let mut total_cost = 0.0;
    let mut remaining: Vec<usize> = terminals[1..].to_vec();
    while !remaining.is_empty() {
        let mut best_cost = f64::INFINITY;
        let mut best_idx = 0;
        let mut best_path_cost = 0.0;
        for (idx, &t) in remaining.iter().enumerate() {
            let dist = dijkstra_for_steiner(adj, t);
            for i in 0..n {
                if in_tree[i] && dist[i] < best_cost {
                    best_cost = dist[i];
                    best_idx = idx;
                    best_path_cost = dist[i];
                }
            }
        }
        let target = remaining.swap_remove(best_idx);
        in_tree[target] = true;
        total_cost += best_path_cost;
    }
    total_cost
}

fn dijkstra_for_steiner(adj: &[Vec<(usize, f64)>], start: usize) -> Vec<f64> {
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

pub fn kirchhoff_spanning_tree_count(adj: &[Vec<usize>]) -> f64 {
    let n = adj.len();
    if n <= 1 {
        return 1.0;
    }
    let mut laplacian = vec![vec![0.0f64; n]; n];
    for (i, neighbors) in adj.iter().enumerate() {
        laplacian[i][i] = neighbors.len() as f64;
        for &j in neighbors {
            laplacian[i][j] -= 1.0;
        }
    }
    let m = n - 1;
    let mut sub = vec![vec![0.0; m]; m];
    for i in 0..m {
        for j in 0..m {
            sub[i][j] = laplacian[i][j];
        }
    }
    det_gauss(&mut sub)
}

fn det_gauss(mat: &mut [Vec<f64>]) -> f64 {
    let n = mat.len();
    let mut sign = 1.0;
    for col in 0..n {
        let mut pivot = col;
        for row in (col + 1)..n {
            if mat[row][col].abs() > mat[pivot][col].abs() {
                pivot = row;
            }
        }
        if pivot != col {
            mat.swap(col, pivot);
            sign = -sign;
        }
        if mat[col][col].abs() < 1e-12 {
            return 0.0;
        }
        for row in (col + 1)..n {
            let factor = mat[row][col] / mat[col][col];
            let (top, bot) = mat.split_at_mut(row);
            for (d, &s) in bot[0][col..n].iter_mut().zip(&top[col][col..n]) {
                *d -= factor * s;
            }
        }
    }
    let mut d = sign;
    for (i, mi) in mat.iter().enumerate() {
        d *= mi[i];
    }
    d.abs()
}

pub fn tree_diameter_weighted(adj: &[Vec<(usize, f64)>]) -> f64 {
    let n = adj.len();
    if n == 0 {
        return 0.0;
    }
    let d1 = bfs_farthest_weighted(adj, 0);
    let far1 = d1
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(i, _)| i)
        .unwrap_or(0);
    let d2 = bfs_farthest_weighted(adj, far1);
    d2.iter().cloned().fold(0.0f64, f64::max)
}

fn bfs_farthest_weighted(adj: &[Vec<(usize, f64)>], start: usize) -> Vec<f64> {
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
