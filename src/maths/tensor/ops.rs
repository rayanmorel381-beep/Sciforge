use super::storage::Tensor;

pub fn reshape(t: &Tensor, new_shape: &[usize]) -> Tensor {
    assert_eq!(new_shape.iter().product::<usize>(), t.data.len());
    Tensor::from_vec(new_shape, t.data.clone())
}

pub fn transpose(t: &Tensor, axes: &[usize]) -> Tensor {
    assert_eq!(axes.len(), t.rank());
    let new_shape: Vec<usize> = axes.iter().map(|&a| t.shape()[a]).collect();
    let new_strides = Tensor::compute_strides(&new_shape);
    let size = t.size();
    let mut new_data = vec![0.0; size];
    let mut indices = vec![0usize; t.rank()];
    for _ in 0..size {
        let new_indices: Vec<usize> = axes.iter().map(|&a| indices[a]).collect();
        let new_flat: usize = new_indices
            .iter()
            .zip(&new_strides)
            .map(|(i, s)| i * s)
            .sum();
        new_data[new_flat] = t.get(&indices);
        for k in (0..t.rank()).rev() {
            indices[k] += 1;
            if indices[k] < t.shape()[k] {
                break;
            }
            indices[k] = 0;
        }
    }
    Tensor {
        data: new_data,
        shape: new_shape,
        strides: new_strides,
    }
}

pub fn contract(a: &Tensor, b: &Tensor, axis_a: usize, axis_b: usize) -> Tensor {
    assert_eq!(a.shape()[axis_a], b.shape()[axis_b]);
    let contract_size = a.shape()[axis_a];
    let mut result_shape = Vec::new();
    for (i, &s) in a.shape().iter().enumerate() {
        if i != axis_a {
            result_shape.push(s);
        }
    }
    for (i, &s) in b.shape().iter().enumerate() {
        if i != axis_b {
            result_shape.push(s);
        }
    }
    let a_rank = a.rank();
    let b_rank = b.rank();
    Tensor::from_fn(&result_shape, |indices| {
        let split = a_rank - 1;
        let idx_a = &indices[..split];
        let idx_b = &indices[split..];
        let mut sum = 0.0;
        for k in 0..contract_size {
            let mut full_a = Vec::with_capacity(a_rank);
            let mut ai = 0;
            for i in 0..a_rank {
                if i == axis_a {
                    full_a.push(k);
                } else {
                    full_a.push(idx_a[ai]);
                    ai += 1;
                }
            }
            let mut full_b = Vec::with_capacity(b_rank);
            let mut bi = 0;
            for i in 0..b_rank {
                if i == axis_b {
                    full_b.push(k);
                } else {
                    full_b.push(idx_b[bi]);
                    bi += 1;
                }
            }
            sum += a.get(&full_a) * b.get(&full_b);
        }
        sum
    })
}

pub fn outer(a: &Tensor, b: &Tensor) -> Tensor {
    let mut new_shape = a.shape().to_vec();
    new_shape.extend_from_slice(b.shape());
    let a_rank = a.rank();
    Tensor::from_fn(&new_shape, |indices| {
        let (ia, ib) = indices.split_at(a_rank);
        a.get(ia) * b.get(ib)
    })
}

pub fn kronecker(a: &Tensor, b: &Tensor) -> Tensor {
    assert!(a.rank() == 2 && b.rank() == 2);
    let (am, an) = (a.shape()[0], a.shape()[1]);
    let (bm, bn) = (b.shape()[0], b.shape()[1]);
    Tensor::from_fn(&[am * bm, an * bn], |idx| {
        a.get(&[idx[0] / bm, idx[1] / bn]) * b.get(&[idx[0] % bm, idx[1] % bn])
    })
}

pub fn levi_civita(n: usize) -> Tensor {
    let shape: Vec<usize> = vec![n; n];
    Tensor::from_fn(&shape, |indices| {
        for i in 0..n {
            for j in (i + 1)..n {
                if indices[i] == indices[j] {
                    return 0.0;
                }
            }
        }
        let mut parity = 0;
        let mut perm: Vec<usize> = indices.to_vec();
        for i in 0..n {
            while perm[i] != i {
                let target = perm[i];
                perm.swap(i, target);
                parity += 1;
            }
        }
        if parity % 2 == 0 { 1.0 } else { -1.0 }
    })
}

pub fn metric_raise(t: &Tensor, metric_inv: &Tensor, index: usize) -> Tensor {
    contract(t, metric_inv, index, 1)
}

pub fn metric_lower(t: &Tensor, metric: &Tensor, index: usize) -> Tensor {
    contract(t, metric, index, 0)
}
