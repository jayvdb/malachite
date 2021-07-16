use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_vecs_min_length;
use std::fmt::Debug;

fn exhaustive_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = exhaustive_vecs_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

#[test]
fn test_exhaustive_vecs_min_length() {
    exhaustive_vecs_min_length_helper(0, nevers(), &[&[]]);
    exhaustive_vecs_min_length_helper(4, nevers(), &[]);
    exhaustive_vecs_min_length_helper(
        0,
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), (), ()],
            &[(), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
        ],
    );
    exhaustive_vecs_min_length_helper(
        5,
        exhaustive_units(),
        &[
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            &[
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
        ],
    );
    exhaustive_vecs_min_length_helper(
        0,
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
            &[false, false, false],
            &[false, false],
            &[false, false, true],
            &[false, true],
            &[false, false, false, false, false],
            &[true, false],
            &[false, true, false],
            &[true, true],
            &[false, false, false, false],
            &[false, true, true],
            &[false, false, false, true],
            &[true, false, false],
            &[false, false, false, false, false, false, false],
            &[true, false, true],
            &[false, false, true, false],
            &[true, true, false],
            &[false, false, false, false, true],
        ],
    );
    exhaustive_vecs_min_length_helper(
        3,
        exhaustive_bools(),
        &[
            &[false, false, false],
            &[false, false, false, false],
            &[false, false, true],
            &[false, false, false, false, false],
            &[false, true, false],
            &[false, false, false, true],
            &[false, true, true],
            &[false, false, false, false, false, false],
            &[true, false, false],
            &[false, false, true, false],
            &[true, false, true],
            &[false, false, false, false, true],
            &[true, true, false],
            &[false, false, true, true],
            &[true, true, true],
            &[false, false, false, false, false, false, false],
            &[false, true, false, false],
            &[false, false, false, true, false],
            &[false, true, false, true],
            &[false, false, false, false, false, true],
        ],
    );
    exhaustive_vecs_min_length_helper(
        0,
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'a', 'a', 'a', 'a'],
            &['b', 'a'],
            &['a', 'a', 'b'],
            &['b', 'b'],
            &['a', 'a', 'a', 'a'],
            &['a', 'c'],
            &['a', 'b', 'a'],
            &['b', 'c'],
            &['a', 'a', 'a', 'a', 'a', 'a'],
            &['c', 'a'],
            &['a', 'b', 'b'],
            &['c', 'b'],
            &['a', 'a', 'a', 'b'],
        ],
    );
    exhaustive_vecs_min_length_helper(
        3,
        'a'..='c',
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'a'],
            &['a', 'b', 'a'],
            &['a', 'a', 'a', 'b'],
            &['a', 'b', 'b'],
            &['a', 'a', 'a', 'a', 'a', 'a'],
            &['b', 'a', 'a'],
            &['a', 'a', 'b', 'a'],
            &['b', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'b'],
            &['b', 'b', 'a'],
            &['a', 'a', 'b', 'b'],
            &['b', 'b', 'b'],
            &['a', 'a', 'a', 'a', 'a', 'a', 'a'],
            &['a', 'a', 'c'],
            &['a', 'b', 'a', 'a'],
            &['a', 'b', 'c'],
            &['a', 'a', 'a', 'b', 'a'],
        ],
    );
    exhaustive_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['d'],
            &['a', 'a', 'a', 'a'],
            &['e'],
            &['a', 'b'],
            &['f'],
            &['a', 'a', 'b'],
            &['g'],
            &['b', 'a'],
            &['h'],
            &['a', 'a', 'a', 'a', 'a'],
            &['i'],
            &['b', 'b'],
            &['j'],
            &['a', 'b', 'a'],
        ],
    );
    exhaustive_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'a'],
            &['a', 'b', 'a'],
            &['a', 'a', 'a', 'b'],
            &['a', 'b', 'b'],
            &['a', 'a', 'a', 'a', 'a', 'a'],
            &['b', 'a', 'a'],
            &['a', 'a', 'b', 'a'],
            &['b', 'a', 'b'],
            &['a', 'a', 'a', 'a', 'b'],
            &['b', 'b', 'a'],
            &['a', 'a', 'b', 'b'],
            &['b', 'b', 'b'],
            &['a', 'a', 'a', 'a', 'a', 'a', 'a'],
            &['a', 'a', 'c'],
            &['a', 'b', 'a', 'a'],
            &['a', 'a', 'd'],
            &['a', 'a', 'a', 'b', 'a'],
        ],
    );
    exhaustive_vecs_min_length_helper(
        0,
        exhaustive_unsigneds::<u32>(),
        &[
            &[],
            &[0],
            &[1],
            &[0, 0, 0],
            &[2],
            &[0, 0],
            &[3],
            &[0, 0, 0, 0],
            &[4],
            &[0, 1],
            &[5],
            &[0, 0, 1],
            &[6],
            &[1, 0],
            &[7],
            &[0, 0, 0, 0, 0],
            &[8],
            &[1, 1],
            &[9],
            &[0, 1, 0],
        ],
    );
    exhaustive_vecs_min_length_helper(
        3,
        exhaustive_unsigneds::<u32>(),
        &[
            &[0, 0, 0],
            &[0, 0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 0, 0, 0],
            &[0, 1, 0],
            &[0, 0, 0, 1],
            &[0, 1, 1],
            &[0, 0, 0, 0, 0, 0],
            &[1, 0, 0],
            &[0, 0, 1, 0],
            &[1, 0, 1],
            &[0, 0, 0, 0, 1],
            &[1, 1, 0],
            &[0, 0, 1, 1],
            &[1, 1, 1],
            &[0, 0, 0, 0, 0, 0, 0],
            &[0, 0, 2],
            &[0, 1, 0, 0],
            &[0, 0, 3],
            &[0, 0, 0, 1, 0],
        ],
    );
}