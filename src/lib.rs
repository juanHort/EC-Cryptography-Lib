use num_bigint::BigUint;

#[derive(PartialEq, Clone, Debug)]
enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}

struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn add(&self, c: &Point, d: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not in the curve");
        assert!(self.is_on_curve(d), "Point is not in the curve");
        assert!(*c != *d, "Points should not be the same");

        match (c, d) {
            (Point::Identity, _) => d.clone(),
            (_, Point::Identity) => c.clone(),
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                let y1plusy2 = FiniteField::add(&y1, &y2, &self.p);
                if x1 == x2 && y1plusy2 == BigUint::from(0u32){
                    return Point::Identity;
                }
                // s =(y2 - y1) / (x2 - x1) mod p
                // x3 = s^2 - x1 - x2 mod p
                // y3 = s(x1 - x3) - y1 mod p
                let y2minusy1: BigUint = FiniteField::subtract(y2, y1, &self.p);
                let x2minusx1: BigUint = FiniteField::subtract(x2, x1, &self.p);
                let s: BigUint = FiniteField::divide(&y2minusy1, &x2minusx1, &self.p);
                let s2: BigUint = s.modpow(&BigUint::from(2u32), &self.p);
                let s2minusx1: BigUint = FiniteField::subtract(&s2, &x1, &self.p);
                let x3: BigUint = FiniteField::subtract(&s2minusx1, &x2, &self.p);
                let x1minusx3: BigUint = FiniteField::subtract(x1, &x3, &self.p);
                let sx1minusx3: BigUint = FiniteField::multiplication(&s, &x1minusx3, &self.p);
                let y3: BigUint = FiniteField::subtract(&sx1minusx3, y1, &self.p);
                Point::Coordinate(x3, y3)
            }
        }
    }

    fn double(&self, c: &Point) -> Point {
        assert!(self.is_on_curve(c), "Point is not in the curve");

        match c {
            Point::Identity => Point::Identity,
            Point::Coordinate(x1, y1) => {
                // s =(3 * x1^2 + a) / (2 * y1) mod p
                // x2 = s^2 - 2 * x1 mod p
                // y2 = s(x1 - x2) - y1 mod p
                let x1_pow_2 : BigUint = x1.modpow(&BigUint::from(2u32), &self.p);
                let three_plus_x1p2 :BigUint = FiniteField::multiplication(&BigUint::from(3u32), &x1_pow_2, &self.p);
                let s_numerator: BigUint = FiniteField::add(&self.a, &three_plus_x1p2, &self.p);
                let s_denominator: BigUint = FiniteField::multiplication(&BigUint::from(2u32), &y1, &self.p);
                let s: BigUint = FiniteField::divide(&s_numerator, &s_denominator, &self.p);

                let s_pow_2: BigUint = s.modpow(&BigUint::from(2u32), &self.p);
                let two_times_x1: BigUint = FiniteField::multiplication(&BigUint::from(2u32), &x1, &self.p);
                let x2: BigUint = FiniteField::subtract(&s_pow_2, &two_times_x1, &self.p);

                let x1_minus_x2: BigUint = FiniteField::subtract(&x1, &x2, &self.p);
                let s_times_x1x2: BigUint = FiniteField::multiplication(&s, &x1_minus_x2, &self.p);
                let y2: BigUint = FiniteField::subtract(&s_times_x1x2, &y1, &self.p);

                Point::Coordinate(x2, y2)
            }
        }
    }

    fn scalar_mul(&self, c: &Point, d: &BigUint) -> Point {
        // addition/doubling algorithm
        // B = d * A
        let mut t: Point = c.clone();
        for i in (0..(d.bits() - 1)).rev(){
            t = self.double(&t);
            if d.bit(i) {
                t = self.add(&t, c);
            }
        }
        t
    }

    fn is_on_curve(&self, c: &Point) -> bool {
        match c {
            Point::Coordinate(x, y) => {
                // y^2 = x^3 + a*x + b
                let y2: BigUint = y.modpow(&BigUint::from(2u32), &self.p);
                let x3: BigUint = x.modpow(&BigUint::from(3u32), &self.p);
                let ax: BigUint = FiniteField::multiplication(&self.a, x, &self.p);
                let x3plusax = FiniteField::add(&x3, &ax, &self.p);
                y2 == FiniteField::add(&x3plusax, &self.b, &self.p)
            }
            Point::Identity => true,
        }
    }
}

struct FiniteField {}

