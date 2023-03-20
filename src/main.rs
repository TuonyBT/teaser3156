use std::collections::HashMap;

fn main() {

//  primes less than 20 enumerated manually
    let primes = [1, 2, 3, 5, 7, 11, 13, 17, 19];

//  find prime pairs (n, a) such that n - a is prime: more efficient to find primes (n - a, a) such that n - a + a = n < 20 and n prime
//  group these pairs according to the value of n - a (the smallest number of acrobats) 
    let mut prime_pairs: HashMap<&i32, Vec<(&i32, i32)>> = HashMap::new();

    for (idx, r) in primes[..5].iter().enumerate() {
        for s in primes[idx + 1..].iter() {

            let p_sum = r + s;
            if p_sum >= 20 {break}

            if primes.contains(&p_sum) {
                // we do not know whether n - a is larger or smaller than a, so either of the numbers we have found could be n - a
                prime_pairs.entry(r).or_insert(Vec::new()).push((s, r + s));
                prime_pairs.entry(s).or_insert(Vec::new()).push((r, r + s));
            }
        }

    }
    println!("prime pairs {:?}", prime_pairs);

//  for each value of n - a, find possible complementary pairs that could represent moves from n - a back to n or from n - a to n - a + b  
    for (k, v) in &prime_pairs {
        // we can only get pairs if there is more than one in the first place
        if v.len() > 1 {
            println!("prime pairs leading to the same minimum number ({}) of acrobats {:?}", k, v);

            let mut elephant_steps: HashMap<&i32, Vec<&i32>> = HashMap::new();

            for (idx, bb) in v.iter().enumerate() {
                for aa in v[idx + 1..].iter() {

                //  although the pairs are interchangeable, we are only interested in cases where p > q, 
                //  so we can rely on the sorting we have already imposed on the vector of pairs
                    let p = aa.0;
                    let q = bb.0;

                    elephant_steps.entry(q).or_insert(Vec::new()).push(p);
                }
            }

            println!("Possible values for elephant movements:");


            // we could find the solution if we are told p, so there must be one value of q with only one value of p
            // which must be our solution
            for (k, v) in &elephant_steps {
                println!("{} forwards and then {} back", v[0], k);
                if v.len() == 1 {
        
                    println!("Elephant's actual movements were {} forwards and then {} back.", v[0], k);
                }
            }
                
        }

    }


}
