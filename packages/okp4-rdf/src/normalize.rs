use itertools::Itertools;
use rio_api::model::{BlankNode, GraphName, Quad, Subject, Term};
use sha2;
use sha2::Digest;
use std::collections::hash_map::{Entry, Iter};
use std::collections::{BTreeMap, HashMap};
use thiserror::Error;

/// A RDF normalizer allowing to canonicalize RDF data, following the https://www.w3.org/TR/rdf-canon specification.
#[derive(Eq, PartialEq, Debug)]
pub struct Normalizer<'a> {
    blank_node_to_quads: HashMap<String, Vec<Quad<'a>>>,
    hash_to_blank_nodes: BTreeMap<String, Vec<String>>,
    blank_node_to_hash: HashMap<String, String>,
    canonical_issuer: IdentifierIssuer,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NormalizationError {
    /// An unexpected error denotes an error that should never occur.  
    #[error("An unexpected error occurred: {0}")]
    Unexpected(String),
}

impl<'a> Normalizer<'a> {
    const CANONICAL_BLANK_NODES_IDENTIFIER_PREFIX: &'static str = "c14n";
    const TEMPORARY_BLANK_NODES_IDENTIFIER_PREFIX: &'static str = "b";

    const HASH_FIRST_DEGREE_MARKER_SELF: &'static str = "a";
    const HASH_FIRST_DEGREE_MARKER_OTHER: &'static str = "z";

    const HASH_RELATED_BLANK_NODE_POSITION_S: &'static str = "s";
    const HASH_RELATED_BLANK_NODE_POSITION_O: &'static str = "o";
    const HASH_RELATED_BLANK_NODE_POSITION_G: &'static str = "g";

    pub fn new() -> Self {
        Normalizer {
            blank_node_to_quads: HashMap::new(),
            hash_to_blank_nodes: BTreeMap::new(),
            blank_node_to_hash: HashMap::new(),
            canonical_issuer: IdentifierIssuer::new(
                Self::CANONICAL_BLANK_NODES_IDENTIFIER_PREFIX,
                0u128,
            ),
        }
    }

