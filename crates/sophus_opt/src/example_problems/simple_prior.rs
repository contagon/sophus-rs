use crate::example_problems::cost_fn::isometry2_prior::Isometry2PriorCostFn;
use crate::example_problems::cost_fn::isometry2_prior::Isometry2PriorTerm;
use crate::example_problems::cost_fn::isometry3_prior::Isometry3PriorCostFn;
use crate::example_problems::cost_fn::isometry3_prior::Isometry3PriorTerm;
use crate::nlls::optimize;
use crate::nlls::OptParams;
use crate::prelude::*;
use crate::quadratic_cost::cost_fn::CostFn;
use crate::quadratic_cost::term::Terms;
use crate::variables::VarFamily;
use crate::variables::VarKind;
use crate::variables::VarPoolBuilder;
use sophus_autodiff::linalg::MatF64;
use sophus_autodiff::linalg::VecF64;
use sophus_lie::Isometry2F64;
use sophus_lie::Isometry3F64;

extern crate alloc;

/// Simple 2D isometry prior problem
pub struct SimpleIso2PriorProblem {
    /// True world from robot isometry
    pub true_world_from_robot: Isometry2F64,
    /// Estimated world from robot isometry
    pub est_world_from_robot: Isometry2F64,
}

impl Default for SimpleIso2PriorProblem {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleIso2PriorProblem {
    fn new() -> Self {
        let p = VecF64::<3>::from_f64_array([0.2, 0.0, 1.0]);
        let true_world_from_robot = Isometry2F64::exp(p);
        Self {
            true_world_from_robot,
            est_world_from_robot: Isometry2F64::identity(),
        }
    }

    /// Test the simple 2D isometry prior problem
    pub fn test(&self) {
        use sophus_autodiff::linalg::EPS_F64;
        let cost_term = alloc::vec![Isometry2PriorTerm {
            isometry_prior_mean: self.true_world_from_robot,
            entity_indices: [0],
        }];

        let obs_pose_a_from_pose_b_poses =
            Terms::<1, Isometry2F64, Isometry2PriorTerm>::new(["poses".into()], cost_term);

        let family: VarFamily<Isometry2F64> =
            VarFamily::new(VarKind::Free, alloc::vec![self.est_world_from_robot]);

        let families = VarPoolBuilder::new().add_family("poses", family).build();

        approx::assert_abs_diff_ne!(
            self.true_world_from_robot.compact(),
            self.est_world_from_robot.compact(),
            epsilon = EPS_F64
        );

        let up_families = optimize(
            families,
            alloc::vec![CostFn::new_box(
                (),
                obs_pose_a_from_pose_b_poses.clone(),
                Isometry2PriorCostFn {},
            )],
            OptParams {
                num_iter: 1,            // should converge in single iteration
                initial_lm_nu: EPS_F64, // if lm prior param is tiny
                parallelize: true,
                ..Default::default()
            },
        )
        .unwrap();
        let refined_world_from_robot = up_families.get_members::<Isometry2F64>("poses".into());

        approx::assert_abs_diff_eq!(
            self.true_world_from_robot.compact(),
            refined_world_from_robot[0].compact(),
            epsilon = EPS_F64
        );
    }
}

/// Simple 3D isometry prior problem
pub struct SimpleIso3PriorProblem {
    /// True world from robot isometry
    pub true_world_from_robot: Isometry3F64,
    /// Estimated world from robot isometry
    pub est_world_from_robot: Isometry3F64,
}

impl Default for SimpleIso3PriorProblem {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleIso3PriorProblem {
    fn new() -> Self {
        let p = VecF64::<6>::from_real_array([0.2, 0.0, 1.0, 0.2, 0.0, 1.0]);
        let true_world_from_robot = Isometry3F64::exp(p);
        Self {
            true_world_from_robot,
            est_world_from_robot: Isometry3F64::identity(),
        }
    }

    /// Test the simple 3D isometry prior problem
    pub fn test(&self) {
        use sophus_autodiff::linalg::EPS_F64;

        let cost_term = alloc::vec![Isometry3PriorTerm {
            isometry_prior: (self.true_world_from_robot, MatF64::<6, 6>::identity()),
            entity_indices: [0],
        }];

        let obs_pose_a_from_pose_b_poses = Terms::<
            1,
            (Isometry3F64, MatF64<6, 6>),
            Isometry3PriorTerm,
        >::new(["poses".into()], cost_term);

        let family: VarFamily<Isometry3F64> =
            VarFamily::new(VarKind::Free, alloc::vec![self.est_world_from_robot]);

        let families = VarPoolBuilder::new().add_family("poses", family).build();

        approx::assert_abs_diff_ne!(
            self.true_world_from_robot.compact(),
            self.est_world_from_robot.compact(),
            epsilon = EPS_F64
        );

        let up_families = optimize(
            families,
            alloc::vec![CostFn::new_box(
                (),
                obs_pose_a_from_pose_b_poses.clone(),
                Isometry3PriorCostFn {},
            )],
            OptParams {
                num_iter: 1,            // should converge in single iteration
                initial_lm_nu: EPS_F64, // if lm prior param is tiny
                parallelize: true,
                linear_solver: crate::nlls::LinearSolverType::NalgebraDenseFullPiVLu,
                error_tol_relative: 1e-6,
                error_tol_absolute: 1e-6,
                error_tol: 0.0,
            },
        )
        .unwrap();
        let refined_world_from_robot = up_families.get_members::<Isometry3F64>("poses".into());

        approx::assert_abs_diff_eq!(
            self.true_world_from_robot.compact(),
            refined_world_from_robot[0].compact(),
            epsilon = EPS_F64
        );
    }
}

#[test]
fn simple_prior_opt_tests() {
    SimpleIso2PriorProblem::new().test();
    SimpleIso3PriorProblem::new().test();
}
