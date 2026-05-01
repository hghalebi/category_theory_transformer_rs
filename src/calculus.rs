use crate::error::{CtError, CtResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scalar(f32);

impl Scalar {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() {
            return Err(CtError::InvalidLoss(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocalGradient(f32);

impl LocalGradient {
    pub fn new(value: f32) -> CtResult<Self> {
        if !value.is_finite() {
            return Err(CtError::InvalidLoss(value));
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MulOp;

impl MulOp {
    pub fn forward(&self, x: Scalar, y: Scalar) -> CtResult<Scalar> {
        Scalar::new(x.value() * y.value())
    }

    /// Given upstream gradient dL/dz, return `(dL/dx, dL/dy)` for `z = x * y`.
    pub fn backward(
        &self,
        x: Scalar,
        y: Scalar,
        upstream: LocalGradient,
    ) -> CtResult<(LocalGradient, LocalGradient)> {
        let dz_dx = y.value();
        let dz_dy = x.value();

        Ok((
            LocalGradient::new(upstream.value() * dz_dx)?,
            LocalGradient::new(upstream.value() * dz_dy)?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_backward_returns_local_chain_rule_gradients() -> CtResult<()> {
        let mul = MulOp;
        let x = Scalar::new(2.0)?;
        let y = Scalar::new(3.0)?;
        let upstream = LocalGradient::new(1.0)?;
        let (dl_dx, dl_dy) = mul.backward(x, y, upstream)?;

        assert_eq!(dl_dx.value(), 3.0);
        assert_eq!(dl_dy.value(), 2.0);
        Ok(())
    }
}
