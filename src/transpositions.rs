use crate::game;
use game::{Bitmap, Score};

const TTABLE_SIZE: usize = 8388593; // prime, about 64 MB
const NULL_ENTRY: Entry = Entry(0);

type EntryData = u64;
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Entry(EntryData);

impl Entry {
    pub fn from_pos(key: Bitmap, eval: Score) -> Self {
        // This code is awful but Rust does not like my antics
        // Converts eval to unsigned, preserving bits and appending 0s
        let part1 = eval as EntryData;
        let offset = (game::BITMAP_SIZE_BYTES * 8) as EntryData;
        let part2 = key as EntryData;
        Self((part1 << offset) + part2)
    }

    pub fn get_key(&self) -> Bitmap {
        // Hmm, maybe you could get away with just returning the entry as is
        let mask = !(EntryData::MAX - (EntryData::MAX >> 8));
        (self.0 & mask) as Bitmap
    }

    pub fn get_eval(&self) -> Score {
        let offset = (game::BITMAP_SIZE_BYTES * 8) as EntryData;
        (self.0 >> offset) as Score
    }

    #[allow(dead_code)]
    pub fn bit_string(&self) -> String {
        format!("{:064b}", self.get_key())
    }
}

#[derive(Debug)]
pub struct TTable {
    t: Vec<Entry>,
}

#[allow(dead_code)]
impl TTable {
    pub fn new() -> Self {
        Self {
            t: vec![NULL_ENTRY; TTABLE_SIZE],
        }
    }

    pub fn reset(&mut self) {
        // don't worry, this is optimized to a single MEMSET call
        self.t.iter_mut().for_each(|x| *x = NULL_ENTRY)
    }

    pub fn put(&mut self, key: Bitmap, eval: Score) {
        let i = Self::entry_hash(key);
        let e = Entry::from_pos(key, eval);
        self.t[i] = e;
    }

    pub fn get(&self, key: Bitmap) -> Score {
        let i = Self::entry_hash(key);
        if self.t[i].get_key() == key {
            self.t[i].get_eval()
        } else {
            0
        }
    }

    fn entry_hash(e: Bitmap) -> usize {
        e as usize % TTABLE_SIZE
    }
}
