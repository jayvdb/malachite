use malachite_base::num::arithmetic::traits::ModInverse;
use malachite_base::num::basic::traits::One;
use natural::arithmetic::gcd::extended_gcd::limbs_extended_gcd;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_right;
use natural::InnerNatural::Small;
use natural::Natural;

fn mod_inverse_helper(x: Natural, m: Natural) -> Option<Natural> {
    let mut xs = x.into_limbs_asc();
    let mut ys = m.to_limbs_asc();
    let len = ys.len();
    xs.resize(len, 0);
    let mut gs = vec![0; len];
    let mut ss = vec![0; len + 1];
    let (g_len, ss_sign) = limbs_extended_gcd(&mut gs, &mut ss, &mut xs, &mut ys);
    gs.truncate(g_len);
    if Natural::from_owned_limbs_asc(gs) != 1u32 {
        return None;
    }
    if !ss_sign {
        assert_eq!(ss.pop(), Some(0));
        limbs_sub_same_length_in_place_right(&m.into_limbs_asc(), &mut ss);
    }
    Some(Natural::from_owned_limbs_asc(ss))
}

impl ModInverse for Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$.
    /// Assumes the first [`Natural`] is already reduced modulo $m$. Both [`Natural`]s are taken by
    /// value.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), m.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_inverse(Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(Natural::from(4u32).mod_inverse(Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: Natural) -> Option<Natural> {
        assert_ne!(self, 0u32);
        assert!(self < m);
        match (self, m) {
            (x @ natural_one!(), _) => Some(x),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(y).map(Natural::from),
            (a, b) => mod_inverse_helper(a, b),
        }
    }
}

impl<'a> ModInverse<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$.
    /// Assumes the first [`Natural`] is already reduced modulo $m$. The first [`Natural`] is taken
    /// by value and the second by reference.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), m.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_inverse(&Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!(Natural::from(4u32).mod_inverse(&Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: &'a Natural) -> Option<Natural> {
        assert_ne!(self, 0u32);
        assert!(self < *m);
        match (self, m) {
            (x @ natural_one!(), _) => Some(x),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(*y).map(Natural::from),
            (a, b) => mod_inverse_helper(a, b.clone()),
        }
    }
}

impl<'a> ModInverse<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$.
    /// Assumes the first [`Natural`] is already reduced modulo $m$. The first [`Natural`]s is
    /// taken by reference and the second by value.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), m.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_inverse(Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!((&Natural::from(4u32)).mod_inverse(Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: Natural) -> Option<Natural> {
        assert_ne!(*self, 0u32);
        assert!(*self < m);
        match (self, m) {
            (natural_one!(), _) => Some(Natural::ONE),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(y).map(Natural::from),
            (a, b) => mod_inverse_helper(a.clone(), b),
        }
    }
}

impl<'a, 'b> ModInverse<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo another [`Natural`] $m$.
    /// Assumes  the first [`Natural`] is already reduced modulo $m$. Both [`Natural`]s are taken
    /// by reference.
    ///
    /// Returns `None` if $x$ and $m$ are not coprime.
    ///
    /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is
    /// `max(self.significant_bits(), m.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    ///
    /// use malachite_base::num::arithmetic::traits::ModInverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_inverse(&Natural::from(10u32)),
    ///     Some(Natural::from(7u32))
    /// );
    /// assert_eq!((&Natural::from(4u32)).mod_inverse(&Natural::from(10u32)), None);
    /// ```
    fn mod_inverse(self, m: &'a Natural) -> Option<Natural> {
        assert_ne!(*self, 0u32);
        assert!(self < m);
        match (self, m) {
            (natural_one!(), _) => Some(Natural::ONE),
            (Natural(Small(x)), Natural(Small(y))) => x.mod_inverse(*y).map(Natural::from),
            (a, b) => mod_inverse_helper(a.clone(), b.clone()),
        }
    }
}
