use crate::{
    algebra::{BinaryOperator, Monoid, Semiring, UnaryOperator},
    descriptor::Descriptor,
    indices::Indices,
    mask::{MatMask, VecMask},
    matrix::Matrix,
    types::{IndexType, NoValue},
    vector::Vector,
    GblasResult,
};

pub trait MatOps: Matrix {
    fn mxm(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_binary_op(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_monoid(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_semiring(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_binary_op(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_monoid(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_semiring(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn extract(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        row_indices: Indices,
        col_indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn assign(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        row_indices: Indices,
        col_indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn assign_col(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        row_indices: Indices,
        col_index: IndexType,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn assign_row(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        row_index: IndexType,
        col_indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    // TODO(robert): rename to `assign_udt`? (https://graphblas.org/docs/GraphBLAS_API_C_v1.3.0.pdf#table.5.5)
    fn assign_value(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        value: Self::Scalar,
        row_indices: Indices,
        col_indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl UnaryOperator<Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply_1st(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        value: Self::Scalar,
        a: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply_2nd(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        value: Self::Scalar,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn reduce(
        &self,
        val: &mut Self::Scalar,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn transpose(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_binary_op(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_monoid(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_semiring(
        &mut self,
        mask: Option<impl MatMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        b: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;
}

// ---------------------------------------------------------------------------

pub trait VecOps: Vector {
    fn vxm(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        a: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn mxv(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_binary_op(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_monoid(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_mult_semiring(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_binary_op(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_monoid(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn e_wise_add_semiring(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        v: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn extract(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        u: &impl Vector<Scalar = Self::Scalar>,
        indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn extract_col(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        row_indices: Indices,
        col_index: IndexType,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn assign(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        u: &impl Vector<Scalar = Self::Scalar>,
        indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    // TODO(robert): rename to `assign_udt`? (https://graphblas.org/docs/GraphBLAS_API_C_v1.3.0.pdf#table.5.5)
    fn assign_value(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        value: Self::Scalar,
        indices: Indices,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl UnaryOperator<Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply_1st(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        value: Self::Scalar,
        u: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn apply_2nd(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        u: &impl Vector<Scalar = Self::Scalar>,
        value: Self::Scalar,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn reduce(
        &self,
        val: &mut Self::Scalar,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn reduce_binary_op(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn reduce_monoid(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        a: &impl Matrix<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn transpose(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        a: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_binary_op(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
        a: &impl Vector<Scalar = Self::Scalar>,
        b: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_monoid(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Monoid<Self::Scalar>,
        a: &impl Vector<Scalar = Self::Scalar>,
        b: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;

    fn kronecker_semiring(
        &mut self,
        mask: Option<impl VecMask<Self>>,
        accum: Option<impl BinaryOperator<Self::Scalar, Output = Self::Scalar>>,
        op: impl Semiring<Self::Scalar, Output = Self::Scalar>,
        a: &impl Vector<Scalar = Self::Scalar>,
        b: &impl Vector<Scalar = Self::Scalar>,
        desc: Option<Descriptor>,
    ) -> GblasResult<NoValue>;
}