    pub fn normalize(&mut self, dataset: &[Quad<'a>]) -> Result<String, NormalizationError> {
        self.reset();
        self.track_blank_nodes(dataset);
        self.compute_first_degree_hashes();
        self.label_unique_nodes()?;
        self.compute_n_degree_hashes()?;

        let swap_fn = |n| {
            self.canonical_issuer.get(n).ok_or_else(|| {
                NormalizationError::Unexpected(
                    "Could not replace blank node, canonical identifier not found".to_string(),
                )
            })
        };
        let mut canonicalized_dataset = dataset.to_vec();
        for quad in canonicalized_dataset.iter_mut() {
            quad.try_swap_blank_nodes(&swap_fn)?;
        }

        Ok(Self::serialize(&canonicalized_dataset))
    }

    fn reset(&mut self) {
        self.blank_node_to_quads = HashMap::new();
        self.hash_to_blank_nodes = BTreeMap::new();
        self.blank_node_to_hash = HashMap::new();
        self.canonical_issuer =
            IdentifierIssuer::new(Self::CANONICAL_BLANK_NODES_IDENTIFIER_PREFIX, 0u128);
    }

    fn track_blank_nodes(&mut self, dataset: &[Quad<'a>]) {
        for quad in dataset {
            for node in quad.blank_nodes() {
                self.blank_node_to_quads
                    .entry(node)
                    .and_modify(|e| e.push(*quad))
                    .or_insert(vec![*quad]);
            }
        }
    }

    fn compute_first_degree_hashes(&mut self) {
        for (target, quads) in &self.blank_node_to_quads {
            let mut replacements = quads.clone();
            let swap_fn = |n| {
                if n == target {
                    return Self::HASH_FIRST_DEGREE_MARKER_SELF;
                }
                Self::HASH_FIRST_DEGREE_MARKER_OTHER
            };
            replacements.iter_mut().for_each(|quad| {
                quad.swap_blank_nodes(&swap_fn);
            });

            let hash = Self::hash(Self::serialize(&replacements));
            self.hash_to_blank_nodes
                .entry(hash.clone())
                .and_modify(|v| v.push(target.clone()))
                .or_insert(vec![target.clone()]);
            self.blank_node_to_hash.insert(target.clone(), hash);
        }
    }

    fn label_unique_nodes(&mut self) -> Result<(), NormalizationError> {
        let unique_nodes = self
            .hash_to_blank_nodes
            .iter()
            .filter(|(_, nodes)| nodes.len() <= 1)
            .map(|(hash, nodes)| {
                nodes
                    .first()
                    .ok_or_else(|| {
                        NormalizationError::Unexpected(
                            "Could not label unique node, node not found".to_string(),
                        )
                    })
                    .map(|node| (hash.clone(), node.clone()))
            })
            .collect::<Result<Vec<_>, NormalizationError>>()?;

        for (hash, node) in unique_nodes {
            self.hash_to_blank_nodes.remove(&hash);
            self.canonical_issuer.get_or_issue(node);
        }

        Ok(())
    }

    fn compute_n_degree_hashes(&mut self) -> Result<(), NormalizationError> {
        for nodes in self
            .hash_to_blank_nodes
            .values()
            .cloned()
            .collect::<Vec<_>>()
        {
            let mut hash_path_list: Vec<(String, IdentifierIssuer)> =
                Vec::with_capacity(nodes.len());

            for node in &nodes {
                if self.canonical_issuer.issued(node) {
                    continue;
                }

                let mut scoped_issuer =
                    IdentifierIssuer::new(Self::TEMPORARY_BLANK_NODES_IDENTIFIER_PREFIX, 0u128);
                scoped_issuer.get_or_issue(node.clone());

                let (n_degree_hash, issuer) =
                    self.compute_n_degree_hash(&mut scoped_issuer, node)?;
                hash_path_list.push((n_degree_hash, issuer.clone()));
            }

            hash_path_list.sort_by(|left, right| left.0.cmp(&right.0));
            for (_, issuer) in hash_path_list {
                for node in issuer.issue_log {
                    self.canonical_issuer.get_or_issue(node);
                }
            }
        }

        Ok(())
    }

    fn compute_n_degree_hash(
        &mut self,
        scoped_issuer: &mut IdentifierIssuer,
        node: &String,
    ) -> Result<(String, IdentifierIssuer), NormalizationError> {
        let mut hashes: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for quad in self.blank_node_to_quads.get(node).ok_or_else(|| {
            NormalizationError::Unexpected(
                "Could not compute n degree hash, quads for node not found".to_string(),
            )
        })? {
            for (related, position) in [
                match quad.subject {
                    Subject::BlankNode(BlankNode { id }) if id != node => {
                        Some((id, Self::HASH_RELATED_BLANK_NODE_POSITION_S))
                    }
                    _ => None,
                },
                match quad.object {
                    Term::BlankNode(BlankNode { id }) if id != node => {
                        Some((id, Self::HASH_RELATED_BLANK_NODE_POSITION_O))
                    }
                    _ => None,
                },
                match quad.graph_name {
                    Some(GraphName::BlankNode(BlankNode { id })) if id != node => {
                        Some((id, Self::HASH_RELATED_BLANK_NODE_POSITION_G))
                    }
                    _ => None,
                },
            ]
            .iter()
            .flatten()
            {
                let hash =
                    self.compute_related_blank_node_hash(quad, scoped_issuer, related, position)?;

                hashes
                    .entry(hash)
                    .and_modify(|v| v.push(related.to_string()))
                    .or_insert(vec![related.to_string()]);
            }
        }

        let mut hasher = sha2::Sha256::new();
        let mut chosen_issuer =
            IdentifierIssuer::new(Self::TEMPORARY_BLANK_NODES_IDENTIFIER_PREFIX, 0u128);
        let mut chosen_path = String::new();

        for (hash, related) in hashes {
            hasher.update(hash);

            for p in related.as_slice().permutations() {
                let mut issuer = scoped_issuer.clone();
                let mut path = String::from("_:");
                let mut recursion_list = Vec::new();

                for related in p {
                    if let Some(id) = self.canonical_issuer.get(&related) {
                        path.push_str(id);
                    } else {
                        if !issuer.issued(&related) {
                            recursion_list.push(related.clone());
                        }
                        path.push_str(issuer.get_str_or_issue(related));
                    }
                }

                if !chosen_path.is_empty() && path.len() >= chosen_path.len() && path > chosen_path
                {
                    continue;
                }

                for related in recursion_list {
                    let (result, mut issuer) = self.compute_n_degree_hash(&mut issuer, &related)?;
                    path.push_str("_:");
                    path.push_str(issuer.get_str_or_issue(related));
                    path.push('<');
                    path.push_str(&result);
                    path.push('>');

                    if !chosen_path.is_empty()
                        && path.len() >= chosen_path.len()
                        && path > chosen_path
                    {
                        continue;
                    }
                }

                if chosen_path.is_empty() || chosen_path > path {
                    chosen_path = path;
                    chosen_issuer = issuer;
                }
            }

            hasher.update(chosen_path.as_str());
        }

        Ok((
            base16ct::lower::encode_string(&hasher.finalize()),
            chosen_issuer,
        ))
    }

    fn compute_related_blank_node_hash(
        &self,
        quad: &Quad<'_>,
        scoped_issuer: &mut IdentifierIssuer,
        node: &str,
        position: &str,
    ) -> Result<String, NormalizationError> {
        let mut hasher = sha2::Sha256::new();
        hasher.update(position);
        if position != Self::HASH_RELATED_BLANK_NODE_POSITION_G {
            hasher.update("<");
            hasher.update(quad.predicate.iri);
            hasher.update(">");
        }

        hasher.update(
            self.canonical_issuer
                .get(node)
                .or_else(|| scoped_issuer.get(node))
                .map(|s| format!("_:{0}", s))
                .or_else(|| self.blank_node_to_hash.get(node).cloned())
                .ok_or_else(|| {
                    NormalizationError::Unexpected(
                        "Could not compute related node hash, node not found".to_string(),
                    )
                })?,
        );

        Ok(base16ct::lower::encode_string(&hasher.finalize()))
    }

    fn hash(data: String) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();

        base16ct::lower::encode_string(&hash)
    }

    fn serialize(quads: &[Quad<'_>]) -> String {
        let mut raw_sorted = BTreeMap::new();
        for quad in quads {
            raw_sorted.insert(format!("{} .\n", quad), ());
        }

        raw_sorted.keys().join("")
    }
}

impl<'a> Default for Normalizer<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Canonical blank node identifier issuer, specified by: https://www.w3.org/TR/rdf-canon/#issue-identifier.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct IdentifierIssuer {
    prefix: String,
    pub counter: u128,
    issued: HashMap<String, (u128, String)>,
    issue_log: Vec<String>,
}

impl IdentifierIssuer {
    pub fn new(prefix: &str, counter_offset: u128) -> Self {
        Self {
            prefix: prefix.to_string(),
            counter: counter_offset,
            issued: HashMap::new(),
            issue_log: Vec::new(),
        }
    }

