fn main() {
    let end: i32 = 100_000_000;

    println!("{}", run_sieve(end));
}

fn run_sieve(end: i32) -> i32 {
    let begin: i32 = 2;
    let length = end - begin + 1;
    let mut numbers_vec = vec![true; length as usize];
    let ceil_loop = (end as f64).sqrt() as i32;

    for i in begin..=ceil_loop {
        let numbers_vec_index = (i - begin) as usize;
        let val = numbers_vec[numbers_vec_index];
        if val == false {
            continue;
        }
        let mut j = i + i;
        while j <= end {
            let numbers_vec_index2 = (j - begin) as usize;
            numbers_vec[numbers_vec_index2] = false;
            j += i;
        }
    }

    let mut count = 0;
    for flag in numbers_vec {
        if flag {
            count += 1;
        }
    }

    count
}

#[test]
fn it_calculates_the_number_of_primes_up_to_passed_argument() {
    assert_eq!(run_sieve(10), 4);
    assert_eq!(run_sieve(100), 25);
    assert_eq!(run_sieve(1000), 168);
    assert_eq!(run_sieve(10000), 1229);
    assert_eq!(run_sieve(100000), 9592);
    assert_eq!(run_sieve(1000000), 78498);
    assert_eq!(run_sieve(10000000), 664579);
    assert_eq!(run_sieve(100000000), 5761455);
}
