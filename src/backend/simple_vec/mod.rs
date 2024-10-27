use crate::types::IndexType;

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleVec<T> {
    pub data: Vec<(IndexType, T)>,
    pub size: IndexType,
}

mod vector_impl;
