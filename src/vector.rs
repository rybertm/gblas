use crate::{
    algebra::BinaryOperator,
    types::{IndexType, NoValue},
    GblasResult,
};

/// Frontend Vector.
pub trait Vector: Clone + PartialEq + Sized {
    type Scalar;

    fn new(size: IndexType) -> GblasResult<Self>;
    fn dup(&self) -> GblasResult<Self> {
        Ok(self.clone())
    }
    fn resize(&mut self, size: IndexType) -> GblasResult<NoValue>;
    fn clear(&mut self) -> GblasResult<NoValue>;
    fn size(&self) -> IndexType;
    fn nvals(&self) -> IndexType;
    fn build(
        self,
        indices: impl Iterator<Item = IndexType>,
        values: impl Iterator<Item = Self::Scalar>,
        n: IndexType,
        dup: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
    ) -> GblasResult<Self>;
    fn set_element(&mut self, index: IndexType, val: Self::Scalar) -> GblasResult<NoValue>;
    fn remove_element(&mut self, index: IndexType) -> GblasResult<NoValue>;
    fn extract_element(&self, index: IndexType) -> GblasResult<&Self::Scalar>;
    fn extract_tuples(self) -> GblasResult<(Vec<IndexType>, Vec<Self::Scalar>)>;
}

pub trait VectorUtils: Vector {
    fn iter(&self) -> impl Iterator<Item = (IndexType, &Self::Scalar)> {
        // TODO: test perf and check if it's better to change the default implementation
        (0..self.size()).filter_map(move |i| self.extract_element(i).map(|val| (i, val)).ok())
    }
}
