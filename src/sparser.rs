use crate::bitmap::Bitmap;

use crate::common::time_start;
use crate::decompose_ascii_rawfilters::ascii_rawfilters;
use crate::rdtsc;

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
    passthrough_masks: Vec<Bitmap>,
    // Cost of the full parser.
    full_parse_cost: f64,
    // Best cost so far.
    best_cost: f64,
    // Best schedule (indexes into ascii_rawfilters_t).
    best_schedule: Vec<usize>,
    // Length of the full parser.
    schedule_len: usize,

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
    predicates: &ascii_rawfilters,
    sd: &mut search_data,
    len: usize,
    start: usize,
    result: &mut Vec<usize>,
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
        sd.joint = *sd.passthrough_masks.get(*first_index).unwrap();

        let mut total_cost = rf_cost(predicates.region.get(*first_index).unwrap().len());
        for i in 0..result.len() {
            let index = result.get(i).unwrap();
            let joint_rate = sd.joint.count();
            let filter_cost = rf_cost(predicates.region.get(i).unwrap().len());
            let rate = joint_rate as f64 / sd.num_records as f64;
            total_cost += filter_cost * rate;

            sd.joint = sd
                .joint
                .and(*sd.passthrough_masks.get(*first_index).unwrap());
        }

        let joint_rate = sd.joint.count();
        let filter_cost = sd.full_parse_cost;
        let rate = joint_rate as f64 / sd.num_records as f64;

        total_cost += filter_cost * rate;

        if (total_cost < sd.best_cost) {
            assert!(result.len() <= MAX_SCHEDULE_SIZE);
            sd.best_schedule = result.clone();
            sd.schedule_len = result.len();
        }

        let end_rdtsc = rdtsc();
        sd.processed += 1;
        sd.total_cycles += end_rdtsc - start_rdtsc;
        return;
    }

    for i in start..predicates.num_strings as usize - len {
        let result_len = result.len();
        if let Some(elem) = result.get_mut(result_len - len) {
            *elem = i;
        }
        search_schedules(&predicates, sd, len - 1, i + 1, result);
    }
}

#[derive(Default)]
pub struct calibrate_timing {
    sampling_total: f64,
    searching_total: f64,
    grepping_total: f64,

    cycles_per_schedule_avg: f64,
    cycles_per_parse_avg: f64,

    // scheudles.
    processed: f64,
    skipped: f64,

    total: f64,
}

fn sparser_calibrate(predicates: ascii_rawfilters) {
    let mut timing: calibrate_timing = Default::default();
    let start_e2e = time_start();

    let mut passthrough_masks: Vec<Bitmap> = Vec::with_capacity(MAX_SUBSTRINGS);
    for _ in 0..MAX_SUBSTRINGS {
        passthrough_masks.push(Default::default());
    }

    // The number of substrings to process.
    let mut num_substrings = if predicates.num_strings > MAX_SUBSTRINGS as i32 {
        MAX_SUBSTRINGS as i32
    } else {
        predicates.num_strings
    };

    // Counts number of records processed thus far.
    let mut records = 0;
    let mut parsed_records = 0;
    let mut passed = 0;
    let mut parse_cost = 0;

    let start = time_start();
}
