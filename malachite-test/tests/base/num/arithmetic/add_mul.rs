use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::Rand;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigneds, triples_of_signeds_var_2, triples_of_unsigneds_var_3,
};

fn add_mul_properties_unsigned_helper<T: PrimitiveUnsigned + Rand>() {
    test_properties(triples_of_unsigneds_var_3::<T>, |&(x, y, z)| {
        let result = x.add_mul(y, z);

        let mut x_alt = x;
        x_alt.add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.add_mul(z, y), result);
        assert_eq!(result.sub_mul(y, z), x);
        assert_eq!(x.checked_add_mul(y, z), Some(result));
        assert_eq!(x.saturating_add_mul(y, z), result);
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.overflowing_add_mul(y, z), (result, false));
    });

    test_properties(pairs_of_unsigneds::<T>, |&(a, b)| {
        assert_eq!(a.add_mul(T::ZERO, b), a);
        assert_eq!(a.add_mul(b, T::ZERO), a);
    });
}

fn add_mul_properties_signed_helper<T: PrimitiveSigned + Rand>()
where
    T::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    test_properties(triples_of_signeds_var_2::<T>, |&(x, y, z)| {
        let result = x.add_mul(y, z);

        let mut x_alt = x;
        x_alt.add_mul_assign(y, z);
        assert_eq!(x_alt, result);

        assert_eq!(x.add_mul(z, y), result);
        assert_eq!(result.sub_mul(y, z), x);
        assert_eq!(x.checked_add_mul(y, z), Some(result));
        assert_eq!(x.saturating_add_mul(y, z), result);
        assert_eq!(x.wrapping_add_mul(y, z), result);
        assert_eq!(x.overflowing_add_mul(y, z), (result, false));

        test_properties(pairs_of_signeds::<T>, |&(a, b)| {
            assert_eq!(a.add_mul(T::ZERO, b), a);
            assert_eq!(a.add_mul(b, T::ZERO), a);
        });
    });
}

#[test]
fn add_mul_properties() {
    add_mul_properties_unsigned_helper::<u8>();
    add_mul_properties_unsigned_helper::<u16>();
    add_mul_properties_unsigned_helper::<u32>();
    add_mul_properties_unsigned_helper::<u64>();
    add_mul_properties_unsigned_helper::<usize>();

    add_mul_properties_signed_helper::<i8>();
    add_mul_properties_signed_helper::<i16>();
    add_mul_properties_signed_helper::<i32>();
    add_mul_properties_signed_helper::<i64>();
    add_mul_properties_signed_helper::<isize>();
}