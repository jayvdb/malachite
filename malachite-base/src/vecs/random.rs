use random::Seed;
use vecs::exhaustive::validate_oi_map;

/// Generates random `Vec`s of a given length using elements from a single iterator.
///
/// This `struct` is created by the `random_fixed_length_vecs_from_single` method. See its
/// documentation for more.
#[derive(Clone, Debug)]
pub struct RandomFixedLengthVecsFromSingle<I: Iterator> {
    len: usize,
    xs: I,
}

impl<I: Iterator> Iterator for RandomFixedLengthVecsFromSingle<I> {
    type Item = Vec<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<Vec<I::Item>> {
        Some((&mut self.xs).take(self.len).collect())
    }
}

/// Randomly generates `Vec`s of a given length using elements from a single iterator.
///
/// The probability of a particular length-$n$ `Vec` being generated is the product of the
/// probabilities of each of its elements.
///
/// If `len` is 0, the output consists of the empty list, repeated.
///
/// `xs` must be infinite.
///
/// # Expected complexity per iteration
///
/// $T(i, n) = O(nT^\prime(i))$
///
/// $M(i, n) = O(nM^\prime(i))$
///
/// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $T^\prime$ and $M^\prime$ are the
/// time and additional memory functions of `xs`.
///
/// # Examples
/// ```
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::vecs::random::random_fixed_length_vecs_from_single;
///
/// let xss = random_fixed_length_vecs_from_single(
///     2,
///     random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100)
/// ).take(10).collect::<Vec<_>>();
/// assert_eq!(
///     xss.iter().map(Vec::as_slice).collect::<Vec<_>>().as_slice(),
///     &[
///         &[95, 24], &[99, 71], &[93, 53], &[85, 34], &[48, 2], &[55, 11], &[48, 18], &[90, 93],
///         &[67, 93], &[93, 95]
///     ]
/// );
/// ```
#[inline]
pub fn random_fixed_length_vecs_from_single<I: Iterator>(
    len: usize,
    xs: I,
) -> RandomFixedLengthVecsFromSingle<I> {
    RandomFixedLengthVecsFromSingle { len, xs }
}

