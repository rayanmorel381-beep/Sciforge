//! Dispatch handler for graph theory functions.

use super::super::params::*;
use crate::hub::domain::common::errors::{HubError, HubResult};
use crate::hub::domain::maths;
use crate::hub::engine::experience::runner::RunOutput;

pub(super) fn dispatch(func: &str, p: &Params) -> HubResult<RunOutput> {
    match func {
        "bfs" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::bfs(adj, get_u(p, "start")?)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "dfs" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::dfs(adj, get_u(p, "start")?)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "topological_sort" => {
            let adj = get_um(p, "adj")?;
            match maths::graph::topological_sort(adj) {
                Some(v) => Ok(RunOutput::Vector(v.into_iter().map(|x| x as f64).collect())),
                None => Err(HubError::ComputationFailed("cycle detected".into())),
            }
        }
        "connected_components" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::connected_components(adj)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "is_bipartite" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Boolean(maths::graph::is_bipartite(adj)))
        }
        "has_cycle_directed" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Boolean(maths::graph::has_cycle_directed(adj)))
        }
        "bfs_distance" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::IntVector(maths::graph::bfs_distance(
                adj,
                get_u(p, "start")?,
            )))
        }
        "iterative_deepening_dfs" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::iterative_deepening_dfs(
                    adj,
                    get_u(p, "start")?,
                    get_u(p, "max_depth")?,
                )
                .into_iter()
                .map(|x| x as f64)
                .collect(),
            ))
        }
        "strongly_connected_components" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Matrix(
                maths::graph::strongly_connected_components(adj)
                    .into_iter()
                    .map(|c| c.into_iter().map(|x| x as f64).collect())
                    .collect(),
            ))
        }
        "articulation_points" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::articulation_points(adj)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "bridges" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::PairVec(
                maths::graph::bridges(adj)
                    .into_iter()
                    .map(|(a, b)| (a as f64, b as f64))
                    .collect(),
            ))
        }
        "graph_diameter" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Integer(maths::graph::graph_diameter(adj) as i64))
        }
        "eccentricity" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Integer(
                maths::graph::eccentricity(adj, get_u(p, "node")?) as i64,
            ))
        }
        "graph_center" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::graph_center(adj)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "degree_sequence" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(
                maths::graph::degree_sequence(adj)
                    .into_iter()
                    .map(|x| x as f64)
                    .collect(),
            ))
        }
        "clustering_coefficient" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Scalar(maths::graph::clustering_coefficient(
                adj,
                get_u(p, "node")?,
            )))
        }
        "pagerank" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Vector(maths::graph::pagerank(
                adj,
                get_f(p, "damping")?,
                get_u(p, "iterations")?,
            )))
        }
        "bidirectional_bfs" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Integer(maths::graph::bidirectional_bfs(
                adj,
                get_u(p, "start")?,
                get_u(p, "end")?,
            ) as i64))
        }
        "has_cycle_undirected" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Boolean(maths::graph::has_cycle_undirected(adj)))
        }
        "dijkstra" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Vector(maths::graph::dijkstra(
                &wadj,
                get_u(p, "start")?,
            )))
        }
        "bellman_ford" => {
            let e = get_edges(p, "edges")?;
            match maths::graph::bellman_ford(e, get_u(p, "n")?, get_u(p, "start")?) {
                Some(v) => Ok(RunOutput::Vector(v)),
                None => Err(HubError::ComputationFailed("negative cycle".into())),
            }
        }
        "floyd_warshall" => {
            let mut dist = get_m(p, "dist")?.to_vec();
            maths::graph::floyd_warshall(&mut dist);
            Ok(RunOutput::Matrix(dist))
        }
        "dijkstra_path" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            let (cost, path) =
                maths::graph::dijkstra_path(&wadj, get_u(p, "start")?, get_u(p, "end")?);
            Ok(RunOutput::Matrix(vec![
                vec![cost],
                path.into_iter().map(|x| x as f64).collect(),
            ]))
        }
        "johnson" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            match maths::graph::johnson(&wadj) {
                Some(m) => Ok(RunOutput::Matrix(m)),
                None => Err(HubError::ComputationFailed("negative cycle".into())),
            }
        }
        "spfa" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            match maths::graph::spfa(&wadj, get_u(p, "start")?) {
                Some(v) => Ok(RunOutput::Vector(v)),
                None => Err(HubError::ComputationFailed("negative cycle".into())),
            }
        }
        "dag_shortest_path" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Vector(maths::graph::dag_shortest_path(
                &wadj,
                get_u(p, "start")?,
            )))
        }
        "k_shortest_paths" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            let paths = maths::graph::k_shortest_paths(
                &wadj,
                get_u(p, "start")?,
                get_u(p, "end")?,
                get_u(p, "k")?,
            );
            Ok(RunOutput::Matrix(
                paths
                    .into_iter()
                    .map(|(cost, path)| {
                        let mut row = vec![cost];
                        row.extend(path.into_iter().map(|x| x as f64));
                        row
                    })
                    .collect(),
            ))
        }
        "closeness_centrality" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(maths::graph::closeness_centrality(
                &wadj,
                get_u(p, "node")?,
            )))
        }
        "betweenness_centrality" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Vector(maths::graph::betweenness_centrality(
                &wadj,
            )))
        }
        "all_pairs_unweighted" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::IntVector(
                maths::graph::all_pairs_unweighted(adj)
                    .into_iter()
                    .flatten()
                    .collect(),
            ))
        }
        "diameter_weighted" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(maths::graph::diameter_weighted(&wadj)))
        }
        "negative_cycle_exists" => {
            let e = get_edges(p, "edges")?;
            Ok(RunOutput::Boolean(maths::graph::negative_cycle_exists(
                e,
                get_u(p, "n")?,
            )))
        }
        "kruskal" => {
            let mut e: Vec<(usize, usize, f64)> = get_edges(p, "edges")?.to_vec();
            let (w, tree) = maths::graph::kruskal(get_u(p, "n")?, &mut e);
            Ok(RunOutput::Matrix(vec![
                vec![w],
                tree.into_iter()
                    .flat_map(|(a, b, c)| vec![a as f64, b as f64, c])
                    .collect::<Vec<_>>(),
            ]))
        }
        "prim" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            let (w, edges) = maths::graph::prim(&wadj);
            Ok(RunOutput::Matrix(vec![
                vec![w],
                edges
                    .into_iter()
                    .flat_map(|(a, b)| vec![a as f64, b as f64])
                    .collect(),
            ]))
        }
        "boruvka" => {
            let e = get_edges(p, "edges")?;
            let (w, tree) = maths::graph::boruvka(get_u(p, "n")?, e);
            Ok(RunOutput::Matrix(vec![
                vec![w],
                tree.into_iter()
                    .flat_map(|(a, b, c)| vec![a as f64, b as f64, c])
                    .collect::<Vec<_>>(),
            ]))
        }
        "is_tree" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Boolean(maths::graph::is_tree(adj)))
        }
        "mst_weight" => {
            let mut e: Vec<(usize, usize, f64)> = get_edges(p, "edges")?.to_vec();
            Ok(RunOutput::Scalar(maths::graph::mst_weight(
                get_u(p, "n")?,
                &mut e,
            )))
        }
        "is_connected_weighted" => {
            let e = get_edges(p, "edges")?;
            Ok(RunOutput::Boolean(maths::graph::is_connected_weighted(
                get_u(p, "n")?,
                e,
            )))
        }
        "bottleneck_mst_edge" => {
            let mut e: Vec<(usize, usize, f64)> = get_edges(p, "edges")?.to_vec();
            Ok(RunOutput::Scalar(maths::graph::bottleneck_mst_edge(
                get_u(p, "n")?,
                &mut e,
            )))
        }
        "second_best_mst" => {
            let mut e: Vec<(usize, usize, f64)> = get_edges(p, "edges")?.to_vec();
            Ok(RunOutput::Scalar(maths::graph::second_best_mst(
                get_u(p, "n")?,
                &mut e,
            )))
        }
        "mst_edges_count" => {
            let mut e: Vec<(usize, usize, f64)> = get_edges(p, "edges")?.to_vec();
            Ok(RunOutput::Integer(
                maths::graph::mst_edges_count(get_u(p, "n")?, &mut e) as i64,
            ))
        }
        "steiner_tree_approx" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(maths::graph::steiner_tree_approx(
                &wadj,
                get_uv(p, "terminals")?,
            )))
        }
        "kirchhoff_spanning_tree_count" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Scalar(
                maths::graph::kirchhoff_spanning_tree_count(adj),
            ))
        }
        "tree_diameter_weighted" => {
            let wadj: Vec<Vec<(usize, f64)>> = get_m(p, "adj")?
                .iter()
                .map(|row| row.chunks(2).map(|c| (c[0] as usize, c[1])).collect())
                .collect();
            Ok(RunOutput::Scalar(maths::graph::tree_diameter_weighted(
                &wadj,
            )))
        }
        "ford_fulkerson" => Ok(RunOutput::Scalar(maths::graph::ford_fulkerson(
            get_m(p, "capacity")?,
            get_u(p, "source")?,
            get_u(p, "sink")?,
        ))),
        "min_cut" => Ok(RunOutput::PairVec(
            maths::graph::min_cut(
                get_m(p, "capacity")?,
                get_u(p, "source")?,
                get_u(p, "sink")?,
            )
            .into_iter()
            .map(|(a, b)| (a as f64, b as f64))
            .collect(),
        )),
        "bipartite_matching" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::PairVec(
                maths::graph::bipartite_matching(adj, get_u(p, "n_left")?, get_u(p, "n_right")?)
                    .into_iter()
                    .map(|(a, b)| (a as f64, b as f64))
                    .collect(),
            ))
        }
        "min_cost_flow" => {
            let (flow, cost) = maths::graph::min_cost_flow(
                get_m(p, "capacity")?,
                get_m(p, "cost")?,
                get_u(p, "source")?,
                get_u(p, "sink")?,
                get_f(p, "max_flow_limit")?,
            );
            Ok(RunOutput::Pair(flow, cost))
        }
        "edmonds_karp" => Ok(RunOutput::Scalar(maths::graph::edmonds_karp(
            get_m(p, "capacity")?,
            get_u(p, "source")?,
            get_u(p, "sink")?,
        ))),
        "edge_connectivity" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Scalar(maths::graph::edge_connectivity(adj)))
        }
        "vertex_connectivity" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Scalar(maths::graph::vertex_connectivity(adj)))
        }
        "max_flow_scaling" => Ok(RunOutput::Scalar(maths::graph::max_flow_scaling(
            get_m(p, "capacity")?,
            get_u(p, "source")?,
            get_u(p, "sink")?,
        ))),
        "multi_source_multi_sink_flow" => Ok(RunOutput::Scalar(
            maths::graph::multi_source_multi_sink_flow(
                get_m(p, "capacity")?,
                get_uv(p, "sources")?,
                get_uv(p, "sinks")?,
            ),
        )),
        "circulation_demand" => Ok(RunOutput::Boolean(maths::graph::circulation_demand(
            get_m(p, "capacity")?,
            get_v(p, "demand")?,
        ))),
        "residual_graph" => Ok(RunOutput::Matrix(maths::graph::residual_graph(
            get_m(p, "capacity")?,
            get_m(p, "flow")?,
        ))),
        "matching_size" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Integer(maths::graph::matching_size(
                adj,
                get_u(p, "n_left")?,
                get_u(p, "n_right")?,
            ) as i64))
        }
        "is_perfect_matching" => {
            let adj = get_um(p, "adj")?;
            Ok(RunOutput::Boolean(maths::graph::is_perfect_matching(
                adj,
                get_u(p, "n_left")?,
                get_u(p, "n_right")?,
            )))
        }

        _ => Err(HubError::InvalidInput(format!("unknown function: {func}"))),
    }
}
