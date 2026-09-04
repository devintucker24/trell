use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::embed::{self, cosine, embed_text, lerp_vec, sub_vec};

#[derive(Debug, Clone)]
pub struct Axis {
    pub name: String,
    pub low: String,
    pub high: String,
    pub low_vec: Vec<f32>,
    pub high_vec: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct Offer {
    pub coord: Vec<f32>,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct Space {
    pub axes: Vec<Axis>,
    pub offers: Vec<Offer>,
}

#[derive(Debug, Clone)]
pub struct Grain {
    pub coord: Vec<f32>,
    pub residual: Vec<f32>,
    pub text: String,
    pub shadow: Option<Box<Grain>>,
}

impl Space {
    pub fn new() -> Self {
        Self {
            axes: Vec::new(),
            offers: Vec::new(),
        }
    }

    pub fn axis_index(&self, name: &str) -> Result<usize> {
        self.axes
            .iter()
            .position(|axis| axis.name == name)
            .ok_or_else(|| anyhow!("Unknown axis '{name}'"))
    }

    pub fn add_axis(&mut self, name: String, low: String, high: String) -> Result<()> {
        if self.axes.iter().any(|axis| axis.name == name) {
            return Err(anyhow!("Axis '{name}' is already declared"));
        }
        let low_vec = embed_text(&low);
        let high_vec = embed_text(&high);
        self.axes.push(Axis {
            name,
            low,
            high,
            low_vec,
            high_vec,
        });
        Ok(())
    }

    pub fn add_offer(&mut self, pairs: Vec<(String, f64)>, text: String) -> Result<()> {
        if self.axes.is_empty() {
            return Err(anyhow!("Cannot offer into a space with no axes"));
        }
        let mut coord = vec![0.5f32; self.axes.len()];
        for (name, value) in pairs {
            let index = self.axis_index(&name)?;
            coord[index] = clamp01(value as f32);
        }
        self.offers.push(Offer { coord, text });
        Ok(())
    }

    pub fn feel(&self, text: &str) -> Result<Grain> {
        if self.axes.is_empty() {
            return Err(anyhow!("Cannot feel until axes are declared"));
        }
        let residual = embed_text(text);
        let coord = self.project(&residual);
        Ok(Grain {
            coord,
            residual,
            text: text.to_string(),
            shadow: None,
        })
    }

    pub fn project(&self, residual: &[f32]) -> Vec<f32> {
        self.axes
            .iter()
            .map(|axis| project_axis(residual, axis))
            .collect()
    }
}

impl Grain {
    pub fn along(
        &self,
        space: &Space,
        axis_name: &str,
        toward: f64,
        by: f64,
        frozen: &HashSet<String>,
    ) -> Result<Grain> {
        let index = space.axis_index(axis_name)?;
        if frozen.contains(axis_name) {
            return Ok(self.clone());
        }

        let mut next = self.clone();
        let target = clamp01(toward as f32);
        let rate = clamp01(by as f32);
        next.coord[index] = next.coord[index] + (target - next.coord[index]) * rate;

        let shadow = Grain {
            coord: self.coord.clone(),
            residual: self.residual.clone(),
            text: self.text.clone(),
            shadow: None,
        };
        next.shadow = Some(accumulate_shadow(self.shadow.clone(), shadow));
        next.residual = steer_residual(&self.residual, &space.axes[index], next.coord[index]);
        Ok(next)
    }

    pub fn without(&self, space: &Space, other: &Grain, frozen: &HashSet<String>) -> Grain {
        let mut next = self.clone();
        for (index, axis) in space.axes.iter().enumerate() {
            if frozen.contains(&axis.name) {
                continue;
            }
            let opposite = 1.0 - other.coord[index];
            next.coord[index] = next.coord[index] * 0.25 + opposite * 0.75;
        }
        next.residual = sub_vec(&self.residual, &other.residual, 0.7);
        let shadow = Grain {
            coord: other.coord.clone(),
            residual: other.residual.clone(),
            text: other.text.clone(),
            shadow: None,
        };
        next.shadow = Some(accumulate_shadow(self.shadow.clone(), shadow));
        next
    }

    pub fn blend(&self, other: &Grain, by: f64) -> Grain {
        let t = clamp01(by as f32);
        let coord: Vec<f32> = self
            .coord
            .iter()
            .zip(other.coord.iter())
            .map(|(a, b)| a * (1.0 - t) + b * t)
            .collect();
        let residual = lerp_vec(&self.residual, &other.residual, t);
        let text = if t < 0.5 {
            merge_text(&self.text, &other.text)
        } else {
            merge_text(&other.text, &self.text)
        };
        Grain {
            coord,
            residual,
            text,
            shadow: self.shadow.clone().or(other.shadow.clone()),
        }
    }

    pub fn with_shadow(&self) -> Grain {
        match &self.shadow {
            Some(shadow) => {
                let mut restored = self.blend(shadow, 0.55);
                restored.shadow = None;
                restored
            }
            None => self.clone(),
        }
    }

    pub fn shadow_grain(&self) -> Option<Grain> {
        self.shadow.as_deref().cloned()
    }

    pub fn axis_value(&self, space: &Space, name: &str) -> Result<f32> {
        let index = space.axis_index(name)?;
        Ok(self.coord[index])
    }
}

pub fn resonance(left: &Grain, right: &Grain) -> f32 {
    let n = left.coord.len().max(1) as f32;
    let axis_dist: f32 = left
        .coord
        .iter()
        .zip(right.coord.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f32>()
        / n;
    let residual = (1.0 - cosine(&left.residual, &right.residual)) * 0.5;
    let distance = (axis_dist * 0.7 + residual * 0.3).clamp(0.0, 1.0);
    1.0 - distance
}

pub fn coord_distance(a: &[f32], b: &[f32]) -> f32 {
    if a.is_empty() || a.len() != b.len() {
        return 1.0;
    }
    let sum: f32 = a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum();
    sum / a.len() as f32
}

pub const RESONANCE_THRESHOLD: f32 = 0.62;

fn project_axis(residual: &[f32], axis: &Axis) -> f32 {
    let high = cosine(residual, &axis.high_vec);
    let low = cosine(residual, &axis.low_vec);
    (((high - low) + 2.0) / 4.0).clamp(0.0, 1.0)
}

fn steer_residual(residual: &[f32], axis: &Axis, toward: f32) -> Vec<f32> {
    let pole = if toward >= 0.5 {
        &axis.high_vec
    } else {
        &axis.low_vec
    };
    let amount = (toward - 0.5).abs() * 0.8;
    let mut steered = residual
        .iter()
        .zip(pole.iter())
        .map(|(x, y)| x * (1.0 - amount) + y * amount)
        .collect::<Vec<_>>();
    embed::l2_normalize(&mut steered);
    steered
}

fn accumulate_shadow(existing: Option<Box<Grain>>, incoming: Grain) -> Box<Grain> {
    match existing {
        Some(old) => {
            let mut kept = *old;
            kept.text = merge_unique_text(&kept.text, &incoming.text);
            Box::new(kept)
        }
        None => Box::new(incoming),
    }
}

fn merge_unique_text(primary: &str, secondary: &str) -> String {
    if primary.trim().is_empty() {
        return secondary.to_string();
    }
    if secondary.trim().is_empty() || primary.contains(secondary) || secondary.contains(primary) {
        return primary.to_string();
    }
    format!("{primary} {secondary}")
}

fn merge_text(primary: &str, secondary: &str) -> String {
    if primary.trim().is_empty() {
        secondary.to_string()
    } else if secondary.trim().is_empty() {
        primary.to_string()
    } else {
        format!("{primary} {secondary}")
    }
}

fn clamp01(value: f32) -> f32 {
    value.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn warmth_space() -> Space {
        let mut space = Space::new();
        space
            .add_axis(
                "warmth".into(),
                "ice chart fluorescent instruments sterile".into(),
                "ember hands held darling tender".into(),
            )
            .unwrap();
        space
    }

    #[test]
    fn intimate_text_is_warmer_than_clinical_text() {
        let space = warmth_space();
        let clinical = space
            .feel("The ward was closed. Fluorescent instruments. Ice and charts.")
            .unwrap();
        let intimate = space
            .feel("I held your hands, darling, a tender ember of you.")
            .unwrap();
        assert!(
            intimate.coord[0] > clinical.coord[0],
            "intimate {:.3} vs clinical {:.3}",
            intimate.coord[0],
            clinical.coord[0]
        );
    }

    #[test]
    fn moving_along_an_axis_casts_a_shadow() {
        let space = warmth_space();
        let scene = space
            .feel("fluorescent instruments on a sterile chart")
            .unwrap();
        let moved = scene
            .along(&space, "warmth", 0.9, 1.0, &HashSet::new())
            .unwrap();
        assert!((moved.coord[0] - 0.9).abs() < 1e-5);
        let shadow = moved.shadow_grain().expect("shadow");
        assert!(shadow.coord[0] < moved.coord[0]);
    }
}
