use crate::{
    algebra::BinaryOperator,
    types::{IndexType, NoValue},
    GblasResult,
};

/// Frontend Matrix.
pub trait Matrix: Clone + PartialEq + Sized {
    type Scalar;

    fn new(rows: IndexType, cols: IndexType) -> GblasResult<Self>;
    fn dup(&self) -> GblasResult<Self> {
        Ok(self.clone())
    }
    fn resize(&mut self, rows: IndexType, cols: IndexType) -> GblasResult<NoValue>;
    fn clear(&mut self) -> GblasResult<NoValue>;
    fn nrows(&self) -> IndexType;
    fn ncols(&self) -> IndexType;
    fn nvals(&self) -> IndexType;
    fn build(
        self,
        rows: impl Iterator<Item = IndexType>,
        cols: impl Iterator<Item = IndexType>,
        values: impl Iterator<Item = Self::Scalar>,
        n: IndexType,
        dup: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
    ) -> GblasResult<Self>;
    fn set_element(
        &mut self,
        row: IndexType,
        col: IndexType,
        value: Self::Scalar,
    ) -> GblasResult<NoValue>;
    fn remove_element(&mut self, row: IndexType, col: IndexType) -> GblasResult<NoValue>;
    fn extract_element(&self, row: IndexType, col: IndexType) -> GblasResult<Self::Scalar>;
    fn extract_tuples(&self) -> GblasResult<(Vec<IndexType>, Vec<IndexType>, Vec<Self::Scalar>)>;
}

pub trait MatrixExtra: Matrix {
    fn iter(&self) -> impl Iterator<Item = (IndexType, IndexType, Self::Scalar)> {
        // TODO(robert): test perf and check if it's better to change the default implementation
        (0..self.nrows())
            .flat_map(move |i| (0..self.ncols()).map(move |j| (i, j)))
            .filter_map(move |(i, j)| self.extract_element(i, j).map(|val| (i, j, val)).ok())
    }

    fn index_iter(&self) -> impl Iterator<Item = (IndexType, IndexType)> {
        (0..self.nrows()).flat_map(move |i| (0..self.ncols()).map(move |j| (i, j)))
    }
}
