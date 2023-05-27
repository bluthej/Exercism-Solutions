pub mod graph {
    use std::collections::HashMap;

    pub mod graph_items {
        use super::*;

        pub mod node {
            use super::*;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                id: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(id: &str) -> Self {
                    let id = id.to_string();
                    Self {
                        id,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs = attrs
                        .iter()
                        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                        .collect();
                    Self { attrs, ..self }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }

                pub(crate) fn id(&self) -> &str {
                    self.id.as_str()
                }
            }
        }

        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Self {
                    let start = start.to_string();
                    let end = end.to_string();
                    Self {
                        start,
                        end,
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
                    let attrs = attrs
                        .iter()
                        .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                        .collect();
                    Self { attrs, ..self }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|s| s.as_str())
                }
            }
        }
    }

    use graph_items::{edge::Edge, node::Node};

    #[derive(Clone, Debug, PartialEq)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Default for Graph {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            let nodes = nodes.to_vec();
            Self { nodes, ..self }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            let edges = edges.to_vec();
            Self { edges, ..self }
        }

        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let attrs = attrs
                .iter()
                .map(|(s1, s2)| (s1.to_string(), s2.to_string()))
                .collect();
            Self { attrs, ..self }
        }

        pub fn node(&self, id: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.id() == id)
        }
    }
}
