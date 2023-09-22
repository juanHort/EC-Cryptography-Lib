use num_bigint::{BigUint};

struct Point{
    x: BigUint,
    y: BigUint
}
struct EllipticCurve{
    a: BigUint,
    b: BigUint,
    p: BigUint
}

impl EllipticCurve{
    fn add(c: &Point, d: &Point) -> Point{
        todo!();
    }

    fn double(c: &Point) -> Point{
        todo!();
    }
    fn scalar_mul(c: &Point, d: &BigUint) -> Point{
        // addition/doubling algorithm
        todo!();
    }

}

struct FiniteField {}

impl FiniteField{
    fn addition(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint{
        // c + d = r mod p
        todo!();
    }

    fn multtiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint{
        // c * d = r mod p
        todo!();
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint{
        // -c mod p
        todo!();
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint{
        // c^(-1) mod p
        todo!();
    }

}