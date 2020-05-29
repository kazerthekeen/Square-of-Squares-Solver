extern crate rand;
extern crate threadpool;
use std::sync::mpsc::{channel, Sender};
use threadpool::ThreadPool;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use rand::Rng;
const STRIDE: i64 = 1000;

// The searching Algorithm
fn balanced_partition(root: i64) -> Vec<i64> {
    if root%3==1{
        return vec![];
    }
    let mut potential = vec![root];
    let mut lower = root;
    let mut upper = root + 1;
    let mut lower_sum = root;
    let mut upper_sum = root + 1;
    while lower >= 0 {
        lower = lower-1;
        lower_sum += lower;
        if lower_sum == upper_sum{
            if lower%3!=0 && upper%3!=1{
                potential.push(lower-1);
                potential.push(upper);
            }
            lower_sum=0;
            lower = lower -1;
            lower_sum += lower;
            upper_sum=0;
            upper += 1;
            upper_sum += upper;
        }
        if upper_sum < lower_sum{
            upper += 1;
            upper_sum += upper;
        }
    }
    return potential
}

// Checks if a soluton is valid.
fn is_unique_subset(sol: [i128;9], potential: &[i128] ) -> bool {
    for i in &sol{
        if !potential.contains(i){
            return false;
        }
    }
    return true;
}

// Validates a potential set.
fn validate_set(potentials :Vec<i64>)-> String {
    let mut expanded = expand(&potentials);
    let mut i = 1;
    while i < expanded.len() {
       let mut j = i + 2;
       while j < expanded.len(){
           let solution0 = compute_square(expanded[0], expanded[i], expanded[j]);
           if is_unique_subset(solution0, &expanded) {
               let s = format!("{:?}", solution0 );
               return s;
           }
           let solution1 = compute_square(expanded[0], expanded[i], expanded[j+1]);
           if is_unique_subset(solution1, &expanded) {
               let s = format!("{:?}", solution1 );
               return s;

           }
           let solution2 = compute_square(expanded[0], expanded[j], expanded[i]);
           if is_unique_subset(solution2, &expanded) {
               let s = format!("{:?}", solution2 );
               return s;

           }
           j = j + 2;
       }
       i = i + 2;
   }
    let s  = format!("{}", "");
    return s;
}

// This takes the numbers and generates a valid magic square from the seed.
fn compute_square(root: i128, x: i128, y: i128 ) -> [i128; 9] {
    // |1|2|3|
    // |8|0|4|
    // |7|6|5|
    let mut square: [i128; 9] = [0; 9];
    square[0] = root;
    square[1] = x;
    square[2] = y;
    square[3] = root * 3 - x - y;
    square[5] = root * 2 - x;
    square[6] = root * 2 - y;
    square[7] = x + y - root;
    square[4] = -root * 2 + 2 * x + y;
    square[8] = root * 4 -2 * x - y;
    return square
}

//Main process
fn run_thread(start: i64, end: i64, offset: i64, stepsize: i64, tx: Sender<String>){
    let mut i = start + offset;
    let mut c = 0;
    let mut timer = Instant::now();
       while i < end {
        let p = balanced_partition(i);
        if p.len() >= 9 {
            let s = validate_set(p);
            if s != "" {
                tx.send(s).unwrap();
            }
        }
        c = c + 1;
        i = i + stepsize;
        if c % STRIDE == 0 {
            let s = format!("i={} o={} t={}", i, offset, timer.elapsed().as_millis());
            timer = Instant::now();
            c = 0;
            tx.send(s).unwrap();
        }
    }
}

fn expand(potentials :&[i64]) -> Vec<i128> {
    let mut expanded = Vec::new();
    for i in 0..potentials.len() {
        let num = (potentials[i] * 2) + 1;
        let sqr = num as i128 * num as i128;
        expanded.push(sqr) ;
    }
    return expanded;
}

#[cfg_attr(tarpaulin, skip)]
fn main()-> std::io::Result<()> {
    let start = 90_000;
    let end = 300_000;
    let pool_size=12;
    let mut file = File::create("log.txt")?;
    let pool = ThreadPool::new(pool_size);
    let (tx, rx) = channel::<String>();
    for i in 0..pool_size {
        let txt = tx.clone();
        pool.execute(move || {
            run_thread(start, end, i as i64, pool_size as i64, txt);
        });
    }
    drop(tx);


    for received in rx {
        let s = received;
        print!("{}\n", s);
        file.write_all(s.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}

#[test]
fn test_is_valid_true() {
    let nums = vec![1,2,3,4,5,6,7,8,9,10,11];
    let sol: [i128; 9] = [1,2,3,4,5,6,7,8,9];
    assert!(is_unique_subset(sol, &nums));
}
/*
#[test]
fn test_is_valid_false_multiplicity() {
    let nums = vec![1,2,3,4,5,6,7,8,9,10,11];
    let sol: [i128; 9] = [1,1,1,1,1,1,1,1,1];
    assert!(!is_unique_subset(sol, &nums));
}*/

#[test]
fn test_is_valid_false() {
    let nums = vec![10,11,12,13,14,15,16,17,18,19,20,21];
    let sol: [i128; 9] = [1,2,3,4,5,6,7,8,9];
    assert!( !is_unique_subset(sol, &nums));
}

#[test]
fn test_compute_square_constant(){
    let result = compute_square(4,4,4);
    let solution: [i128; 9] = [4; 9];
    assert_eq!(solution, result);
}

#[test]
fn test_compute_square_base(){
    let result = compute_square(5,4,9);
    let solution: [i128; 9] = [5,4,9,2,7,6,1,8,3];
    assert_eq!(solution, result);
}

#[test]
fn test_balanced_partition(){
    let result = balanced_partition(12350);
    let solution = vec![12350, 7644, 15704, 5085, 16709, 2400, 17300];
    assert_eq!(solution, result);
}

#[test]
fn test_balanced_partition_rand_500(){
    let timer = Instant::now();
    let mut rng = rand::thread_rng();
    let mut s: i64;
    for _ in 0..500{
        s = rng.gen_range(0, 10_000_000);
        let result = balanced_partition(s);
        let l = result.len();
        print!("{:?}", &result);
        if l > 2{
            let er = expand(&result);
            for i in (1..l).step_by(2){
                assert_eq!(er[0]*2, er[i] + er[i+1]);
            }
        }
    }
    print!("500 random tests ran in {}ms.", timer.elapsed().as_millis()  )
}