macro_rules! random_fixed_length_vecs {
    (
        $exhaustive_struct: ident,
        $exhaustive_fn: ident,
        $exhaustive_1_to_1_fn: ident,
        $([$i: expr, $it: ident, $xs: ident, $xs_gen: ident]),*
    ) => {
        /// Generates random `Vec`s of a given length using elements from $m$ iterators.
        ///
        /// The fixed length $n$ of the `Vec`s is greater than or equal to $m$.
        ///
        /// This struct is macro-generated. The value of $m$ is in the struct's name. Remember that
        /// $m$ is the number of input iterators, not the length of the output `Vec`s!
        #[derive(Clone, Debug)]
        pub struct $exhaustive_struct<T, $($it: Iterator<Item = T>),*> {
            $($xs: $it,)*
            output_to_input_map: Vec<usize>,
        }

        impl<T, $($it: Iterator<Item = T>),*> Iterator
            for $exhaustive_struct<T, $($it),*>
        {
            type Item = Vec<T>;

            #[inline]
            fn next(&mut self) -> Option<Vec<T>> {
                let mut out = Vec::with_capacity(self.output_to_input_map.len());
                for &i in &self.output_to_input_map {
                    out.push(
                        match i {
                            $(
                                $i => self.$xs.next(),
                            )*
                            _ => unreachable!(),
                        }
                        .unwrap(),
                    );
                }
                Some(out)
            }
        }

        /// Generates random length-$n$ `Vec`s using elements from $m$ iterators, where $m \leq n$.
        ///
        /// The `output_to_input_map` parameter defines which iterators are mapped to which slot in
        /// the output `Vec`s. The length of the output `Vec`s, $n$, is specified by the length of
        /// `output_to_input_map`.
        ///
        /// The $i$th element of `output_to_input_map` is an index from 0 to $m-1$ which specifies
        /// which iterator the $i$th output slot is populated with. Together, the elements must
        /// include all indices from 0 to $m-1$, inclusive, possibly with repetitions.
        ///
        /// This function is macro-generated. The value of $m$ is in the function's name. Remember
        /// that $m$ is the number of input iterators, not the length of the output `Vec`s!
        ///
        /// `xs` must be infinite.
        ///
        /// # Expected complexity per iteration
        ///
        /// Let $j$ be the largest index of any output associated with `xs`, $X$ the set of outputs
        /// with indices higher than $j$, $P$ the product of the lengths of all the iterators
        /// associated with the outputs in $X$, including multiplicities, and $T^\prime$ and
        /// $M^\prime$ the time and additional memory complexities of `xs`.
        ///
        /// We have
        ///
        /// $$
        /// T(i, n) = O(\sum_{j=0}^{n-1}T_j(i))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(\sum_{j=0}^{n-1}M_j(i))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_j$ and $M_j$ are the time and additional memory functions of the iterator
        /// corresponding to the $j$th output.
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::random` module.
        pub fn $exhaustive_fn<T, $($it: Iterator<Item = T>),*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
            output_to_input_map: &[usize],
        ) -> $exhaustive_struct<T, $($it),*> {
            $(
                let _max_input_index = $i;
            )*
            validate_oi_map(_max_input_index, output_to_input_map.iter().cloned());
            $exhaustive_struct {
                $($xs: $xs_gen(seed.fork(stringify!($xs))),)*
                output_to_input_map: output_to_input_map.to_vec(),
            }
        }

        /// Generates random length-$n$ `Vec`s with elements from $n$ iterators.
        ///
        /// This function is macro-generated. The value of $n$ is in the function's name.
        ///
        /// The probability of a particular length-$n$ `Vec` being generated is the product of the
        /// probabilities of each of its elements.
        ///
        /// `xs`, `ys`, `zs`, ... must be infinite.
        ///
        /// # Expected complexity per iteration
        ///
        /// $$
        /// T(i, n) = O(\sum_{j=0}^{n-1}T_j(i))
        /// $$
        ///
        /// $$
        /// M(i, n) = O(\sum_{j=0}^{n-1}M_j(i))
        /// $$
        ///
        /// where $T$ is time, $M$ is additional memory, $n$ is the number of input iterators, and
        /// $T_0, T_1, \ldots T_{n-1}$ and $M_0, M_1, \ldots M_{n-1}$ are the time and additional
        /// memory functions of the input iterators.
        ///
        /// # Examples
        ///
        /// See the documentation of the `vecs::random` module.
        #[inline]
        pub fn $exhaustive_1_to_1_fn<T, $($it: Iterator<Item = T>),*>(
            seed: Seed,
            $($xs_gen: &dyn Fn(Seed) -> $it,)*
        ) -> $exhaustive_struct<T, $($it),*> {
            $exhaustive_fn(seed, $($xs_gen,)* &[$($i),*])
        }
    }
}

random_fixed_length_vecs!(
    RandomFixedLengthVecs2Inputs,
    random_fixed_length_vecs_2_inputs,
    random_length_2_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs3Inputs,
    random_fixed_length_vecs_3_inputs,
    random_length_3_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs4Inputs,
    random_fixed_length_vecs_4_inputs,
    random_length_4_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen],
    [3, L, ws, ws_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs5Inputs,
    random_fixed_length_vecs_5_inputs,
    random_length_5_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen],
    [3, L, ws, ws_gen],
    [4, M, vs, vs_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs6Inputs,
    random_fixed_length_vecs_6_inputs,
    random_length_6_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen],
    [3, L, ws, ws_gen],
    [4, M, vs, vs_gen],
    [5, N, us, us_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs7Inputs,
    random_fixed_length_vecs_7_inputs,
    random_length_7_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen],
    [3, L, ws, ws_gen],
    [4, M, vs, vs_gen],
    [5, N, us, us_gen],
    [6, O, ts, ts_gen]
);
random_fixed_length_vecs!(
    RandomFixedLengthVecs8Inputs,
    random_fixed_length_vecs_8_inputs,
    random_length_8_vecs,
    [0, I, xs, xs_gen],
    [1, J, ys, ys_gen],
    [2, K, zs, zs_gen],
    [3, L, ws, ws_gen],
    [4, M, vs, vs_gen],
    [5, N, us, us_gen],
    [6, O, ts, ts_gen],
    [7, P, ss, ss_gen]
);