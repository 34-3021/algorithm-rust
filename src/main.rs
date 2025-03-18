use core::panic;
use std::collections::HashMap;
use std::io;

fn reverse_dna(dna: &str) -> String {
    dna.chars().rev().map(|c| match c {
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => panic!("Invalid DNA character")
    }).collect()
}

fn dna_to_num(dna: char) -> u64 {
    match dna {
        'A' => 0,
        'T' => 1,
        'C' => 2,
        'G' => 3,
        _ => panic!("Invalid DNA character")
    }
}

#[derive(Debug)]
struct RefSeq {
    start: u64,
    reverse: bool
}

const MOD: u64 = 1_000_000_007;

fn insert_subseq(dna: &str, map: &mut HashMap<u64, RefSeq>, reverse: bool) {
    let dna_len = dna.len();
    let dna: Vec<char> = if reverse {
        reverse_dna(dna)
    } else {
        dna.to_string()
    }.chars().collect();

    for start in 0..dna_len {
        let mut hash: u64 = 0;
        for end in start..dna_len {
            hash = (hash * 4 + dna_to_num(dna[end])) % MOD;
            map.insert(hash, RefSeq { start: start as u64, reverse });
        }
    }
}

#[derive(Debug)]
struct Trace {
    start: u64,
    next: u64,
    reverse: bool
}

fn search_trace(query: &str, ref_map: &HashMap<u64, RefSeq>) -> Vec<Option<Trace>> {
    let query_len = query.len();
    let query: Vec<char> = query.chars().collect();
    let mut dp = vec![u64::MAX - 20; query_len]; // to avoid +1 overflow

    (0..query_len).rev().map(|start| {
        let mut hash: u64 = 0;
        (start..query_len).map(|end| {
            hash = (hash * 4 + dna_to_num(query[start])) % MOD;
            match ref_map.get(&hash) {
                None => None,
                Some(&RefSeq { start: ref_start, reverse}) => {
                    if dp[start] > dp[end] + 1 {
                        dp[start] = dp[end] + 1;
                        Some(Trace {
                            start: ref_start,
                            next: (end + 1) as u64,
                            reverse
                        })
                    } else { None }
                }
            }
        }).rev().find_map(|x| x)
    }).collect()
}

#[derive(Debug)]
struct Result {
    start: u64,
    end: u64,
    reverse: bool
}


fn parse_trace(trace: Vec<Option<Trace>>, ref_len: usize) -> Vec<Result> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < ref_len {
        match trace[pos] {
            None => panic!("No correct trace found"),
            Some(Trace { start, next, reverse }) => {
                result.push( Result {
                    start,
                    end: start + next - (pos as u64) - 1,
                    reverse
                });
                pos = next as usize;
            }
        }
    }
    result
}

fn main() {
    let mut lines = io::stdin().lines().map(|l| l.unwrap().trim().to_string());
    let ref_ = lines.next().unwrap();
    let query = lines.next().unwrap();

    let mut ref_map = HashMap::new();
    insert_subseq(&ref_, &mut ref_map, false);
    // insert_subseq(&ref_, &mut ref_map, true);

    let trace = search_trace(&query, &ref_map);
    let result = parse_trace(trace, ref_.len());

    result.iter().for_each(|x| {
        println!("[{}, {}] {}", x.start, x.end, if x.reverse {"yes"} else {"no"});
    });
}
