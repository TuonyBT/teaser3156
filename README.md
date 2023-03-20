# Sunday Times teaser3156 (19 March 2023)
Consider the three positions for the elephant:
Initially, the elephant is d from the pivot, and balanced by n acrobats at the same distance, so the elephant's weight is equivalent to n acrobat weights
The elephant moves forward p feet, and a acrobats drop off to maintain balance, so n(d - p) = (n - a)d
Then the elephant moves back q feet, and b acrobats jump back on to maintain balance, so n(d - p + q) = (n - a + b)d
Expanding and simplifying each equation tells us that np = ad and nq = bd.
Dividing the corresponding sides of the two equations leaves us with p/q = a/b.
Now we are told that p and q are primes, and a and b must be integers because acrobats are quantized. Therefore a = p and b = q.
We therefore must consider all sets of three primes (n, a, b) from the set of primes less than 20, such that n - a is prime, n - a + b is prime and b < a < n.
From these sets, there must be only one that has a unique value of q for a given p

The routine is as follows:
1 find all pairs of primes < 20 whose difference is also prime.
2 find pairs of these pairs with a common minimum value. Each such pair of pairs represents a possible set of values for (n - a, n) and (n - a, n - a + b)
3 from each of these pairs of pairs, calculate p = a and q = b, then group according to the value of q
4 find the value of q for which there is a unique value for p; this p and q represent the solution
