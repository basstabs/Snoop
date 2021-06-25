use sfml::window::Key;

use std::collections::HashMap;

pub struct Input
{

    keys: HashMap<Key, bool>

}

impl Input
{

    pub fn new() -> Input
    {

        return Input { keys: HashMap::new() };

    }

    pub fn add(&mut self, key: Key)
    {

        self.keys.insert(key, true);

    }

    pub fn remove(&mut self, key: Key)
    {

        self.keys.insert(key, false);
        
    }

    pub fn contains(&self, key: Key) -> bool
    {

        let has_key = self.keys.get(&key);

        match has_key
        {

            Some(k) => return *k,
            None => return false

        };

    }

}
