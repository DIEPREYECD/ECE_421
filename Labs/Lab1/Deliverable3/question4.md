Question 4. Several cryptographic libraries use the Miller-Rabin test to output probable primes. Albrecht
et al. were able to construct composite numbers that some of these libraries declared to be prime.
Hence, we need something better. Identify the crate which most closely implements the
recommendations from Prime and Prejudice. (Hint: The crate mentions the paper “Prime and
Prejudice”.)

My Answer:
The crate that most closely follows the recommendations in _Prime and Prejudice_ (it explicitly references the paper) is **`glass_pumpkin`**: https://crates.io/crates/glass_pumpkin.
