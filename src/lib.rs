use pyo3::prelude::*;
use rand::{self, distributions::Uniform, Rng};
use std::{ptr::copy_nonoverlapping, slice::from_raw_parts_mut};
use vec::Float3;

mod vec;

const G: f32 = 10.0;
const FRAME_TIME: f32 = 1.0 / 24.0;

#[pyfunction]
fn simulate(position_ptr: usize, size: usize, velocity_ptr: usize) {
    let ptr = position_ptr as *mut Float3;
    let position = unsafe { from_raw_parts_mut(ptr, size) };

    let ptr = velocity_ptr as *mut Float3;
    let velocity = unsafe { from_raw_parts_mut(ptr, size) };

    let masss = [0.0, 1.0];

    let mut forces: Vec<Float3> = Vec::with_capacity(size);
    for p1 in 0..position.len() {
        let mut force = Float3::default();
        for p2 in 0..position.len() {
            if p1 != p2 {
                force += (G / (position[p1].distance_square(position[p2])))
                    * (position[p2] - position[p1]).normalize()
                    * masss[p2];
            }
        }
        forces.push(force);
    }
    for i in 0..forces.len() {
        velocity[i] += FRAME_TIME * forces[i];
        position[i] += FRAME_TIME * velocity[i];
    }
}

#[pyfunction]
fn copyposition(position_ptr: usize) {
    let destptr = position_ptr as *mut f32;
    let mut rng = rand::thread_rng();
    // hard coded 1m particles
    let numbers: Vec<f32> = (0..1000000 * 3)
        .map(|_| rng.sample(Uniform::new(0.0, 1.0)))
        .collect();
    unsafe {
        copy_nonoverlapping(numbers.as_ptr(), destptr, 1000000 * 3);
    }
}

#[pymodule]
fn nbody(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate, m)?)?;
    m.add_function(wrap_pyfunction!(copyposition, m)?)?;
    Ok(())
}
