use faer::{
    prelude::SpSolver,
    sparse::FaerError,
};

use super::{
    IsSparseSymmetricLinearSystem,
    NllsError,
    SparseSolverError,
};
use crate::block::symmetric_block_sparse_matrix_builder::SymmetricBlockSparseMatrixBuilder;

/// Sparse LU solver
///
/// Sparse LU decomposition - wrapper around faer's sp_lu implementation.
pub struct SparseLu;

impl IsSparseSymmetricLinearSystem for SparseLu {
    fn solve(
        &self,
        upper_triangular: &SymmetricBlockSparseMatrixBuilder,
        b: &nalgebra::DVector<f64>,
    ) -> Result<nalgebra::DVector<f64>, NllsError> {
        let dim = upper_triangular.scalar_dimension();
        let csr = faer::sparse::SparseColMat::try_new_from_triplets(
            dim,
            dim,
            &upper_triangular.to_symmetric_scalar_triplets(),
        )
        .unwrap();
        let mut x = b.clone();
        let x_slice_mut = x.as_mut_slice();
        let mut x_ref = faer::mat::from_column_major_slice_mut(x_slice_mut, dim, 1);

        match csr.sp_lu() {
            Ok(lu) => {
                lu.solve_in_place(faer::reborrow::ReborrowMut::rb_mut(&mut x_ref));
                Ok(x)
            }
            Err(e) => Err(NllsError::SparseLuError {
                details: match e {
                    faer::sparse::LuError::Generic(faer_error) => match faer_error {
                        FaerError::IndexOverflow => SparseSolverError::IndexOverflow,
                        FaerError::OutOfMemory => SparseSolverError::OutOfMemory,
                        _ => SparseSolverError::Unspecific,
                    },
                    faer::sparse::LuError::SymbolicSingular(_) => {
                        SparseSolverError::SymbolicSingular
                    }
                },
            }),
        }
    }
}