impl FiniteField {
    fn add(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c + d = r mod p
        let r: BigUint = c + d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn multiplication(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        // c * d = r mod p
        let r: BigUint = c * d;
        r.modpow(&BigUint::from(1u32), p)
    }

    fn inverse_addition(c: &BigUint, p: &BigUint) -> BigUint {
        // -c mod p
        assert!(c < p, "Number c: {} is bigger or equal than p: {}", *c, *p);
        p - c
    }

    fn subtract(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv: BigUint = FiniteField::inverse_addition(d, p);
        FiniteField::add(c, &d_inv, p)
    }

    fn inverse_multiplication(c: &BigUint, p: &BigUint) -> BigUint {
        // TODO: this function only is valid for a p prime
        // c^(-1) mod p
        c.modpow(&(p - BigUint::from(2u32)), p)
    }

    fn divide(c: &BigUint, d: &BigUint, p: &BigUint) -> BigUint {
        let d_inv: BigUint = FiniteField::inverse_multiplication(d, p);
        FiniteField::multiplication(c, &d_inv, p)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_1() {
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r: BigUint = FiniteField::add(&c, &d, &p);
        assert_eq!(r, BigUint::from(3u32));
    }

    #[test]
    fn test_add_2() {
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(31u32);

        let r: BigUint = FiniteField::add(&c, &d, &p);
        assert_eq!(r, BigUint::from(14u32));
    }

    #[test]
    fn test_multiplication_1() {
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(11u32);

        let r: BigUint = FiniteField::multiplication(&c, &d, &p);
        assert_eq!(r, BigUint::from(7u32));
    }

    #[test]
    fn test_multiplication_2() {
        let c: BigUint = BigUint::from(4u32);
        let d: BigUint = BigUint::from(10u32);
        let p: BigUint = BigUint::from(41u32);

        let r: BigUint = FiniteField::multiplication(&c, &d, &p);
        assert_eq!(r, BigUint::from(40u32));
    }

    #[test]
    fn test_inverse_addition_1() {
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(51u32);

        let r: BigUint = FiniteField::inverse_addition(&c, &p);
        assert_eq!(r, BigUint::from(47u32));
    }

    #[test]
    #[should_panic]
    fn test_inverse_addition_2() {
        let c: BigUint = BigUint::from(52u32);
        let p: BigUint = BigUint::from(51u32);

        FiniteField::inverse_addition(&c, &p);
    }

    #[test]
    fn test_inverse_addition_identity() {
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(51u32);

        let c_inverse: BigUint = FiniteField::inverse_addition(&c, &p);
        assert_eq!(c_inverse, BigUint::from(47u32));
        assert_eq!(FiniteField::add(&c, &c_inverse, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_inverse_multiplication() {
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(11u32);

        let c_inverse: BigUint = FiniteField::inverse_multiplication(&c, &p);

        assert_eq!(c_inverse, BigUint::from(3u32));
        assert_eq!(
            FiniteField::multiplication(&c, &c_inverse, &p),
            BigUint::from(1u32)
        )
    }

    #[test]
    fn test_ec_point_addition() {
        // y^2 = x^2 + 2x + 2 mod 17
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr = Point::Coordinate(BigUint::from(10u32), BigUint::from(6u32));

        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add(&p2, &p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_addition_identity() {
        // y^2 = x^2 + 2x + 2 mod 17
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (6,3) + (5,1) = (10,6)
        let p1 = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let p2 = Point::Identity;
        let pr = p1.clone();

        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add(&p2, &p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_addition_reflected_in_x() {
        // y^2 = x^2 + 2x + 2 mod 17
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5, 16) + (5,1) = Point::Identity
        let p1: Point = Point::Coordinate(BigUint::from(5u32), BigUint::from(16u32));
        let p2: Point = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr: Point = Point::Identity;

        let res = ec.add(&p1, &p2);
        assert_eq!(res, pr);

        let res = ec.add(&p2, &p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_doubling() {
        // y^2 = x^2 + 2x + 2 mod 17
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // (5, 1) + (5,1) = 2(5,1) = (6,3)
        let p1: Point = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
        let pr: Point = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));

        let res = ec.double(&p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_ec_point_doubling_identity() {
        // y^2 = x^2 + 2x + 2 mod 17
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        // I + I = 2I = I
        let p1: Point = Point::Identity;
        let pr: Point = Point::Identity;

        let res = ec.double(&p1);
        assert_eq!(res, pr);
    }

    #[test]
    fn test_subtract() {
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(51u32);

        assert_eq!(FiniteField::subtract(&c, &c, &p), BigUint::from(0u32));
    }

    #[test]
    fn test_divide() {
        let c: BigUint = BigUint::from(4u32);
        let p: BigUint = BigUint::from(11u32);

        assert_eq!(FiniteField::divide(&c, &c, &p), BigUint::from(1u32))
    }

    #[test]
    fn test_ec_scalar_multiplication() {
        // y^2 = x^2 + 2x + 2 mod 17 |G| = 19
        let ec: EllipticCurve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };

        let c:Point = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));

        // 2 (5, 1) = (6,3)
        let pr: Point = Point::Coordinate(BigUint::from(6u32), BigUint::from(3u32));
        let res:Point = ec.scalar_mul(&c, &BigUint::from(2u32));
        assert_eq!(res, pr);

        // 10 (5, 1) = (7,11)
        let pr: Point = Point::Coordinate(BigUint::from(7u32), BigUint::from(11u32));
        let res:Point = ec.scalar_mul(&c, &BigUint::from(10u32));
        assert_eq!(res, pr);

        // 19 (5, 1) = I
        let pr: Point = Point::Identity;
        let res:Point = ec.scalar_mul(&c, &BigUint::from(19u32));
        assert_eq!(res, pr);
    }

}
