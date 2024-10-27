use crate::types::IndexType;

#[derive(Debug, Clone, PartialEq)]
pub struct SparseMatrix<T> {
    mat: Vec<Vec<(IndexType, T)>>,
    nrows: IndexType,
    ncols: IndexType,
    nvals: IndexType,
}

mod matrix_impl;
