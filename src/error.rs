use thiserror::Error;

#[derive(Debug, Error)]
pub enum GblasError {
    #[error("ApiError({0})")]
    ApiError(#[from] ApiError),
    #[error("ExecutionError({0})")]
    ExecutionError(#[from] ExecutionError),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("UninitializedObject: A GraphBLAS object is passed to a method before new was called on it.")]
    UninitializedObject,
    #[error("NullPointer: A NULL is passed for a pointer parameter.")]
    NullPointer,
    #[error("InvalidValue: Miscellaneous incorrect values.")]
    InvalidValue,
    #[error("InvalidIndex: Indices passed are larger than dimensions of the matrix or vector being accessed.")]
    InvalidIndex,
    #[error("DomainMismatch: A mismatch between domains of collections and operations when user-defined domains are in use.")]
    DomainMismatch,
    #[error("DimensionMismatch: Operations on matrices and vectors with incompatible dimensions.")]
    DimensionMismatch,
    #[error("OutputNotEmpty: An attempt was made to build a matrix or vector using an output object that already contains valid tuples (elements).")]
    OutputNotEmpty,
    #[error("NoValue: A location in a matrix or vector is being accessed that has no stored value at the specified location.")]
    NoValue,
}

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("OutOfMemory: Not enough memory for operations.")]
    OutOfMemory,
    #[error("InsufficientSpace: The array provided is not large enough to hold output.")]
    InsufficientSpace,
    #[error("InvalidObject: One of the opaque GraphBLAS objects (input or output) is in an invalid state caused by a previous execution error.")]
    InvalidObject,
    #[error("IndexOutOfBounds: Reference to a vector or matrix element that is outside the defined dimensions of the object.")]
    IndexOutOfBounds,
    #[error("Panic: Unknown internal error.")]
    Panic,
}
