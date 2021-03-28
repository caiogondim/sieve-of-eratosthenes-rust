// max = 1_000_000_000

// primeCounts = { 10 : 1,                 # Historical data for validating our results - the number of primes
//                 100 : 25,               # to be found under some limit, such as 168 primes under 1000
//                 1000 : 168,
//                 10000 : 1229,
//                 100000 : 9592,
//                 1000000 : 78498,
//                 10000000 : 664579,
//                 100000000 : 5761455

// fn sieve(end) {

// }

fn main() {
    // let end: i32 = 1_000_000_000;
    let end: i32 = 100_000_000;
    let begin: i32 = 2;
    let length = end - begin + 1;
    let mut numbers_vec = vec![true; length as usize];
    let ceil_loop = (end as f64).sqrt() as i32;
    // println!("ceil_loop {}", ceil_loop);

    for i in begin..=ceil_loop {
        let numbers_vec_index = (i - begin) as usize;
        let val = numbers_vec[numbers_vec_index];
        if val == true {
            let mut j = i + i;
            while j <= end {
                let numbers_vec_index2 = (j - begin) as usize;
                numbers_vec[numbers_vec_index2] = false;
                j += i;
            }
        }
    }

    // for i in begin..=end {
    //     let numbers_vec_index = (i - begin) as usize;
    //     println!("{}: {}", i, numbers_vec[numbers_vec_index]);
    // }

    let mut count = 0;
    for flag in numbers_vec {
        if flag {
            count += 1;
        }
    }
    println!("{}", count);
}

// #[test]
// fn it_calculates_the_number_of_primes() {
//     assert_eq!(sieve(), 5761455);
// }
