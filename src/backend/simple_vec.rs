use crate::{
    algebra::{BinaryOperator, First},
    types::{IndexType, NoValue},
    vector::Vector,
    ApiError, ExecutionError, GblasResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleVec<T> {
    pub data: Vec<(IndexType, T)>,
    pub size: IndexType,
}

impl<T> SimpleVec<T>
where
    T: Clone + PartialEq,
{
    /// Since the error for out of bounds can change
    /// we return `None` here so the caller can decide
    fn set_element_dup(
        &mut self,
        index: IndexType,
        val: T,
        dup: Option<&impl BinaryOperator<T, Output = T>>,
    ) -> Option<NoValue> {
        if index >= self.size {
            return None;
        }

        let data = &mut self.data;

        if data.is_empty() {
            data.push((index, val));
        } else {
            let mut val = val;
            let found = data
                .iter()
                .enumerate()
                .find_map(|(i, (idx, v))| match idx.cmp(&index) {
                    std::cmp::Ordering::Equal => {
                        if let Some(bin) = dup {
                            val = bin.op(v.clone(), val.clone());
                        }
                        Some((i, true))
                    }
                    std::cmp::Ordering::Greater => Some((i, false)),
                    std::cmp::Ordering::Less => None,
                });
            match found {
                Some((i, true)) => {
                    data[i].1 = val;
                }
                Some((i, false)) => {
                    data.insert(i, (index, val));
                }
                None => {
                    data.push((index, val));
                }
            }
        }

        Some(())
    }
}

impl<T> Vector for SimpleVec<T>
where
    T: Clone + PartialEq,
{
    type Scalar = T;

    fn new(size: IndexType) -> GblasResult<Self> {
        if size == 0 {
            return Err(ApiError::InvalidValue.into());
        }
        Ok(Self {
            data: Vec::with_capacity(size),
            size,
        })
    }

    fn dup(&self) -> GblasResult<Self> {
        Ok(self.clone())
    }

    fn resize(&mut self, size: IndexType) -> GblasResult<NoValue> {
        if size == 0 {
            return Err(ApiError::InvalidValue.into());
        }

        self.data.reserve(size);
        if size < self.size {
            self.data.retain(|(i, _)| *i < size);
        }
        self.size = size;

        Ok(())
    }

    fn clear(&mut self) -> GblasResult<NoValue> {
        self.data.clear();
        Ok(())
    }

    fn size(&self) -> IndexType {
        self.size
    }

    fn nvals(&self) -> IndexType {
        self.data.len()
    }

    fn build(
        self,
        indices: impl Iterator<Item = IndexType>,
        values: impl Iterator<Item = Self::Scalar>,
        _: IndexType,
        dup: impl BinaryOperator<Self::Scalar, Output = Self::Scalar>,
    ) -> GblasResult<Self> {
        if self.nvals() == 0 {
            return Err(ApiError::OutputNotEmpty.into());
        }

        let mut data = self;
        // TODO: check if rows, cols and values sizes match `n`
        // TODO: try better implementation?
        for (idx, val) in indices.zip(values) {
            let res = data.set_element_dup(idx, val, Some(&dup));
            if res.is_none() {
                return Err(ExecutionError::IndexOutOfBounds.into());
            }
        }

        Ok(data)
    }

    fn set_element(&mut self, index: IndexType, val: Self::Scalar) -> GblasResult<NoValue> {
        self.set_element_dup(index, val, Option::<&First<_>>::None)
            .ok_or_else(|| ApiError::InvalidIndex.into())
    }

    fn remove_element(&mut self, index: IndexType) -> GblasResult<NoValue> {
        if index >= self.size() {
            return Err(ApiError::InvalidIndex.into());
        }

        let data = &mut self.data;

        if data.is_empty() {
            Ok(())
        } else {
            let found = data.iter().position(|(idx, _)| *idx == index);

            if let Some(idx) = found {
                data.remove(idx);
            }

            Ok(())
        }
    }

    fn extract_element(&self, index: IndexType) -> GblasResult<&Self::Scalar> {
        if index >= self.size() {
            return Err(ApiError::InvalidIndex.into());
        }

        let data = &self.data;

        if data.is_empty() {
            Err(ApiError::NoValue.into())
        } else {
            data.iter()
                .find_map(|(idx, val)| if *idx == index { Some(val) } else { None })
                .ok_or_else(|| ApiError::NoValue.into())
        }
    }

    fn extract_tuples(self) -> GblasResult<(Vec<IndexType>, Vec<Self::Scalar>)> {
        let len = self.data.len();
        let mut idxs = Vec::with_capacity(len);
        let mut vals = Vec::with_capacity(len);
        for (idx, val) in self.data {
            idxs.push(idx);
            vals.push(val);
        }

        Ok((idxs, vals))
    }
}
