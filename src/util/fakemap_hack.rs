use fakemap::FakeMap;


pub trait FakeMapExt<K> {
    fn duplicate_by_key(&mut self, k: K, reserved: K);
}

impl<K: Eq + Clone, V: Clone> FakeMapExt<K> for FakeMap<K, V> {
    /// Duplicates the entry with a given key.
    fn duplicate_by_key(&mut self, dup: K, reserved: K) {
        let keys = self.keys().cloned().collect::<Vec<_>>();
        let vals = self.values().cloned().collect::<Vec<_>>();
        for k in &keys {
            self.remove(k);
        }

        for (k, v) in keys.into_iter().zip(vals.into_iter()) {
            if k == dup {
                self.insert(reserved.clone(), v.clone());
            }
            self.insert(k, v);
        }

        for (k, _v) in self.iter_mut() {
            if *k == reserved {
                *k = dup;
                break;
            }
        }
    }
}
