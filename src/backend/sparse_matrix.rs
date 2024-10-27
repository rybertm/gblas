use std::{cmp::Ordering, ops::Index};

use crate::{
    algebra::{BinaryOperator, First},
    complement_mask::MatrixComplementMask,
    mask::MatMask,
    matrix::{Matrix, MatrixExtra},
    structure_mask::MatrixStructureMask,
    types::{IndexType, NoValue},
    ApiError, ExecutionError, GblasResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SparseMatrix<T> {
    mat: Vec<Vec<(IndexType, T)>>,
    nrows: IndexType,
    ncols: IndexType,
    nvals: IndexType,
}

impl<T> SparseMatrix<T>
where
    T: Clone + PartialEq,
{
    /// Since the error for out of bounds can change
    /// we return `None` here so the caller can decide
    fn set_element_dup(
        &mut self,
        row: IndexType,
        col: IndexType,
        value: T,
        dup: Option<&impl BinaryOperator<T, Output = T>>,
    ) -> Option<NoValue> {
        if row >= self.nrows || col >= self.ncols {
            return None;
        }

        let data = if let Some(data) = self.mat.get_mut(row) {
            data
        } else {
            // prevents panics if e.g Mat is 5x5 and we try to set element at (3, 1)
            // without having set any elements at row 3 or before
            if row >= self.mat.len() {
                self.mat.resize_with(row + 1, Vec::new);
            }
            self.mat.insert(row, vec![(col, value)]);
            self.nvals += 1;
            return Some(());
        };

        if data.is_empty() {
            data.push((col, value));
            self.nvals += 1;
        } else {
            let mut value = value;
            let found = data
                .iter()
                .enumerate()
                .find_map(|(idx, (c, v))| match c.cmp(&col) {
                    Ordering::Equal => {
                        if let Some(bin) = dup {
                            value = bin.op(v.clone(), value.clone());
                        }
                        Some((idx, true))
                    }
                    Ordering::Greater => Some((idx, false)),
                    Ordering::Less => None,
                });
            match found {
                Some((idx, true)) => {
                    data[idx].1 = value;
                }
                Some((idx, false)) => {
                    data.insert(idx, (col, value));
                    self.nvals += 1;
                }
                None => {
                    data.push((col, value));
                    self.nvals += 1;
                }
            }
        }

        Some(())
    }
}

impl<T> Matrix for SparseMatrix<T>
where
    T: Clone + PartialEq,
{
    type Scalar = T;

    fn new(rows: IndexType, cols: IndexType) -> GblasResult<Self> {
        if rows == 0 || cols == 0 {
            return Err(ApiError::InvalidValue.into());
        }

        Ok(Self {
            mat: Vec::with_capacity(rows),
            nrows: rows,
            ncols: cols,
            nvals: 0,
        })
    }

    fn resize(&mut self, rows: IndexType, cols: IndexType) -> GblasResult<NoValue> {
        if rows == 0 || cols == 0 {
            return Err(ApiError::InvalidValue.into());
        }

        self.mat.reserve(rows);

        if rows < self.nrows {
            self.mat = self
                .mat
                .iter_mut()
                .enumerate()
                .filter_map(|(idx, row)| {
                    if idx < rows {
                        Some(core::mem::take(row))
                    } else {
                        self.nvals -= row.len();
                        None
                    }
                })
                .collect();
        }
        self.nrows = rows;

        if cols < self.ncols {
            for row in self.mat.iter_mut() {
                if !row.is_empty() {
                    // remove all elements that are out of bounds for new col
                    let sz = row.len();
                    row.retain(|(col, _)| *col < cols);
                    let diff = sz - row.len();
                    self.nvals -= diff;
                }
            }
        }
        self.ncols = cols;

        Ok(())
    }

    fn clear(&mut self) -> GblasResult<NoValue> {
        self.nvals = 0;
        // TODO: Should we clear the row mat vec? Probably not since the outer vec only hold row indices
        for row in self.mat.iter_mut() {
            row.clear();
        }
        Ok(())
    }

    fn nrows(&self) -> IndexType {
        self.nrows
    }

    fn ncols(&self) -> IndexType {
        self.ncols
    }

    fn nvals(&self) -> IndexType {
        self.nvals
    }

    fn build(
        self,
        rows: impl Iterator<Item = IndexType>,
        cols: impl Iterator<Item = IndexType>,
        values: impl Iterator<Item = Self::Scalar>,
        _: IndexType,
        dup: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
    ) -> GblasResult<Self> {
        if self.nvals > 0 {
            return Err(ApiError::OutputNotEmpty.into());
        }

        let mut s = self;
        // TODO: check if rows, cols and values sizes match `n`
        // TODO: try better implementation?
        for ((row, col), value) in rows.zip(cols).zip(values) {
            let res = s.set_element_dup(row, col, value, Some(&dup));
            if res.is_none() {
                return Err(ExecutionError::IndexOutOfBounds.into());
            }
        }
        Ok(s)
    }

    fn set_element(
        &mut self,
        row: IndexType,
        col: IndexType,
        value: Self::Scalar,
    ) -> GblasResult<NoValue> {
        self.set_element_dup(row, col, value, Option::<&First<_>>::None)
            .ok_or_else(|| ApiError::InvalidIndex.into())
    }

    fn remove_element(&mut self, row: IndexType, col: IndexType) -> GblasResult<NoValue> {
        if row >= self.nrows || col >= self.ncols {
            return Err(ApiError::InvalidIndex.into());
        }
        let data = if let Some(data) = self.mat.get_mut(row) {
            data
        } else {
            return Ok(());
        };

        if data.is_empty() {
            Ok(())
        } else {
            let found = data.iter().position(|(c, _)| *c == col);
            if let Some(idx) = found {
                data.remove(idx);
                self.nvals -= 1;
            }
            Ok(())
        }
    }

    fn extract_element(&self, row: IndexType, col: IndexType) -> GblasResult<Self::Scalar> {
        if row >= self.nrows || col >= self.ncols {
            return Err(ApiError::InvalidIndex.into());
        }
        let data = self.mat.get(row).ok_or(ApiError::NoValue)?;

        if data.is_empty() {
            Err(ApiError::NoValue.into())
        } else {
            data.iter()
                .find_map(|(c, v)| if *c == col { Some(v.clone()) } else { None })
                .ok_or_else(|| ApiError::NoValue.into())
        }
    }

    fn extract_tuples(&self) -> GblasResult<(Vec<IndexType>, Vec<IndexType>, Vec<Self::Scalar>)> {
        let mut rows = Vec::with_capacity(self.nvals);
        let mut cols = Vec::with_capacity(self.nvals);
        let mut values = Vec::with_capacity(self.nvals);

        for (row, data) in self.mat.iter().enumerate() {
            for (col, value) in data.iter() {
                rows.push(row);
                cols.push(*col);
                values.push(value.clone());
            }
        }

        Ok((rows, cols, values))
    }
}

impl<T> MatrixExtra for SparseMatrix<T>
where
    T: Clone + PartialEq,
{
    fn iter(&self) -> impl Iterator<Item = (IndexType, IndexType, Self::Scalar)> {
        self.mat
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().map(move |(j, v)| (i, *j, v.clone())))
    }
}

