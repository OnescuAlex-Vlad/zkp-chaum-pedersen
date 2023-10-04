use num_bigint::{BigUint, RandBigInt};


pub fn exponentiate(num: &BigUint, exponent: &BigUint, modulus: &BigUint) -> BigUint {
    num.modpow(exponent, modulus)
}

pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        return (k - c * x).modpow(&BigUint::from(1u32), q);
    }

    q - (c * x - k).modpow(&BigUint::from(1u32), q)
}

pub fn verify(
    r1: &BigUint,
    r2: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    alpha: &BigUint,
    beta: &BigUint,
    c: &BigUint,
    s: &BigUint,
    p: &BigUint,
) -> bool {
    let cond1 = *r1 == (alpha.modpow(s, p) * y1.modpow(c, p)).modpow(&BigUint::from(1u32), &p);
    let cond2 = *r2 == (beta.modpow(s, p) * y2.modpow(c, p)).modpow(&BigUint::from(1u32), &p);

    cond1 && cond2
}

pub fn generate_random(limit: &BigUint) -> BigUint {
    let mut rng = rand::thread_rng();

    rng.gen_biguint_below(limit)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = BigUint::from(7u32);

        let c = BigUint::from(4u32);

        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);

        assert_eq!(y1, BigUint::from(2u32));
        assert_eq!(y2, BigUint::from(3u32));

        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);

        assert_eq!(r1, BigUint::from(8u32));
        assert_eq!(r2, BigUint::from(4u32));

        let s = solve(&k, &c, &x, &q);
        assert_eq!(s, BigUint::from(5u32));

        let res = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(res);
    }

    #[test]
    fn test_random() {
        let alpha = BigUint::from(4u32);
        let beta = BigUint::from(9u32);
        let p = BigUint::from(23u32);
        let q = BigUint::from(11u32);

        let x = BigUint::from(6u32);
        let k = generate_random(&q);

        let c = generate_random(&q);

        let y1 = exponentiate(&alpha, &x, &p);
        let y2 = exponentiate(&beta, &x, &p);

        let r1 = exponentiate(&alpha, &k, &p);
        let r2 = exponentiate(&beta, &k, &p);

        let s = solve(&k, &c, &x, &q);

        let res = verify(&r1, &r2, &y1, &y2, &alpha, &beta, &c, &s, &p);
        assert!(res);
    }
}
