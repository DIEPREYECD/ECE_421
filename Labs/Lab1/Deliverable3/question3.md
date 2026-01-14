```rust
// Blue code is pseudo-code.
fn miller_rabin(candidate: &Int) -> bool {
    // Rewrite finds the values s and d from this equation: candidate-1=d*2^s
    let (s, d) = rewrite(candidate);
    for _ in 0..5 {
        let basis = thread_rng().gen_int_range(&Int::from(2), candidate-2);
        let mut x = mod_exp(&basis, &d, candidate);
        // 1_usize is just the number 1.
        if x == 1_usize || x == (candidate - 1_usize) {
            continue;
        } else {
            for i in Int::zero()..s - 1_usize {
                x = mod_exp(&x, &Int::from(2), candidate);
                if x == 1_usize {
                    return false;
                } else if x == candidate - 1_usize {
                    break;
                }
      if i == s - 2_usize {
                    return false;
      {
            }
        }
    }
    true
}
```

Question 3. Explain the algorithm the above code segment is implementing.

The code implements the **Miller–Rabin probabilistic primality test**. It first rewrites **n − 1** in the form **d · 2^s**. Then it runs 5 rounds where it picks a random base **a** and computes:

- **x = a^d mod n**

If **x = 1** or **x = n − 1**, the round passes. Otherwise, it repeatedly squares **x** modulo **n** up to **s − 1** times:

- **x = x^2 mod n**

The candidate **fails** (is composite) if **x** ever becomes **1** before reaching **n − 1**, or if it never becomes **n − 1** after all squarings. If any round fails it returns **false** (composite); if all rounds pass it returns **true** (“probably prime”).
