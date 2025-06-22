use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

// Placeholder for v8_flags
mod v8_flags {
    pub static mut turbo_log_builtins_count_input: Option<String> = None;
    pub static mut turbo_profiling_input: Option<String> = None;
}

// Placeholder for base::LeakyObject. In Rust, using static initialization
// together with Mutex/RwLock achieves similar behavior and safety.
mod base {
    use std::sync::{Mutex, Once};

    pub struct LeakyObject<T> {
        data: Mutex<Option<T>>,
        once: Once,
    }

    impl<T> LeakyObject<T> {
        pub const fn new() -> Self {
            LeakyObject {
                data: Mutex::new(None),
                once: Once::new(),
            }
        }

        pub fn get(&self, init: impl FnOnce() -> T) -> &T {
            self.once.call_once(|| {
                let mut data = self.data.lock().unwrap();
                *data = Some(init());
            });
            let data = self.data.lock().unwrap();
            data.as_ref().unwrap()
        }
    }
}

// Placeholder for ProfileDataFromFileConstants
mod profile_data_from_file_constants {
    pub const K_BLOCK_COUNTER_MARKER: &str = "BlockCounter";
    pub const K_BUILTIN_HASH_MARKER: &str = "BuiltinHash";
    pub const K_BLOCK_HINT_MARKER: &str = "BlockHint";
}

pub mod profile_data_reader {
    use super::*;
    use std::sync::Mutex;
    use std::sync::Once;

    type BlockHints = HashMap<(usize, usize), bool>;

    #[derive(Default)]
    struct ProfileDataFromFileInternal {
        hash: Mutex<Option<i32>>,
        block_hints_by_id: Mutex<BlockHints>,
        #[cfg(feature = "log_builtin_block_count")]
        executed_count: Mutex<HashMap<usize, u64>>,
    }

    impl ProfileDataFromFileInternal {
        fn hash(&self) -> Option<i32> {
            *self.hash.lock().unwrap()
        }

        fn set_hash(&self, hash: i32) {
            let mut h = self.hash.lock().unwrap();
            *h = Some(hash);
        }

        fn add_hint_to_block(&self, true_block_id: usize, false_block_id: usize, hint: u64) {
            assert!(hint < 2);
            let mut hints = self.block_hints_by_id.lock().unwrap();
            hints.insert((true_block_id, false_block_id), hint != 0);
        }

        #[cfg(feature = "log_builtin_block_count")]
        fn add_block_execution_count(&self, block_id: usize, executed_count: u64) {
            let mut counts = self.executed_count.lock().unwrap();
            counts.insert(block_id, executed_count);
        }
    }

    pub struct ProfileDataFromFile {
        internal: ProfileDataFromFileInternal,
    }

    impl ProfileDataFromFile {
        pub fn block_hints(&self) -> HashMap<(usize, usize), bool> {
            self.internal.block_hints_by_id.lock().unwrap().clone()
        }
    }

    static PROFILE_DATA: base::LeakyObject<HashMap<String, ProfileDataFromFileInternal>> =
        base::LeakyObject::new();

    fn ensure_init_profile_data() -> &'static HashMap<String, ProfileDataFromFileInternal> {
        PROFILE_DATA.get(|| {
            let mut data = HashMap::new();

            #[cfg(feature = "log_builtin_block_count")]
            if let Some(filename) = unsafe { &v8_flags::turbo_log_builtins_count_input } {
                if let Ok(file) = File::open(filename) {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let mut tokens = line.split('\t');
                            if let Some(token) = tokens.next() {
                                if token == profile_data_from_file_constants::K_BLOCK_COUNTER_MARKER
                                {
                                    let builtin_name = tokens.next().expect("builtin_name");
                                    let block_id_str = tokens.next().expect("block_id_str");
                                    let block_id = block_id_str.parse::<usize>().expect("usize");
                                    let executed_count_str =
                                        tokens.next().expect("executed_count_str");
                                    let executed_count =
                                        executed_count_str.parse::<u64>().expect("u64");

                                    let entry = data.entry(builtin_name.to_string()).or_insert(ProfileDataFromFileInternal::default());
                                    entry.add_block_execution_count(block_id, executed_count);
                                } else if token == profile_data_from_file_constants::K_BUILTIN_HASH_MARKER
                                {
                                    let builtin_name = tokens.next().expect("builtin_name");
                                    let hash_str = tokens.next().expect("hash_str");
                                    let hash = hash_str.parse::<i32>().expect("i32");

                                    let entry = data.entry(builtin_name.to_string()).or_insert(ProfileDataFromFileInternal::default());
                                    let current_hash = entry.hash();
                                    assert!(current_hash.is_none() || current_hash == Some(hash));
                                    entry.set_hash(hash);
                                }
                            }
                        }
                    }
                } else {
                    panic!("Can't read raw count file for log builtin hotness.");
                }
            }

            if let Some(filename) = unsafe { &v8_flags::turbo_profiling_input } {
                if let Ok(file) = File::open(filename) {
                    let reader = BufReader::new(file);
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            let mut tokens = line.split(',');
                            if let Some(token) = tokens.next() {
                                if token == profile_data_from_file_constants::K_BLOCK_HINT_MARKER {
                                    let builtin_name = tokens.next().expect("builtin_name");
                                    let true_id_str = tokens.next().expect("true_id");
                                    let true_id = true_id_str.parse::<usize>().expect("usize");
                                    let false_id_str = tokens.next().expect("false_id");
                                    let false_id = false_id_str.parse::<usize>().expect("usize");
                                    let hint_str = tokens.next().expect("hint");
                                    let hint = hint_str.parse::<u64>().expect("u64");

                                    let entry = data.entry(builtin_name.to_string()).or_insert(ProfileDataFromFileInternal::default());
                                    entry.add_hint_to_block(true_id, false_id, hint);
                                } else if token == profile_data_from_file_constants::K_BUILTIN_HASH_MARKER
                                {
                                    let builtin_name = tokens.next().expect("builtin_name");
                                    let hash_str = tokens.next().expect("hash_str");
                                    let hash = hash_str.parse::<i32>().expect("i32");

                                    let entry = data.entry(builtin_name.to_string()).or_insert(ProfileDataFromFileInternal::default());
                                    let current_hash = entry.hash();
                                    assert!(current_hash.is_none() || current_hash == Some(hash));
                                    entry.set_hash(hash);
                                }
                            }
                        }
                    }
                } else {
                    panic!("Can't read log file");
                }
            }

            for pair in &data {
                assert!(pair.1.hash().is_some());
            }

            data
        })
    }

    impl ProfileDataFromFile {
        pub fn try_read(name: &str) -> Option<ProfileDataFromFile> {
            let data = ensure_init_profile_data();
            data.get(name).map(|internal| ProfileDataFromFile { internal: internal.clone() })
        }
    }
}