use ahash::AHashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;

/// A set of node references.
///
/// Can be created from a single node reference or an iterable of
/// node references.
///
/// ```
/// use pathfinding::NodeRefs;
///
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct N(String);
///
/// let red = N("red".into());
/// let blue = N("blue".into());
/// let green = N("green".into());
///
/// let refs: NodeRefs<N> = NodeRefs::from(&red);
/// let refs: NodeRefs<N> = NodeRefs::from_iter([&red, &blue, &green]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRefs<'a, N>(AHashSet<&'a N>)
where
    N: Eq + Hash + Clone;

impl<'a, N: Eq + Hash + Clone> FromIterator<&'a N> for NodeRefs<'a, N> {
    fn from_iter<T: IntoIterator<Item = &'a N>>(iter: T) -> Self {
        NodeRefs(AHashSet::from_iter(iter))
    }
}

impl<'a, N: Eq + Hash + Clone> From<&'a N> for NodeRefs<'a, N> {
    fn from(value: &'a N) -> Self {
        NodeRefs(AHashSet::from_iter([value]))
    }
}

impl<'a, N: Eq + Hash + Clone> IntoIterator for NodeRefs<'a, N> {
    type Item = &'a N;
    type IntoIter = std::collections::hash_set::IntoIter<&'a N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, N: Eq + Hash + Clone> IntoIterator for &'a NodeRefs<'a, N> {
    type Item = &'a N;
    type IntoIter = std::iter::Copied<std::collections::hash_set::Iter<'a, &'a N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl<'a, N: Eq + Hash + Clone> Deref for NodeRefs<'a, N> {
    type Target = AHashSet<&'a N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
    struct Node(u8);

    #[test]
    fn test_from_iterator() {
        let nodes = [Node(1), Node(2), Node(3)];
        let refs = NodeRefs::from_iter(&nodes);
        assert_eq!(
            refs.0,
            AHashSet::from_iter([&nodes[0], &nodes[1], &nodes[2]])
        );
    }

    #[test]
    fn test_from_single_ref() {
        let node = Node(42);
        let refs = NodeRefs::from(&node);
        assert_eq!(refs.0, AHashSet::from_iter([&node]));
    }

    #[test]
    fn test_into_iterator() {
        let nodes = [Node(1), Node(2), Node(3)];
        let refs = NodeRefs::from_iter(&nodes);

        let v = refs.into_iter().sorted().collect::<Vec<_>>();
        assert_eq!(v, vec![&nodes[0], &nodes[1], &nodes[2]]);
    }

    #[test]
    fn test_ref_into_iterator() {
        let nodes = [Node(1), Node(2), Node(3)];
        let refs = NodeRefs::from_iter(&nodes);

        let v = (&refs).into_iter().sorted().collect::<Vec<_>>();
        assert_eq!(v, vec![&nodes[0], &nodes[1], &nodes[2]]);
    }
}
