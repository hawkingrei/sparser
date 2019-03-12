use crate::bitmap::Bitmap;

use decompose_ascii_rawfilters::ascii_rawfilters;
use rdtsc;

// Max size of a single search string.
const SPARSER_MAX_QUERY_LENGTH: usize = 16;
// Max number of search strings in a single query.
const SPARSER_MAX_QUERY_COUNT: usize = 32;

// Max substrings to consider.
const MAX_SUBSTRINGS: usize = 32;
// Max records to sample.
const MAX_SAMPLES: usize = 1024;
// Max record depth.
const MAX_SCHEDULE_SIZE: usize = 4;

const PARSER_MEASUREMENT_SAMPLES: usize = 10;

// Defines a sparser query, which is currently a set of conjunctive string
// terms that we search for.
#[derive(Default)]
pub struct sparser_query {
    queries: Vec<Vec<u8>>,
}

pub struct sparser_stats {
    // Number of records processed.
    records: u64,
    // Number of times the search query matched.
    total_matches: u64,
    // Number of records sparser passed.
    sparser_passed: u64,
    // Number of records the callback passed by returning true.
    callback_passed: u64,
    // Total number of bytes we had to walk forward to see a new record,
    // when a match was found.
    bytes_seeked_forward: u64,
    // Total number of bytes we had to walk backward to see a new record,
    // when a match was found.
    bytes_seeked_backward: u64,
    // Fraction that sparser passed that the callback also passed
    fraction_passed_correct: f64,
    // Fraction of false positives.
    fraction_passed_incorrect: f64,
}

pub struct search_data {
    // Number of records sampled.
    num_records: u64,
    // The false positive masks for each sample.
    passthrough_masks: Bitmap,
    // Cost of the full parser.
    full_parse_cost: u64,
    // Best cost so far.
    best_cost: u64,
    // Best schedule (indexes into ascii_rawfilters_t).
    best_schedule: Vec<u32>,
    // Length of the full parser.
    schedule_len: u32,

    // The joint bitmap (to prevent small repeated malloc's)
    joint: Bitmap,

    // number of schedules skipped.
    skipped: u64,
    // number of schedules processed.
    processed: u64,
    // Total cycles spent *processing and skipping*.
    total_cycles: i64,
}

impl sparser_query {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add(&mut self, string: String) {
        let mut string_bytes = string.as_bytes();
        let data = if SPARSER_MAX_QUERY_LENGTH < string_bytes.len() {
            string_bytes
                .get(0..SPARSER_MAX_QUERY_LENGTH)
                .unwrap()
                .to_vec()
        } else {
            string_bytes.to_vec()
        };
        self.queries.push(data);
    }
}

#[inline(always)]
fn rf_cost(len: usize) -> f64 {
    return len as f64 * 8.0;
}

pub fn search_schedules(
    predicates: ascii_rawfilters,
    mut sd: search_data,
    len: usize,
    start: usize,
    result: Vec<usize>,
) {
    if len == 0 {
        let start_rdtsc = rdtsc();
        for i in 0..result.len() {
            for j in 0..result.len() {
                if i != j
                    && predicates.sources.get(*result.get(i).unwrap()).unwrap()
                        == predicates.sources.get(*result.get(j).unwrap()).unwrap()
                {
                    let end_rdtsc = rdtsc();
                    sd.skipped += 1;
                    sd.total_cycles += (end_rdtsc - start_rdtsc);
                    return;
                }
            }
        }
        let first_index = result.get(0).unwrap();
        unsafe {
            sd.joint
                .set(if sd.passthrough_masks.is_set(*first_index as i64) {
                    1
                } else {
                    0
                });
        }

        let total_cost = rf_cost(predicates.region.get(*first_index).unwrap().len());
        for i in 0..result.len() {}
    }
}
