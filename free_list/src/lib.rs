use std::{
    collections::{hash_map::Entry::Vacant as HashmapVacant, HashMap, HashSet},
    hash::Hash,
};
pub type RenderpassID = u32;
#[derive(Debug)]
pub struct FreeList<T: Eq + Hash + Clone> {
    too_free: HashSet<T>,
    by_renderpass: HashMap<RenderpassID, Vec<T>>,
}
impl<T: Eq + Hash + Clone> FreeList<T> {
    /// Marks the item as used
    pub fn push(&mut self, item: T, renderpass: RenderpassID) {
        if let HashmapVacant(v) = self.by_renderpass.entry(renderpass) {
            v.insert(vec![item]);
        } else {
            self.by_renderpass.get_mut(&renderpass).unwrap().push(item);
        }
    }
    /// Marks a component as to be freed, if it
    pub fn try_free(&mut self, item: T) {
        self.too_free.insert(item);
    }
    /// returns wheter the item is used in a renderpass
    pub fn is_used(&self, item: &T) -> bool {
        for (_pass, data) in self.by_renderpass.iter() {
            for data_item in data.iter() {
                if data_item == item {
                    return true;
                }
            }
        }
        false
    }
    /// Returns all items to free in a renderpass
    pub fn finish_renderpass(&mut self, done_renderpass: RenderpassID) -> HashSet<T> {
        let mut out_free = self.too_free.clone();
        for (rendeprass_id, item_vec) in self.by_renderpass.iter() {
            if rendeprass_id != &done_renderpass {
                for item in item_vec.iter() {
                    out_free.remove(item);
                }
            }
        }
        for freed in out_free.iter() {
            self.too_free.remove(freed);
        }
        self.by_renderpass.remove(&done_renderpass);
        out_free
    }
}
impl<T: Eq + Hash + Clone> Default for FreeList<T> {
    fn default() -> Self {
        Self {
            too_free: HashSet::new(),
            by_renderpass: HashMap::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_freelist() {
        let _list = FreeList::<u32>::default();
    }
    #[test]
    fn run_simple_render() {
        let mut list: FreeList<u32> = Default::default();
        list.push(1, 0);
        let r = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r.len(), 0);
        list.try_free(1);
        let r2 = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r2, vec![1]);
    }
    #[test]
    fn test_is_used() {
        let mut list: FreeList<u32> = Default::default();
        list.push(1, 0);
        assert_eq!(list.is_used(&1), true);
        let r = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r.len(), 0);
        list.try_free(1);
        let r2 = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r2, vec![1]);
        assert_eq!(list.is_used(&1), false);
    }
    #[test]
    fn multiple_renders() {
        let mut list: FreeList<u32> = Default::default();
        list.push(1, 0);
        list.push(1, 1);
        let r = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r.len(), 0);
        list.try_free(1);
        let r2 = list
            .finish_renderpass(0)
            .iter()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(r2.len(), 0);

        let r3 = list
            .finish_renderpass(1)
            .iter()
            .copied()
            .collect::<Vec<_>>();

        assert_eq!(r3, vec![1]);
    }
}