impl<T> Index<(IndexType, IndexType)> for SparseMatrix<T>
where
    T: Clone + PartialEq,
{
    type Output = bool;

    fn index(&self, index: (IndexType, IndexType)) -> &Self::Output {
        let found = <Self as Matrix>::extract_element(self, index.0, index.1);
        if found.is_ok() {
            &true
        } else {
            &false
        }
    }
}

impl<T> MatMask<Self> for SparseMatrix<T>
where
    T: Clone + PartialEq,
{
    fn complement(&self) -> MatrixComplementMask<Self> {
        MatrixComplementMask::new(self)
    }

    fn structure(&self) -> MatrixStructureMask<Self> {
        MatrixStructureMask::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_new() {
        let mat = SparseMatrix::<f64>::new(3, 3).unwrap();
        assert_eq!(mat.nrows(), 3);
        assert_eq!(mat.ncols(), 3);
        assert_eq!(mat.nvals(), 0);
    }

    #[test]
    fn test_matrix_ops() {
        let mut mat = SparseMatrix::<f64>::new(10, 10).unwrap();
        assert!(mat.set_element(0, 0, 1.0).is_ok());
        assert!(mat.set_element(1, 1, 2.0).is_ok());
        assert!(mat.set_element(1, 2, 3.0).is_ok());
        assert!(mat.set_element(1, 3, 4.0).is_ok());
        assert!(mat.set_element(1, 5, 5.0).is_ok());
        assert!(mat.set_element(1, 5, 6.0).is_ok());
        assert!(mat.set_element(1, 9, 7.0).is_ok());
        assert!(mat.set_element(7, 7, 10.0).is_ok());
        assert!(mat.set_element(2, 9, 7.0).is_ok());
        assert!(mat.set_element(1, 4, 7.0).is_ok());
        assert!(mat.set_element(1, 7, 8.0).is_ok());
        assert!(mat.set_element(4, 2, 9.0).is_ok());
        assert!(mat.set_element(8, 8, 9.0).is_ok());
        assert!(mat.set_element(8, 2, 9.0).is_ok());
        assert!(mat.set_element(8, 3, 9.0).is_ok());
        assert!(mat.set_element(8, 4, 9.0).is_ok());
        assert_eq!(mat.nvals(), 15);
        let elem = mat.extract_element(0, 0);
        assert!(elem.is_ok());
        assert_eq!(elem.unwrap(), 1.0);
        let elem = mat.extract_element(1, 1);
        assert!(elem.is_ok());
        assert_eq!(elem.unwrap(), 2.0);
        let elem = mat.extract_element(1, 2);
        assert!(elem.is_ok());
        assert_eq!(elem.unwrap(), 3.0);
        let elem = mat.extract_element(1, 7);
        assert!(elem.is_ok());
        assert_eq!(elem.unwrap(), 8.0);
        let elem = mat.extract_element(3, 3);
        assert!(elem.is_err());

        let res = mat.resize(7, 7);
        assert!(res.is_ok());
        let elem = mat.extract_element(1, 5);
        assert!(elem.is_ok());
        assert_eq!(elem.unwrap(), 6.0);
        let elem = mat.extract_element(2, 9);
        assert!(elem.is_err());
        let elem = mat.extract_element(8, 8);
        assert!(elem.is_err());

        assert_eq!(mat.nvals(), 7);

        let res = mat.remove_element(1, 5);
        assert!(res.is_ok());
        let elem = mat.extract_element(1, 5);
        assert!(elem.is_err());

        assert!(mat.extract_tuples().is_ok());

        assert!(mat[(0, 0)]);
        assert!(mat[(4, 2)]);
        assert!(!mat[(4, 3)]);
        assert!(!mat[(1, 9)]);
        assert!(!mat[(15, 15)]);
    }

    #[test]
    fn test_masks() {
        let mut mat = SparseMatrix::<f64>::new(10, 10).unwrap();
        mat.set_element(0, 0, 1.0).unwrap();
        mat.set_element(1, 1, 2.0).unwrap();
        mat.set_element(1, 2, 3.0).unwrap();
        mat.set_element(1, 3, 4.0).unwrap();
        mat.set_element(1, 5, 5.0).unwrap();
        mat.set_element(1, 9, 7.0).unwrap();
        mat.set_element(7, 7, 10.0).unwrap();
        mat.set_element(2, 9, 7.0).unwrap();
        mat.set_element(1, 7, 8.0).unwrap();
        mat.set_element(4, 2, 9.0).unwrap();
        mat.set_element(8, 8, 9.0).unwrap();

        let str_mask = mat.structure();
        let comp_mask = mat.complement();

        assert_eq!(mat[(0, 0)], str_mask[(0, 0)]);
        assert_eq!(mat[(1, 1)], !comp_mask[(1, 1)]);
    }
}