    pub fn get_or_issue(&mut self, identifier: String) -> (u128, &str) {
        let res = match self.issued.entry(identifier.clone()) {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => {
                let n = self.counter;
                let str = format!("{}{}", self.prefix, n);
                self.counter += 1;

                self.issue_log.push(identifier);
                e.insert((n, str))
            }
        };
        (res.0, res.1.as_str())
    }

    pub fn get_n_or_issue(&mut self, identifier: String) -> u128 {
        self.get_or_issue(identifier).0
    }

    pub fn get_str_or_issue(&mut self, identifier: String) -> &str {
        self.get_or_issue(identifier).1
    }

    pub fn get(&self, identifier: &str) -> Option<&str> {
        self.issued.get(identifier).map(|(_, str)| str.as_str())
    }

    pub fn issued(&self, identifier: &str) -> bool {
        self.issued.contains_key(identifier)
    }

    pub fn issued_iter(&self) -> Iter<'_, String, (u128, String)> {
        self.issued.iter()
    }
}

trait WithBlankNodes<'a> {
    fn blank_nodes(&self) -> Vec<String>;

    fn swap_blank_nodes<F>(&mut self, swap_fn: &'a F)
    where
        F: Fn(&'a str) -> &'a str;

    fn try_swap_blank_nodes<F, E>(&mut self, swap_fn: &'a F) -> Result<(), E>
    where
        F: Fn(&'a str) -> Result<&'a str, E>;
}

impl<'a> WithBlankNodes<'a> for Quad<'a> {
    fn blank_nodes(&self) -> Vec<String> {
        let mut nodes = Vec::new();

        if let Subject::BlankNode(n) = self.subject {
            nodes.push(n.id.to_string());
        }
        if let Term::BlankNode(n) = self.object {
            nodes.push(n.id.to_string());
        }
        if let Some(GraphName::BlankNode(n)) = self.graph_name {
            nodes.push(n.id.to_string());
        }

        nodes
    }

    fn swap_blank_nodes<F>(&mut self, swap_fn: &'a F)
    where
        F: Fn(&'a str) -> &'a str,
    {
        if let Subject::BlankNode(n) = self.subject {
            self.subject = Subject::BlankNode(BlankNode { id: swap_fn(n.id) });
        }
        if let Term::BlankNode(n) = self.object {
            self.object = Term::BlankNode(BlankNode { id: swap_fn(n.id) });
        }
        if let Some(GraphName::BlankNode(n)) = self.graph_name {
            self.graph_name = Some(GraphName::BlankNode(BlankNode { id: swap_fn(n.id) }));
        }
    }

    fn try_swap_blank_nodes<F, E>(&mut self, swap_fn: &'a F) -> Result<(), E>
    where
        F: Fn(&'a str) -> Result<&'a str, E>,
    {
        if let Subject::BlankNode(n) = self.subject {
            self.subject = Subject::BlankNode(BlankNode { id: swap_fn(n.id)? });
        }
        if let Term::BlankNode(n) = self.object {
            self.object = Term::BlankNode(BlankNode { id: swap_fn(n.id)? });
        }
        if let Some(GraphName::BlankNode(n)) = self.graph_name {
            self.graph_name = Some(GraphName::BlankNode(BlankNode { id: swap_fn(n.id)? }));
        }

        Ok(())
    }
}

/// Helper iterator over all the possible permutations of an array.
/// It internally implements the quickperm algorithm: https://www.quickperm.org.
struct PermutationsIter<T: Clone> {
    next: Option<Vec<T>>,
    a: Vec<T>,
    p: Vec<usize>,
    i: usize,
}

impl<T: Clone> PermutationsIter<T> {
    pub fn new(src: &[T]) -> Self {
        let mut p = Vec::with_capacity(src.len() + 1);
        for i in 0..=src.len() {
            p.push(i);
        }

        Self {
            next: Some(src.to_vec()),
            a: src.to_vec(),
            p,
            i: 1,
        }
    }

    fn permute(&mut self) -> Option<Vec<T>> {
        if self.i >= self.a.len() {
            None?;
        }

        (&mut self.p)[self.i] -= 1;
        let j = (self.i % 2) * (&self.p)[self.i];

        self.a.swap(j, self.i);
        self.i = 1;

        while (&self.p)[self.i] == 0 {
            (&mut self.p)[self.i] = self.i;
            self.i += 1;
        }

        Some(self.a.clone())
    }
}

impl<T: Clone> Iterator for PermutationsIter<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let crt = self.next.clone()?;
        self.next = self.permute();
        Some(crt)
    }
}

trait Permutable<T: Clone> {
    fn permutations(&self) -> PermutationsIter<T>;
}

impl<T: Clone> Permutable<T> for &[T] {
    fn permutations(&self) -> PermutationsIter<T> {
        PermutationsIter::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rio_api::model::{Literal, NamedNode};

    #[test]
    fn normalize() {
        let cases = vec![
            (
                vec![
                    Quad {
                        subject: Subject::NamedNode(NamedNode {
                            iri: "http://example.com/#p",
                        }),
                        predicate: NamedNode {
                            iri: "http://example.com/#q",
                        },
                        object: Term::BlankNode(BlankNode { id: "e0" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::NamedNode(NamedNode {
                            iri: "http://example.com/#p",
                        }),
                        predicate: NamedNode {
                            iri: "http://example.com/#r",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#s",
                        },
                        object: Term::NamedNode(NamedNode {
                            iri: "http://example.com/#u",
                        }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#t",
                        },
                        object: Term::NamedNode(NamedNode {
                            iri: "http://example.com/#u",
                        }),
                        graph_name: None,
                    },
                ],
                "197dce9a2a3f3c4bb4591910b3762146423c1a4f6901e3789490d1f28fd5e796".to_string(),
            ),
            (
                vec![
                    Quad {
                        subject: Subject::NamedNode(NamedNode {
                            iri: "http://example.com/#p",
                        }),
                        predicate: NamedNode {
                            iri: "http://example.com/#q",
                        },
                        object: Term::BlankNode(BlankNode { id: "e0" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::NamedNode(NamedNode {
                            iri: "http://example.com/#p",
                        }),
                        predicate: NamedNode {
                            iri: "http://example.com/#q",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p",
                        },
                        object: Term::BlankNode(BlankNode { id: "e2" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p",
                        },
                        object: Term::BlankNode(BlankNode { id: "e3" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e2" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#r",
                        },
                        object: Term::BlankNode(BlankNode { id: "e3" }),
                        graph_name: None,
                    },
                ],
                "a561b3db619593d5d255343fe8e40411fdc35836e8a995ffc84b4d54ad9cfabc".to_string(),
            ),
            (
                vec![
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p1",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p2",
                        },
                        object: Term::Literal(Literal::Simple { value: "Foo" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e2" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p1",
                        },
                        object: Term::BlankNode(BlankNode { id: "e3" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e3" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p2",
                        },
                        object: Term::Literal(Literal::Simple { value: "Foo" }),
                        graph_name: None,
                    },
                ],
                "f69f0a9035e18f6c3ab7e0a2a98d2594b19fa05ebebe5cb2efdc0f9d756a8136".to_string(),
            ),
            (
                vec![
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.org/vocab#next",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.org/vocab#prev",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.org/vocab#next",
                        },
                        object: Term::BlankNode(BlankNode { id: "e0" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.org/vocab#prev",
                        },
                        object: Term::BlankNode(BlankNode { id: "e0" }),
                        graph_name: None,
                    },
                ],
                "63e7fb42c6e41ed4b4465cacefbdd27c618e6ec088fd331c92aea1bbadb9a2f1".to_string(),
            ),
            (
                vec![
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p1",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p2",
                        },
                        object: Term::Literal(Literal::Simple { value: "Foo" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p3",
                        },
                        object: Term::BlankNode(BlankNode { id: "g0" }),
                        graph_name: None,
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e0" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p1",
                        },
                        object: Term::BlankNode(BlankNode { id: "e1" }),
                        graph_name: Some(GraphName::BlankNode(BlankNode { id: "g0" })),
                    },
                    Quad {
                        subject: Subject::BlankNode(BlankNode { id: "e1" }),
                        predicate: NamedNode {
                            iri: "http://example.com/#p2",
                        },
                        object: Term::Literal(Literal::Simple { value: "Bar" }),
                        graph_name: Some(GraphName::BlankNode(BlankNode { id: "g0" })),
                    },
                ],
                "94ac982a844fa31a439f98427978be93a1b489988aea0b939cdcc32d6bb4fddc".to_string(),
            ),
        ];

        for case in cases {
            let mut normalizer = Normalizer::new();
            let res = normalizer.normalize(&case.0);
            assert_eq!(res.is_ok(), true);
            assert_eq!(Normalizer::hash(res.unwrap()), case.1);
        }
    }

    #[test]
    fn permutations() {
        let cases: Vec<(Vec<i32>, Vec<Vec<i32>>)> = vec![
            (vec![], vec![vec![]]),
            (vec![1], vec![vec![1]]),
            (vec![1, 2], vec![vec![1, 2], vec![2, 1]]),
            (
                vec![1, 2, 3],
                vec![
                    vec![1, 2, 3],
                    vec![2, 1, 3],
                    vec![3, 1, 2],
                    vec![1, 3, 2],
                    vec![2, 3, 1],
                    vec![3, 2, 1],
                ],
            ),
        ];

        for case in cases {
            let result: Vec<Vec<i32>> = case.0.as_slice().permutations().collect();
            assert_eq!(result, case.1);
        }
    }
}
