use na::{RealField, Unit, UnitQuaternion};

use crate::aliases::{Qua, TVec3};

/// Computes the quaternion exponential.
pub fn quat_exp<N: RealField>(q: &Qua<N>) -> Qua<N> {
    q.exp()
}

/// Computes the quaternion logarithm.
pub fn quat_log<N: RealField>(q: &Qua<N>) -> Qua<N> {
    q.ln()
}

/// Raises the quaternion `q` to the power `y`.
pub fn quat_pow<N: RealField>(q: &Qua<N>, y: N) -> Qua<N> {
    q.powf(y)
}

/// Builds a quaternion from an axis and an angle, and right-multiply it to the quaternion `q`.
pub fn quat_rotate<N: RealField>(q: &Qua<N>, angle: N, axis: &TVec3<N>) -> Qua<N> {
    q * UnitQuaternion::from_axis_angle(&Unit::new_normalize(*axis), angle).into_inner()
}

//pub fn quat_sqrt<N: RealField>(q: &Qua<N>) -> Qua<N> {
//    unimplemented!()
//}
