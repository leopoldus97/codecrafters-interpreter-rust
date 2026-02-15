use crate::{object::ObjString, value::Value};

const TABLE_MAX_LOAD: f64 = 0.75;

#[derive(Clone)]
pub struct Entry {
    pub key: Option<ObjString>,
    pub value: Value,
}

impl Entry {
    fn new() -> Self {
        Self {
            key: None,
            value: Value::Nil,
        }
    }

    fn is_empty(&self) -> bool {
        self.key.is_none() && matches!(self.value, Value::Nil)
    }

    fn is_tombstone(&self) -> bool {
        self.key.is_none() && matches!(self.value, Value::Bool(true))
    }
}

pub struct Table {
    count: usize,
    entries: Vec<Entry>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            count: 0,
            entries: Vec::new(),
        }
    }

    pub fn set(&mut self, key: ObjString, value: Value) -> bool {
        if self.entries.is_empty() {
            self.adjust_capacity(8);
        } else if (self.count + 1) as f64 > self.entries.len() as f64 * TABLE_MAX_LOAD {
            let capacity = self.entries.len() * 2;
            self.adjust_capacity(capacity);
        }

        let entry = Self::find_entry(&self.entries, &key);
        let is_new_key = entry.key.is_none() && !entry.is_tombstone();

        if is_new_key {
            self.count += 1;
        }

        let index = Self::entry_index(&self.entries, &key);
        self.entries[index].key = Some(key);
        self.entries[index].value = value;

        is_new_key
    }

    pub fn get(&self, key: &ObjString) -> Option<Value> {
        if self.count == 0 {
            return None;
        }

        let entry = Self::find_entry(&self.entries, key);
        entry.key.as_ref()?;
        Some(entry.value.clone())
    }

    pub fn delete(&mut self, key: &ObjString) -> bool {
        if self.count == 0 {
            return false;
        }

        let index = Self::entry_index(&self.entries, key);
        let entry = &self.entries[index];

        if entry.key.is_none() {
            return false;
        }

        // Place a tombstone
        self.entries[index].key = None;
        self.entries[index].value = Value::Bool(true);
        true
    }

    pub fn add_all(&self, to: &mut Table) {
        for entry in &self.entries {
            if let Some(ref key) = entry.key {
                to.set(key.clone(), entry.value.clone());
            }
        }
    }

    pub fn find_string(&self, chars: &str, hash: u32) -> Option<ObjString> {
        if self.count == 0 {
            return None;
        }

        let mut index = (hash as usize) % self.entries.len();

        loop {
            let entry = &self.entries[index];

            if entry.key.is_none() {
                // Stop if we find an empty non-tombstone entry
                if entry.is_empty() {
                    return None;
                }
            } else if let Some(ref key) = entry.key
                && key.len() == chars.len()
                && key.hash() == hash
                && key.as_str() == chars
            {
                return Some(key.clone());
            }

            index = (index + 1) % self.entries.len();
        }
    }

    fn find_entry<'a>(entries: &'a [Entry], key: &ObjString) -> &'a Entry {
        let mut index = (key.hash() as usize) % entries.len();
        let mut tombstone: Option<usize> = None;

        loop {
            let entry = &entries[index];

            if entry.key.is_none() {
                if entry.is_empty() {
                    // Empty entry
                    return if let Some(tomb_idx) = tombstone {
                        &entries[tomb_idx]
                    } else {
                        entry
                    };
                } else {
                    // We found a tombstone
                    if tombstone.is_none() {
                        tombstone = Some(index);
                    }
                }
            } else if let Some(ref entry_key) = entry.key
                && entry_key == key
            {
                // We found the key
                return entry;
            }

            index = (index + 1) % entries.len();
        }
    }

    fn entry_index(entries: &[Entry], key: &ObjString) -> usize {
        let mut index = (key.hash() as usize) % entries.len();
        let mut tombstone: Option<usize> = None;

        loop {
            let entry = &entries[index];

            if entry.key.is_none() {
                if entry.is_empty() {
                    // Empty entry
                    return tombstone.unwrap_or(index);
                } else {
                    // We found a tombstone
                    if tombstone.is_none() {
                        tombstone = Some(index);
                    }
                }
            } else if let Some(ref entry_key) = entry.key
                && entry_key == key
            {
                // We found the key
                return index;
            }

            index = (index + 1) % entries.len();
        }
    }

    fn adjust_capacity(&mut self, capacity: usize) {
        let mut new_entries = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            new_entries.push(Entry::new());
        }

        self.count = 0;
        for entry in &self.entries {
            if let Some(ref key) = entry.key {
                let index = Self::entry_index(&new_entries, key);
                new_entries[index].key = Some(key.clone());
                new_entries[index].value = entry.value.clone();
                self.count += 1;
            }
        }

        self.entries = new_entries;
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}
