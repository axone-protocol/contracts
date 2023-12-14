use rio_api::model::{BlankNode, GraphName, Quad, Subject, Term};
use sha2;
use sha2::Digest;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::Index;

/// A RDF normalizer allowing to canonicalize RDF data, following the https://www.w3.org/TR/rdf-canon specification.
#[derive(Eq, PartialEq, Debug)]
pub struct Normalizer<'a> {
    blank_node_to_quads: HashMap<String, Vec<Quad<'a>>>,
    hash_to_blank_nodes: HashMap<String, Vec<String>>,
    blank_node_to_hash: HashMap<String, String>,
    canonical_issuer: IdentifierIssuer,
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
            hash_to_blank_nodes: HashMap::new(),
            blank_node_to_hash: HashMap::new(),
            canonical_issuer: IdentifierIssuer::new(
                Self::CANONICAL_BLANK_NODES_IDENTIFIER_PREFIX.to_string(),
            ),
        }
    }

    pub fn normalize(&'a mut self, dataset: &[Quad<'a>]) -> String {
        self.reset();
        self.track_blank_nodes(dataset);
        self.compute_first_degree_hashes();
        self.label_unique_nodes();
        self.compute_n_degree_hashes();

        let mut canonicalized_dataset = dataset.to_vec();
        for quad in canonicalized_dataset.iter_mut() {
            if let Subject::BlankNode(n) = quad.subject {
                quad.subject = Subject::BlankNode(BlankNode {
                    id: self.canonical_issuer.get(n.id).unwrap(),
                })
            }
            if let Term::BlankNode(n) = quad.object {
                quad.object = Term::BlankNode(BlankNode {
                    id: self.canonical_issuer.get(n.id).unwrap(),
                })
            }
            if let Some(GraphName::BlankNode(n)) = quad.graph_name {
                quad.graph_name = Some(GraphName::BlankNode(BlankNode {
                    id: self.canonical_issuer.get(n.id).unwrap(),
                }))
            }
        }

        Self::serialize(&canonicalized_dataset)
    }

    fn reset(&mut self) {
        self.blank_node_to_quads = HashMap::new();
        self.hash_to_blank_nodes = HashMap::new();
        self.blank_node_to_hash = HashMap::new();
        self.canonical_issuer =
            IdentifierIssuer::new(Self::CANONICAL_BLANK_NODES_IDENTIFIER_PREFIX.to_string());
    }

    fn track_blank_nodes(&mut self, dataset: &[Quad<'a>]) {
        for quad in dataset {
            for node in quad.blank_nodes() {
                self.blank_node_to_quads
                    .entry(node)
                    .and_modify(|e| e.push(quad.clone()))
                    .or_insert(vec![quad.clone()]);
            }
        }
    }

    fn compute_first_degree_hashes(&mut self) {
        for (target, quads) in &self.blank_node_to_quads {
            let mut replacements = quads.clone();
            replacements.iter_mut().for_each(|quad| {
                quad.swap_blank_nodes(|n| {
                    if n == target {
                        return Self::HASH_FIRST_DEGREE_MARKER_SELF;
                    }
                    Self::HASH_FIRST_DEGREE_MARKER_OTHER
                })
            });

            let hash = Self::serialize(&replacements);
            self.hash_to_blank_nodes
                .entry(hash.clone())
                .and_modify(|v| v.push(target.clone()))
                .or_insert(vec![target.clone()]);
            self.blank_node_to_hash.insert(target.clone(), hash);
        }
    }

    fn label_unique_nodes(&mut self) {
        let mut sorted_hash = Vec::with_capacity(self.hash_to_blank_nodes.len());

        for hash in self.hash_to_blank_nodes.iter().filter_map(|(key, nodes)| {
            if nodes.len() > 1 {
                return None;
            }
            Some(key)
        }) {
            sorted_hash.push(hash.clone());
        }

        sorted_hash.sort();
        for hash in sorted_hash {
            self.canonical_issuer.get_or_issue(
                self.hash_to_blank_nodes
                    .remove(&hash)
                    .unwrap()
                    .index(0)
                    .clone(),
            );
        }
    }

    fn compute_n_degree_hashes(&mut self) {
        let mut sorted_first_degree_hashes: Vec<String> =
            Vec::with_capacity(self.hash_to_blank_nodes.len());
        sorted_first_degree_hashes.extend(self.hash_to_blank_nodes.keys().cloned());
        sorted_first_degree_hashes.sort();

        for hash in &sorted_first_degree_hashes {
            let nodes = match self.hash_to_blank_nodes.get(hash).cloned() {
                Some(v) => v,
                _ => continue,
            };

            let mut hash_to_node = HashMap::with_capacity(nodes.len());
            let mut sorted_n_degree_hashes: Vec<String> = Vec::with_capacity(nodes.len());

            for node in &nodes {
                if self.canonical_issuer.issued(node) {
                    continue;
                }

                let mut scoped_issuer = IdentifierIssuer::new(
                    Self::TEMPORARY_BLANK_NODES_IDENTIFIER_PREFIX.to_string(),
                );
                scoped_issuer.get_or_issue(node.clone());

                let (n_degree_hash, _) = self.compute_n_degree_hash(&mut scoped_issuer, node);
                hash_to_node.insert(n_degree_hash.clone(), node.clone());
                sorted_n_degree_hashes.push(n_degree_hash);
            }

            sorted_n_degree_hashes.sort();
            for n_degree_hash in sorted_n_degree_hashes {
                if let Some(node) = hash_to_node.get(n_degree_hash.as_str()) {
                    self.canonical_issuer.get_or_issue(node.clone());
                }
            }
        }
    }

    fn compute_n_degree_hash(
        &mut self,
        scoped_issuer: &mut IdentifierIssuer,
        node: &String,
    ) -> (String, IdentifierIssuer) {
        let mut hashes: HashMap<String, Vec<String>> = HashMap::new();

        // TODO: manage an error if quads not found instead..
        for quad in self.blank_node_to_quads.get(node).unwrap() {
            [
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
            .for_each(|(related, position)| {
                let hash =
                    self.compute_related_blank_node_hash(quad, scoped_issuer, related, position);

                hashes
                    .entry(hash)
                    .and_modify(|v| v.push(related.to_string()))
                    .or_insert(vec![related.to_string()]);
            });
        }

        let mut sorted_hashes: Vec<&String> = Vec::with_capacity(hashes.len());
        sorted_hashes.extend(hashes.keys());
        sorted_hashes.sort();

        let mut hasher = sha2::Sha256::new();
        let mut chosen_issuer =
            IdentifierIssuer::new(Self::TEMPORARY_BLANK_NODES_IDENTIFIER_PREFIX.to_string());
        let mut chosen_path = String::new();

        for hash in sorted_hashes {
            hasher.update(hash);

            for p in hashes.index(hash).as_slice().permutations() {
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
                        path.push_str(issuer.get_or_issue(related).as_str());
                    }
                }

                if !chosen_path.is_empty() && path.len() >= chosen_path.len() && path > chosen_path
                {
                    continue;
                }

                for related in recursion_list {
                    let (result, mut issuer) = self.compute_n_degree_hash(&mut issuer, &related);
                    path.push_str("_:");
                    path.push_str(issuer.get_or_issue(related).as_str());
                    path.push_str("<");
                    path.push_str(result.as_str());
                    path.push_str(">");

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

        (
            base16ct::lower::encode_string(&hasher.finalize().to_vec()),
            chosen_issuer,
        )
    }

    fn compute_related_blank_node_hash(
        &self,
        quad: &Quad<'_>,
        scoped_issuer: &mut IdentifierIssuer,
        node: &str,
        position: &str,
    ) -> String {
        let mut hasher = sha2::Sha256::new();
        hasher.update(position);
        if position != Self::HASH_RELATED_BLANK_NODE_POSITION_G {
            hasher.update("<");
            hasher.update(quad.predicate.iri);
            hasher.update(">");
        }

        hasher.update("_:");

        // TODO: consider to manage the case the node doesn't exists in blank_node_to_hash map and output
        //  an error. This cannot occur as every blank nodes has a computed first degree hash..
        if let Some(hash) = self
            .canonical_issuer
            .get(node)
            .or_else(|| scoped_issuer.get(node))
            .or_else(|| self.blank_node_to_hash.get(node))
        {
            hasher.update(hash);
        }

        base16ct::lower::encode_string(&hasher.finalize().to_vec())
    }

    fn serialize(quads: &[Quad<'_>]) -> String {
        let mut raw_sorted = Vec::with_capacity(quads.len());
        for quad in quads {
            raw_sorted.push(format!("{} .\n", quad));
        }
        raw_sorted.sort();

        let mut hasher = sha2::Sha256::new();
        for raw in raw_sorted {
            hasher.update(raw);
        }
        let hash = hasher.finalize().to_vec();

        base16ct::lower::encode_string(&hash)
    }
}

/// Canonical blank node identifier issuer, specified by: https://www.w3.org/TR/rdf-canon/#issue-identifier.
#[derive(Clone, Eq, PartialEq, Debug)]
struct IdentifierIssuer {
    prefix: String,
    counter: u64,
    issued: HashMap<String, String>,
}

impl IdentifierIssuer {
    pub fn new(prefix: String) -> Self {
        Self {
            prefix,
            counter: 0,
            issued: HashMap::new(),
        }
    }

    pub fn get_or_issue(&mut self, identifier: String) -> String {
        match self.issued.entry(identifier) {
            Entry::Occupied(e) => e.get().clone(),
            Entry::Vacant(e) => {
                let identifier = format!("{}{}", self.prefix, self.counter);
                self.counter += 1;

                e.insert(identifier).clone()
            }
        }
    }

    pub fn get(&self, identifier: &str) -> Option<&String> {
        self.issued.get(identifier)
    }

    pub fn issued(&self, identifier: &str) -> bool {
        self.issued.contains_key(identifier)
    }
}

trait WithBlankNodes {
    fn blank_nodes(&self) -> Vec<String>;

    fn swap_blank_nodes<F>(&mut self, swap_fn: F)
    where
        F: Fn(&str) -> &str;
}

impl WithBlankNodes for Quad<'_> {
    fn blank_nodes(&self) -> Vec<String> {
        let mut nodes = Vec::new();

        if let Subject::BlankNode(n) = self.subject {
            nodes.push(n.id.to_string())
        }
        if let Term::BlankNode(n) = self.object {
            nodes.push(n.id.to_string())
        }
        if let Some(GraphName::BlankNode(n)) = self.graph_name {
            nodes.push(n.id.to_string())
        }

        nodes
    }

    fn swap_blank_nodes<F>(&mut self, swap_fn: F)
    where
        F: Fn(&str) -> &str,
    {
        if let Subject::BlankNode(n) = self.subject {
            self.subject = Subject::BlankNode(BlankNode { id: swap_fn(n.id) })
        }
        if let Term::BlankNode(n) = self.object {
            self.object = Term::BlankNode(BlankNode { id: swap_fn(n.id) })
        }
        if let Some(GraphName::BlankNode(n)) = self.graph_name {
            self.graph_name = Some(GraphName::BlankNode(BlankNode { id: swap_fn(n.id) }))
        }
    }
}

/// Helper iterator over all the possible permutations of an array.
/// It internally implements the quickperm algorithm.
struct PermutationsIter<T: Clone> {
    next: Option<Vec<T>>,
    a: Vec<T>,
    p: Vec<usize>,
    i: usize,
}

impl<T: Clone> PermutationsIter<T> {
    pub fn new(src: &[T]) -> Self {
        let mut p = Vec::with_capacity(src.len() + 1);
        for i in 0..src.len() + 1 {
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
            None?
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
