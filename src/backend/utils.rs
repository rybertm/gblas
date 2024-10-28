use std::cmp::Ordering;

use crate::{
    algebra::{BinaryOperator, Semiring},
    types::{IndexType, NoValue},
    ApiError, GblasResult,
};

pub(crate) fn check_vals(n1: IndexType, n2: IndexType) -> GblasResult<NoValue> {
    if n1 != n2 {
        return Err(ApiError::DimensionMismatch.into());
    }
    Ok(())
}

/// _c += a_ik * b\[..\]_
///
/// **Obs:**
///
/// * `c` ***must be*** sorted by index (i.e., `c[i].0 < c[i+1].0`). This is checked in debug mode.
#[inline]
pub(crate) fn accum_mult_vec<T: Clone>(
    c: &mut Vec<(IndexType, T)>,
    b: &[(IndexType, T)],
    a: &T,
    op: &impl Semiring<T, Output = T>,
) {
    debug_assert!(c.is_sorted_by_key(|val| val.0));
    for (j, b_val) in b.iter() {
        let val = op.mult(a.clone(), b_val.clone());

        let found = c.binary_search_by_key(j, |val| val.0);
        match found {
            Ok(idx) => {
                let c_val = &mut c[idx].1;
                *c_val = op.add(c_val.clone(), val);
            }
            Err(idx) => {
                c.insert(idx, (*j, val));
            }
        }
    }
}

pub(crate) fn ewise_or_accum_vec<T: Clone>(
    res: &mut Vec<(IndexType, T)>,
    v1: &[(IndexType, T)],
    v2: &[(IndexType, T)],
    bin_op: &impl BinaryOperator<T, Output = T>,
) {
    res.clear();

    let mut v1_it = v1.iter().peekable();
    let mut v2_it = v2.iter().peekable();

    while v1_it.peek().is_some() || v2_it.peek().is_some() {
        let value = match (v1_it.peek(), v2_it.peek()) {
            (Some((i1, v1)), Some((i2, v2))) => match i2.cmp(i1) {
                Ordering::Equal => {
                    v1_it.next();
                    v2_it.next();
                    (*i1, bin_op.op(v1.clone(), v2.clone()))
                }
                Ordering::Greater => {
                    v1_it.next();
                    (*i1, v1.clone())
                }
                Ordering::Less => {
                    v2_it.next();
                    (*i2, v2.clone())
                }
            },
            (Some((i1, v1)), None) => {
                v1_it.next();
                (*i1, v1.clone())
            }
            (None, Some((i2, v2))) => {
                v2_it.next();
                (*i2, v2.clone())
            }
            (None, None) => unreachable!(),
        };

        res.push(value);
    }
}
