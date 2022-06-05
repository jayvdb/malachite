use malachite_base::num::arithmetic::traits::{CheckedRoot, Reciprocal, UnsignedAbs};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use Rational;

impl CheckedRoot<u64> for Rational {
    type Output = Rational;

    /// Returns the the $n$th root of a [`Rational`], or `None` if the [`Rational`] is not a
    /// perfect $n$th power. The [`Rational`] is taken by value.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in ℚ, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(999i32).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(1000i32).checked_root(3u64).to_debug_string(), "Some(10)");
    /// assert_eq!(Rational::from(1001i32).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(-1000i32).checked_root(3u64).to_debug_string(), "Some(-10)");
    /// assert_eq!(Rational::from_signeds(22, 7).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(
    ///     Rational::from_signeds(27, 8).checked_root(3u64).to_debug_string(),
    ///     "Some(3/2)"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-27, 8).checked_root(3u64).to_debug_string(),
    ///     "Some(-3/2)"
    /// );
    /// ```
    fn checked_root(self, pow: u64) -> Option<Rational> {
        let sign = self >= 0;
        let (n, d) = self.into_numerator_and_denominator();
        let root_n;
        let root_d;
        if n.significant_bits() <= d.significant_bits() {
            root_n = Integer::from_sign_and_abs(sign, n).checked_root(pow)?;
            root_d = d.checked_root(pow)?;
        } else {
            root_d = d.checked_root(pow)?;
            root_n = Integer::from_sign_and_abs(sign, n).checked_root(pow)?;
        }
        Some(Rational {
            sign: root_n >= 0,
            numerator: root_n.unsigned_abs(),
            denominator: root_d,
        })
    }
}

impl<'a> CheckedRoot<u64> for &'a Rational {
    type Output = Rational;

    /// Returns the the $n$th root of a [`Rational`], or `None` if the [`Rational`] is not a
    /// perfect $n$th power. The [`Rational`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in ℚ, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, or if `exp` is even and `self` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(999i32).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(1000i32).checked_root(3u64).to_debug_string(), "Some(10)");
    /// assert_eq!(Rational::from(1001i32).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(-1000i32).checked_root(3u64).to_debug_string(), "Some(-10)");
    /// assert_eq!(Rational::from_signeds(22, 7).checked_root(3u64).to_debug_string(), "None");
    /// assert_eq!(
    ///     Rational::from_signeds(27, 8).checked_root(3u64).to_debug_string(),
    ///     "Some(3/2)"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-27, 8).checked_root(3u64).to_debug_string(),
    ///     "Some(-3/2)"
    /// );
    /// ```
    fn checked_root(self, pow: u64) -> Option<Rational> {
        let (n, d) = self.numerator_and_denominator_ref();
        let root_n;
        let root_d;
        if n.significant_bits() <= d.significant_bits() {
            root_n = Integer::from_sign_and_abs_ref(*self >= 0, n).checked_root(pow)?;
            root_d = d.checked_root(pow)?;
        } else {
            root_d = d.checked_root(pow)?;
            root_n = Integer::from_sign_and_abs_ref(*self >= 0, n).checked_root(pow)?;
        }
        Some(Rational {
            sign: root_n >= 0,
            numerator: root_n.unsigned_abs(),
            denominator: root_d,
        })
    }
}

impl CheckedRoot<i64> for Rational {
    type Output = Rational;

    /// Returns the the $n$th root of a [`Rational`], or `None` if the [`Rational`] is not a
    /// perfect $n$th power. The [`Rational`] is taken by value.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in ℚ, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, if `exp` is even and `self` is negative, or if `self` is zero and
    /// `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(999i32).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(1000i32).checked_root(3i64).to_debug_string(), "Some(10)");
    /// assert_eq!(Rational::from(1001i32).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!(Rational::from(-1000i32).checked_root(3i64).to_debug_string(), "Some(-10)");
    /// assert_eq!(Rational::from_signeds(22, 7).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!(
    ///     Rational::from_signeds(27, 8).checked_root(3i64).to_debug_string(),
    ///     "Some(3/2)"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-27, 8).checked_root(3i64).to_debug_string(),
    ///     "Some(-3/2)"
    /// );
    ///
    /// assert_eq!(Rational::from(1000i32).checked_root(-3i64).to_debug_string(), "Some(1/10)");
    /// assert_eq!(
    ///     Rational::from_signeds(-27, 8).checked_root(-3i64).to_debug_string(),
    ///     "Some(-2/3)"
    /// );
    /// ```
    fn checked_root(self, pow: i64) -> Option<Rational> {
        let u_pow = pow.unsigned_abs();
        if pow >= 0 {
            self.checked_root(u_pow)
        } else {
            self.checked_root(u_pow).map(Rational::reciprocal)
        }
    }
}

impl<'a> CheckedRoot<i64> for &'a Rational {
    type Output = Rational;

    /// Returns the the $n$th root of a [`Rational`], or `None` if the [`Rational`] is not a
    /// perfect $n$th power. The [`Rational`] is taken by reference.
    ///
    /// $$
    /// f(x, n) = \\begin{cases}
    ///     \operatorname{Some}(sqrt\[n\]{x}) & \text{if} \\quad \sqrt\[n\]{x} \in ℚ, \\\\
    ///     \operatorname{None} & \textrm{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `exp` is zero, if `exp` is even and `self` is negative, or if `self` is zero and
    /// `exp` is negative.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::CheckedRoot;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!((&Rational::from(999i32)).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!((&Rational::from(1000i32)).checked_root(3i64).to_debug_string(), "Some(10)");
    /// assert_eq!((&Rational::from(1001i32)).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!((&Rational::from(-1000i32)).checked_root(3i64).to_debug_string(), "Some(-10)");
    /// assert_eq!((&Rational::from_signeds(22, 7)).checked_root(3i64).to_debug_string(), "None");
    /// assert_eq!(
    ///     (&Rational::from_signeds(27, 8)).checked_root(3i64).to_debug_string(),
    ///     "Some(3/2)"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(-27, 8)).checked_root(3i64).to_debug_string(),
    ///     "Some(-3/2)"
    /// );
    ///
    /// assert_eq!((&Rational::from(1000i32)).checked_root(-3i64).to_debug_string(), "Some(1/10)");
    /// assert_eq!(
    ///     (&Rational::from_signeds(-27, 8)).checked_root(-3i64).to_debug_string(),
    ///     "Some(-2/3)"
    /// );
    /// ```
    fn checked_root(self, pow: i64) -> Option<Rational> {
        let u_pow = pow.unsigned_abs();
        if pow >= 0 {
            self.checked_root(u_pow)
        } else {
            self.checked_root(u_pow).map(Rational::reciprocal)
        }
    }
}