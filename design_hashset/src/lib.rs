pub struct MyHashSet {
    base: Vec<bool>
}

impl MyHashSet {

    pub fn new() -> Self {
        MyHashSet { base: vec![false; 1000000] }
    }
    
    pub fn add(&mut self, key: i32) {
        self.base[key as usize] = true;
    }
    
    pub fn remove(&mut self, key: i32) {
        self.base[key as usize] = false
    }
    
    pub fn contains(&self, key: i32) -> bool {
        self.base[key as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut set = MyHashSet::new();

        set.add(1);
        set.add(2);

        assert_eq!(set.contains(1), true);
        assert_eq!(set.contains(3), false);

        set.remove(2);

        assert_eq!(set.contains(2), false);
    }
}
