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
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint{
        // c + d = r mod p
        let r: BigUint = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint{
        // c * d = r mod p
        let r: BigUint = c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint{
        // -c mod p
        assert!(
            c < p,
            "Number c: {} is bigger or equal than p: {}", *c , *p
        );
        p - c
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint{
        // TODO: this function only is valid for a p prime
        // c^(-1) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_add_1(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r: BigUint = FiniteField::add(&c, &d, &p);
        assert_eq!(r, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(31u32);

        let r: BigUint = FiniteField::add(&c, &d, &p);
        assert_eq!(r, BigUint::from(14u32));
    }

    #[test]
    fn test_multiplication_1(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r: BigUint = FiniteField::multiplication(&c, &d, &p);
        assert_eq!(r, BigUint::from(7u32));
    }

    #[test]
    fn test_multiplication_2(){
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(41u32);

        let r: BigUint = FiniteField::multiplication(&c, &d, &p);
        assert_eq!(r, BigUint::from(40u32));
    }

    #[test]
    fn test_inverse_addition_1(){
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(51u32);

        let r: BigUint = FiniteField::inverse_addition(&c, &p);
        assert_eq!(r, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inverse_addition_2(){
        let c: BigUint = BigUint::from(52u32);
        let p: BigUint = BigUint::from(51u32);

        FiniteField::inverse_addition(&c, &p);
    }

    #[test]
    fn test_inverse_addition_identity(){
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(51u32);

        let c_inverse: BigUint = FiniteField::inverse_addition(&c, &p);
        assert_eq!(c_inverse, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&c, &c_inverse, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_inverse_multiplication(){
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(11u32);

        let c_inverse: BigUint = FiniteField::inverse_multiplication(&c, &p);

        assert_eq!(c_inverse, BigUint::from(3u32));
        assert_eq!(FiniteField::multiplication(&c, &c_inverse, &p), BigUint::from(1u32))
    }

}