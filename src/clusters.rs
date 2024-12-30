// use core::unicode::conversions::to_lower;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct Clusters <T: Hash + Eq + Clone> {
    membership: HashMap<T, u32>,
    clusters: HashMap<u32, Cluster<T>>,
    count: u32,
}

// constructors
impl<T: Hash + Eq + Clone> Clusters<T> {
    fn new() -> Self {
        Self {
            membership: HashMap::new(),
            clusters: HashMap::new(),
            count: 0,
        }
    }

    pub fn from_node_pair(node_a: &T, node_b: &T) -> Self {
        let mut new_clusters = Self::new();

        // make cluster
        let mut cluster = Cluster::new(node_a.clone());
        cluster.add(node_b.clone());

        // add cluster
        new_clusters.add_cluster(cluster);

        new_clusters
    }
}


//methods
impl<T: Hash + Eq + Clone> Clusters<T> {

    fn add_cluster(&mut self, cluster: Cluster<T>) {
        self.clusters.insert(self.count, cluster);
        let members = self.clusters.get(&self.count)
            .expect("Just added this cluster so should be able to get it")
            .get_members();
        for mem in members {
            self.membership.insert(mem, self.count);
        }
        self.count += 1;
    }

    // Finds cluster containing existing member and adds new member to same cluster
    fn add_to_cluster_with(&mut self, new_member: &T, existing_member: &T){
        let cluster_number = self.membership.get(existing_member)
            .expect("This method should only be called after determining this member is already present");
        self.clusters.entry(cluster_number.clone())
            .and_modify(|clus| clus.add(new_member.clone()));
        self.membership.insert(new_member.clone(), cluster_number.clone());
    }

    fn merge_clusters_containing_members(&mut self, member_a: &T, member_b: &T) {
        let clus_num_a = self.membership.get(member_a)
            .expect("This method should only be called after determining this member is already present")
            .clone();
        let clus_num_b = self.membership.get(member_b)
            .expect("This method should only be called after determining this member is already present")
            .clone();

        // remove clus_b and update memberships
        let clus_b= self.clusters.remove(&clus_num_b)
            .expect("cluster numbers should correspond to clusters");

        for mem in clus_b.get_members() {
                self.membership.insert(mem.clone(), clus_num_a);
            }

        // merge clusters
        self.clusters.get_mut(&clus_num_a)
            .expect("cluster numbers should correspond to clusters")
            .merge(clus_b);
    }

    pub fn add_edge(&mut self, node_a: T, node_b: T) {
        // check if either is already in clusters
        let membership = [self.membership.contains_key(&node_a), self.membership.contains_key(&node_b)];
        match membership {
            [false, false] => { self.add_cluster(Cluster::from_edge(node_a, node_b)) },
            [true, false] => { self.add_to_cluster_with(&node_b, &node_a) },
            [false, true] => { self.add_to_cluster_with(&node_a, &node_b) },
            [true, true] => { self.merge_clusters_containing_members(&node_a, &node_b) },
        };
    }
}

// define == behaviour
impl <T: Hash + Eq + Clone> PartialEq for Clusters<T> {
    fn eq(&self, other: &Self) -> bool {
        let a: HashSet<&Cluster<T>> = HashSet::from_iter(self.clusters.values());
        let b: HashSet<&Cluster<T>> = HashSet::from_iter(other.clusters.values());
        a == b
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Cluster <T: Hash + Eq + Clone> {
    members: Vec<T>,
}

// constructors
impl<T: Hash + Eq + Clone> Cluster<T> {
    pub fn new(initial: T) -> Self {
        Self{
            members: vec![initial]
        }
    }

    pub fn from_edge(node_a: T, node_b: T) -> Self {
        let mut clus = Self::new(node_a);
        clus.add(node_b);
        clus
    }
}

// methods
impl<T: Hash + Eq + Clone> Cluster<T> {
    pub fn add(&mut self, new_member: T) {
        self.members.push(new_member)
    }

    pub fn get_members(&self) -> Vec<T> {
        self.members.clone()
    }

    pub fn merge(&mut self, other: Cluster<T>) {
        let other_members = other.get_members();
        self.members.extend(other_members);
    }
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn clusters_merge_clusters_works() {
        let node_a = 1;
        let node_b = 2;
        let node_c = 3;
        let node_d = 4;
        let mut result = Clusters::from_node_pair(&node_a, &node_b);
        result.add_edge(node_c.clone(), node_d.clone());
        result.add_edge(node_a.clone(), node_d.clone());

        let mut expected = Clusters::from_node_pair(&node_a, &node_b);
        expected.add_edge(node_b.clone(), node_c.clone());
        expected.add_edge(node_c.clone(), node_d.clone());

        assert_eq!(result, expected);
    }
}
