use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

const M0: f32 = 0.2969;
const M1: f32 = -0.1260;
const M2: f32 = -0.3516;
const M3: f32 = 0.2843;
const M4: f32 = -0.1015;

#[derive(Debug, Error)]
pub enum NACAError {
    #[error("Input Length is wrong")]
    InvalidLen,
    #[error("Parser error: `{0}`")]
    ParseError(#[from] ParseIntError),
}

pub struct NACA4 {
    m: f32,
    p: f32,
    t: f32,
}

impl NACA4 {
    pub fn yt(&self, x: f32) -> f32 {
        let t = self.t;
        let x0 = M0 * x.sqrt();
        let x1 = M1 * x;
        let x2 = M2 * x.powi(2);
        let x3 = M3 * x.powi(3);
        let x4 = M4 * x.powi(4);
        (x0 + x1 + x2 + x3 + x4) * 5.0 * t
    }

    pub fn yc(&self, x: f32) -> f32 {
        if x < self.p {
            (2.0 * self.p - x) * self.m * x / self.p.powi(2)
        } else {
            (x + 1.0 - 2.0 * self.p) * self.m * (1.0 - x) / (1.0 - self.p).powi(2)
        }
    }

    fn theta(&self, x: f32) -> f32 {
        let dyc = if x < self.p {
            2.0 * self.m * (self.p - x) / self.p.powi(2)
        } else {
            2.0 * self.m * (self.p - x) / (1.0 - self.p.powi(2))
        };
        dyc.atan()
    }
}

impl FromStr for NACA4 {
    type Err = NACAError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(NACAError::InvalidLen);
        }
        let m: u8 = s[..1].parse()?;
        let p: u8 = s[1..2].parse()?;
        let t: u8 = s[2..4].parse()?;
        let m = m as f32 / 100.0;
        let p = p as f32 / 10.0;
        let t = t as f32 / 100.0;
        Ok(Self { m, p, t })
    }
}

/// x should always in 0..=1
pub trait NACAAirfoil {
    fn xu(&self, x: f32) -> f32;
    fn yu(&self, x: f32) -> f32;
    fn xl(&self, x: f32) -> f32;
    fn yl(&self, x: f32) -> f32;
}

impl NACAAirfoil for NACA4 {
    fn xu(&self, x: f32) -> f32 {
        x - self.yt(x) * self.theta(x).sin()
    }

    fn yu(&self, x: f32) -> f32 {
        self.yc(x) + self.yt(x) * self.theta(x).cos()
    }

    fn xl(&self, x: f32) -> f32 {
        x + self.yt(x) * self.theta(x).sin()
    }

    fn yl(&self, x: f32) -> f32 {
        self.yc(x) - self.yt(x) * self.theta(x).cos()
    }
}
