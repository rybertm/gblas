use std::ops::Index;

use crate::{matrix::Matrix, types::IndexType, vector::Vector};

pub struct MatrixStructureMask<'a, M> {
    pub(crate) mat: &'a M,
}

impl<'a, M> MatrixStructureMask<'a, M> {
    pub fn new(mat: &'a M) -> Self {
        Self { mat }
    }
}

impl<'a, M> MatrixStructureMask<'a, M>
where
    M: Matrix,
{
    pub fn ncols(&self) -> IndexType {
        self.mat.ncols()
    }

    pub fn nrows(&self) -> IndexType {
        self.mat.nrows()
    }
}

impl<M> Index<(IndexType, IndexType)> for MatrixStructureMask<'_, M>
where
    M: Matrix + Index<(IndexType, IndexType), Output = bool>,
{
    type Output = bool;

    fn index(&self, index: (IndexType, IndexType)) -> &Self::Output {
        if index.0 >= self.mat.nrows() || index.1 >= self.mat.ncols() {
            return &false;
        }

        self.mat.index(index)
    }
}

// --------------------------------------------------------------------------------

pub struct VectorStructureMask<'a, V> {
    pub(crate) vector: &'a V,
}

impl<'a, V> VectorStructureMask<'a, V> {
    pub fn new(vector: &'a V) -> Self {
        Self { vector }
    }
}

impl<'a, V> VectorStructureMask<'a, V>
where
    V: Vector,
{
    pub fn size(&self) -> IndexType {
        self.vector.size()
    }
}

impl<V> Index<IndexType> for VectorStructureMask<'_, V>
where
    V: Vector + Index<IndexType, Output = bool>,
{
    type Output = bool;

    fn index(&self, index: IndexType) -> &Self::Output {
        if index >= self.vector.size() {
            return &false;
        }

        self.vector.index(index)
    }
}
