use std::ops::Index;

use crate::{
    complement_mask::{MatrixComplementMask, VectorComplementMask},
    matrix::Matrix,
    structure_mask::{MatrixStructureMask, VectorStructureMask},
    types::IndexType,
    vector::Vector,
};

pub trait VecMask<V>: Index<IndexType, Output = bool> {
    fn complement(&self) -> VectorComplementMask<V>;
    fn structure(&self) -> VectorStructureMask<V>;
}

impl<V> VecMask<V> for VectorStructureMask<'_, V>
where
    V: Vector + Index<IndexType, Output = bool>,
{
    fn complement(&self) -> VectorComplementMask<V> {
        VectorComplementMask::new(self.vector)
    }

    fn structure(&self) -> VectorStructureMask<V> {
        VectorStructureMask::new(self.vector)
    }
}

impl<V> VecMask<V> for VectorComplementMask<'_, V>
where
    V: Vector + Index<IndexType, Output = bool>,
{
    fn complement(&self) -> VectorComplementMask<V> {
        VectorComplementMask::new(self.vector)
    }

    fn structure(&self) -> VectorStructureMask<V> {
        VectorStructureMask::new(self.vector)
    }
}

// --------------------------------------------------------------------------------

pub trait MatMask<M>: Index<(IndexType, IndexType), Output = bool> {
    fn complement(&self) -> MatrixComplementMask<M>;
    fn structure(&self) -> MatrixStructureMask<M>;
}

impl<M> MatMask<M> for MatrixStructureMask<'_, M>
where
    M: Matrix + Index<(IndexType, IndexType), Output = bool>,
{
    fn complement(&self) -> MatrixComplementMask<M> {
        MatrixComplementMask::new(self.mat)
    }

    fn structure(&self) -> MatrixStructureMask<M> {
        MatrixStructureMask::new(self.mat)
    }
}

impl<M> MatMask<M> for MatrixComplementMask<'_, M>
where
    M: Matrix + Index<(IndexType, IndexType), Output = bool>,
{
    fn complement(&self) -> MatrixComplementMask<M> {
        MatrixComplementMask::new(self.mat)
    }

    fn structure(&self) -> MatrixStructureMask<M> {
        MatrixStructureMask::new(self.mat)
    }
}

