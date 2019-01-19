use integer::Integer;
use malachite_base::num::{AddMul, AddMulAssign, SubMul, SubMulAssign, UnsignedAbs};
use platform::{Limb, SignedLimb};

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` and b by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), -4i32), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x1_0000),
///         0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl SubMul<Integer, SignedLimb> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: Integer, c: SignedLimb) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` by value and b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), -4i32), 22);
///     assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x1_0000),
///         0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMul<&'a Integer, SignedLimb> for Integer {
    type Output = Integer;

    fn sub_mul(mut self, b: &'a Integer, c: SignedLimb) -> Integer {
        self.sub_mul_assign(b, c);
        self
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` by reference and b by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(Integer::from(3u32), -4i32), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(Integer::from(-0x1_0000),
///         0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMul<Integer, SignedLimb> for &'a Integer {
    type Output = Integer;

    fn sub_mul(self, b: Integer, c: SignedLimb) -> Integer {
        self.sub_mul(&b, c)
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), taking
/// `self` and b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).sub_mul(&Integer::from(3u32), -4i32), 22);
///     assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x1_0000),
///         0x1_0000i32).to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Integer, SignedLimb> for &'b Integer {
    type Output = Integer;

    fn sub_mul(self, b: &'a Integer, c: SignedLimb) -> Integer {
        if c >= 0 {
            self.sub_mul(b, c as Limb)
        } else {
            self.add_mul(b, c.unsigned_abs())
        }
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), in place,
/// taking b by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(Integer::from(3u32), -4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(Integer::from(-0x1_0000), 0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl SubMulAssign<Integer, SignedLimb> for Integer {
    fn sub_mul_assign(&mut self, b: Integer, c: SignedLimb) {
        self.sub_mul_assign(&b, c);
    }
}

/// Adds the product of an `Integer` (b) and a `SignedLimb` (c) to an `Integer` (self), in place,
/// taking b by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::SubMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.sub_mul_assign(&Integer::from(3u32), -4i32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.sub_mul_assign(&Integer::from(-0x1_0000), 0x1_0000i32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl<'a> SubMulAssign<&'a Integer, SignedLimb> for Integer {
    fn sub_mul_assign(&mut self, b: &'a Integer, c: SignedLimb) {
        if c >= 0 {
            self.sub_mul_assign(b, c as Limb);
        } else {
            self.add_mul_assign(b, c.unsigned_abs())
        }
    }
}
