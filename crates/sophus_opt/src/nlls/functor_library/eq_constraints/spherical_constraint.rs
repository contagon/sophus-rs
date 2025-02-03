use sophus_autodiff::{
    dual::{
        DualScalar,
        DualVector,
    },
    linalg::VecF64,
};

use crate::{
    nlls::constraint::eq_constraint::IsEqConstraint,
    prelude::*,
};

/// spherical equality constraint
#[derive(Clone, Debug)]
pub struct SphericalConstraint {
    /// sphere radius
    pub radius: f64,
    /// entity index
    pub entity_indices: [usize; 1],
}

impl SphericalConstraint {
    /// Compute the residual
    pub fn residual<Scalar: IsSingleScalar<DM, DN>, const DM: usize, const DN: usize>(
        vec: Scalar::Vector<3>,
        radius: Scalar,
    ) -> Scalar::Vector<1> {
        let norm = vec.norm();
        Scalar::Vector::<1>::from_array([norm - radius])
    }
}

impl IsEqConstraint<1, 3, 1, (), VecF64<3>> for SphericalConstraint {
    fn idx_ref(&self) -> &[usize; 1] {
        &self.entity_indices
    }

    fn eval(
        &self,
        _global_constants: &(),
        idx: [usize; 1],
        vec3: VecF64<3>,
        var_kinds: [crate::variables::VarKind; 1],
    ) -> crate::nlls::constraint::evaluated_eq_constraint::EvaluatedEqConstraint<1, 3, 1> {
        let residual = Self::residual(vec3, self.radius);
        let dx_res_fn = |x: DualVector<3, 3, 1>| -> DualVector<1, 3, 1> {
            let radius_dual = DualScalar::from_f64(self.radius);
            Self::residual::<DualScalar<3, 1>, 3, 1>(x, radius_dual)
        };

        (|| dx_res_fn(DualVector::var(vec3)).jacobian(),).make_eq(idx, var_kinds, residual)
    }
}
