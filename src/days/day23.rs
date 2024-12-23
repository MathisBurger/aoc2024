use std::collections::{HashMap, HashSet};

struct Party {
    connections: HashMap<String, HashSet<String>>,
    clusters: HashSet<Vec<String>>,
}

impl From<String> for Party {
    fn from(value: String) -> Self {
        let lines: Vec<&str> = value.split("\n").filter(|l| !l.is_empty()).collect();
        let mut conns: HashMap<String, HashSet<String>> = HashMap::new();
        for line in lines {
            let (from, to) = line.split_once("-").unwrap();
            if let Some(val) = conns.get_mut(&from.to_string()) {
                (*val).insert(to.to_string());
            } else {
                let mut set = HashSet::new();
                set.insert(to.to_string());
                conns.insert(from.to_string(), set);
            }
            if let Some(val) = conns.get_mut(&to.to_string()) {
                (*val).insert(from.to_string());
            } else {
                let mut set = HashSet::new();
                set.insert(from.to_string());
                conns.insert(to.to_string(), set);
            }
        }
        Party {
            connections: conns,
            clusters: HashSet::new(),
        }
    }
}

impl Party {
    pub fn task1(&mut self) -> usize {
        for node in self.connections.clone().keys() {
            self.get_all_clusters(node.clone());
        }
        self.amount_of_t_clusters()
    }

    pub fn task2(&self) -> String {
        let mut largest: usize = 0;
        let mut largest_str = "".to_string();
        for node in self.connections.clone().keys() {
            let host_largest = self.get_largest_set_for_host(node);
            if host_largest.0 > largest {
                largest = host_largest.0;
                largest_str = host_largest.1;
            }
        }
        largest_str
    }

    fn amount_of_t_clusters(&self) -> usize {
        self.clusters
            .iter()
            .filter(|c| c.iter().filter(|n| n.starts_with("t")).count() > 0)
            .count()
    }

    fn get_all_clusters(&mut self, from: String) {
        let set = self.connections.get(&from).unwrap();
        let mut checked_cache = HashSet::new();
        for left in set.iter() {
            for right in set.iter() {
                if checked_cache.contains(&(left, right)) {
                    continue;
                }
                if left == right {
                    continue;
                }
                checked_cache.insert((left, right));
                checked_cache.insert((right, left));

                if self.has_connection_to(left, right) {
                    let mut sorted: Vec<String> = vec![from.clone(), left.clone(), right.clone()];
                    sorted.sort();
                    self.clusters.insert(sorted);
                }
            }
        }
    }

    fn get_largest_set_for_host(&self, host: &String) -> (usize, String) {
        let commons = self.get_commons(host);
        for common in commons.iter() {
            if self.get_commons(common) != commons {
                return (0, "".to_string());
            }
        }

        let mut as_vec: Vec<String> = commons.into_iter().collect();
        as_vec.sort();

        (as_vec.len(), as_vec.join(","))
    }

    fn get_commons(&self, host: &String) -> HashSet<String> {
        let nodes = self.connections.get(host).unwrap();
        let commons: Vec<HashSet<String>> = nodes
            .iter()
            .map(|node| self.get_nodes_in_common(node, &nodes))
            .collect();

        let mut most_common = Self::most_common_elements(commons);
        most_common.insert(host.clone());
        most_common
    }

    fn get_nodes_in_common(&self, node: &String, nodes: &HashSet<String>) -> HashSet<String> {
        let mut set1 = self.connections.get(node).unwrap().clone();
        set1.insert(node.clone());
        set1.intersection(nodes).cloned().collect()
    }

    fn has_connection_to(&self, from: &String, to: &String) -> bool {
        if let Some(val) = self.connections.get(from) {
            return val.get(to).is_some();
        }
        false
    }

    fn most_common_elements(vec_of_sets: Vec<HashSet<String>>) -> HashSet<String> {
        let mut frequency_map: HashMap<String, u32> = HashMap::new();

        // Count the frequency of each element across all hashsets
        for set in vec_of_sets {
            for element in set {
                *frequency_map.entry(element).or_insert(0) += 1;
            }
        }

        // Find the maximum frequency
        let max_frequency = *frequency_map.values().max().unwrap();

        // Collect the elements that appear exactly `max_frequency` times
        let result: HashSet<String> = frequency_map
            .into_iter()
            .filter_map(|(key, value)| {
                if value == max_frequency {
                    Some(key)
                } else {
                    None
                }
            })
            .collect();

        result
    }
}

pub fn run(contents: String) {
    let mut party = Party::from(contents);
    println!("Task 1: {}", party.task1());
    println!("Task 2: {}", party.task2());
}
