use std::collections::{HashMap, BTreeSet};

//  primes less than 20 enumerated manually
const PRIMES: [i32; 8] = [2, 3, 5, 7, 11, 13, 17, 19];

//  non-primes less than 20 enumerated manually
const NON_PRIMES: [i32; 10] = [4, 6, 8, 9, 10, 12, 14, 15, 16, 18];

fn main() {

//  Factorise all of the non-primes so we can use a look-up when we are looking for HCFs
    let mut fact_dict: HashMap<i32, Vec<i32>> = HashMap::new();
    for np in NON_PRIMES {
        fact_dict.insert(np, factors(np));
    }

//  find prime pairs (n, a) such that n - a is prime: more efficient to find primes (n - a, a) such that n - a + a = n < 20 and n prime
//  group these pairs according to the value of n - a (the smallest number of acrobats) 
    let mut prime_pairs: HashMap<i32, Vec<(i32, &i32)>> = HashMap::new();

//  it is not necessarily the case that a and b are primes, but they must be the same multiple of two primes, so we also need to collect
//  cases where a and be are not prime, then find pairs that have an HCF which complements a prime in each case
    let mut non_prime_pairs: HashMap<i32, Vec<(i32, &i32)>> = HashMap::new();

    for (idx, r) in PRIMES[..PRIMES.len() - 1].iter().enumerate() {
        for s in PRIMES[idx + 1..].iter() {

            let p_delta = s - r;

            if PRIMES.contains(&p_delta) {
                // we do not know whether n - a is larger or smaller than a, so either of the numbers we have found could be n - a
                prime_pairs.entry(*r).or_insert(Vec::new()).push((p_delta, s));
                prime_pairs.entry(p_delta).or_insert(Vec::new()).push((*r, s));
            }
            else if p_delta != 1 {
                // if p_delta is not prime, it cannot represent n - a
                non_prime_pairs.entry(*r).or_insert(Vec::new()).push((p_delta, s));
            }
        }

    }
    println!("prime pairs {:?}", prime_pairs);
    println!("");

    println!("non-prime pairs {:?}", non_prime_pairs);
    println!("");

    
    // This is where we will keep all possible values of p for each possible value of q
    let mut elephant_steps: HashMap<i32, BTreeSet<i32>> = HashMap::new();


//  for each value of n - a, find possible complementary pairs that could represent moves from n - a back to n or from n - a to n - a + b  
    for (_k, v) in &prime_pairs {
        // we can only get pairs if there is more than one in the first place
        if v.len() > 1 {
        //    println!("prime pairs leading to the same minimum number ({}) of acrobats {:?}", k, v);


            for (idx, bb) in v.iter().enumerate() {
                for aa in v[idx + 1..].iter() {
                    if aa == bb {continue}

                    let p = aa.0;
                    let q = bb.0;

                    if p > q {
                        elephant_steps.entry(q).or_insert(BTreeSet::new()).insert(p);
                    }

                }
            }
        }
    }

    //  analyse non-prime values for a and b to check no further solutions lurking here  
    for (_k, v) in &non_prime_pairs {
        // we can only get pairs if there is more than one in the first place
        if v.len() > 1 {

            for (idx, bb) in v.iter().enumerate() {
                for aa in v[idx + 1..].iter() {
                    if aa == bb {continue}

                    let a = &fact_dict[&aa.0];
                    let b = &fact_dict[&bb.0];


                    // short-hand approach to dividing by HCF in the special case of this limited set of non-primes
                    // a and b can only be equivalent to p.HCF and q.HCF if they have the same number of factors
                    // if the vector of factors is sorted, the HCF is the product of all but the last factor
                    if a.len() == b.len() {
                        let p = a[a.len() - 1];
                        let q = b[b.len() - 1];
                        elephant_steps.entry(q).or_insert(BTreeSet::new()).insert(p);
                    }
                }
            }
        }
    }


//    println!("Possible values for elephant movements:");


    // we could find the solution if we are told p, so there must be one value of q with only one value of p
    // which must be our solution
    for (k, v) in &elephant_steps {
//        println!("{:?} forwards and then {:?} back", v, k);
        if v.len() == 1 {
       
            println!("Elephant's actual movements were {:?} forwards and then {:?} back.", v, k);
        }
    }
                
}



fn factors(x: i32) -> Vec<i32> {

    let mut factors: Vec<i32> = Vec::new();

    let mut complement = x;
    for f in PRIMES {
        let mut remainder = x % f;
        while remainder == 0 && complement > 1 {
            factors.push(f);
            complement /= f;
            remainder = complement % f;
        }
    }
    factors
}