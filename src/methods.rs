use std::{f64::consts::PI, mem::discriminant};

use eframe::egui::plot::Value;

#[derive(Clone, Copy)]
pub enum StopCriteria {
    Iterations(usize),
    Epsilon(f64),
}

// StopCriteria will be equal to each other if they have the same types, no mather what values they hold.
impl PartialEq for StopCriteria {
    fn eq(&self, other: &StopCriteria) -> bool {
        discriminant(self) == discriminant(other)
    }
}

pub struct ComputeOutput {
    pub values: Vec<Value>,
    pub bisection: RootInfo,
    pub newton: RootInfo,
}

pub struct RootInfo {
    pub x: f64,
    pub y: f64,
    pub iterations: usize,
}

#[derive(Debug)]
pub enum ComputeError {
    SameSign,
}

pub fn nth_root(x: f64, n: u32) -> f64 {
    let exp = 1.0 / n as f64;
    // If n is even we calculate it normally.
    if (n % 2) == 0 {
        x.powf(exp)
    } else {
        // Otherwise we take abs() of the base and then take it to the power.
        let absroot = x.abs().powf(exp);
        // After that we "return" the sign of the base.
        if x < 0.0 {
            -absroot
        } else {
            absroot
        }
    }
}

fn function(x: f64) -> f64 {
    x.sin()
}

fn derivative(x: f64) -> f64 {
    x.cos()
}

pub fn find_root(
    range_left: f64,
    range_right: f64,
    stop_criteria: &StopCriteria,
) -> Result<ComputeOutput, ComputeError> {
    // Swap range if it's in the wrong order
    let (range_left, range_right) = if range_left > range_right {
        (range_right, range_left)
    } else {
        (range_left, range_right)
    };

    // Check if function has different signs at the two ends
    if function(range_left) * function(range_right) > 0.0 {
        return Err(ComputeError::SameSign);
    }

    // Find root using bisection
    let bisection = bisection(range_left, range_right, &stop_criteria);

    // Find root using Newton's method
    let newton = newton(range_left, range_right, &stop_criteria);

    // Generate values for the plot
    let values: Vec<Value> = (0..10000)
        .map(|i| {
            let x = range_left + i as f64 * ((range_right - range_left) / 10000.0);
            Value::new(x, function(x))
        })
        .collect();

    Ok(ComputeOutput {
        values,
        bisection,
        newton,
    })
}

pub fn bisection(range_left: f64, range_right: f64, stop_criteria: &StopCriteria) -> RootInfo {
    let mut left = range_left;
    let mut right = range_right;

    let mut iterations = 0;
    let mut last_x = left;

    loop {
        // Compute midpoint.
        let middle = (left + right) / 2.0;

        // If the middle didn't change it means that the range didn't change, which would happen if
        // left and right were too close to one another.
        // This would happen if function was too "vertical" around the root and / or epsilon was too small.
        if last_x == middle {
            break;
        }

        // If function has the same sign in the middle as it has on left, we move the left end to the right
        if function(middle) * function(left) > 0.0 {
            left = middle;
        // and otherwise the right one to the left.
        } else {
            right = middle;
        }

        // Update iteration count and last x.
        iterations += 1;
        last_x = middle;

        // Break if we've fulfilled the stop criteria.
        match stop_criteria {
            StopCriteria::Iterations(n) => {
                if iterations >= *n {
                    break;
                }
            }
            StopCriteria::Epsilon(epsilon) => {
                if function(last_x).abs() < *epsilon {
                    break;
                }
            }
        }
    }

    RootInfo {
        x: last_x,
        y: function(last_x),
        iterations,
    }
}

pub fn newton(range_left: f64, range_right: f64, stop_criteria: &StopCriteria) -> RootInfo {
    let mut x = (range_left + range_right) / 2.0;

    let mut iterations = 0;
    let mut last_x = x;

    let mut last_last_x = last_x;

    loop {
        // If we're about to divide by zero, we break instead.
        if derivative(x).abs() <= f64::MIN_POSITIVE {
            break;
        }

        // Compute new x.
        x -= function(x) / derivative(x);

        // If one before last value of x is the same as the current one, it means we're going circles,
        // therefore we break instead.
        // This would happen if function was too "vertical" around the root and / or epsilon was too small.
        if x == last_last_x {
            break;
        }

        // Update iteration count and last x.
        iterations += 1;
        last_last_x = last_x;
        last_x = x;

        // Break if we've fulfilled the stop criteria.
        match stop_criteria {
            StopCriteria::Iterations(n) => {
                if iterations >= *n {
                    break;
                }
            }
            StopCriteria::Epsilon(epsilon) => {
                if function(last_x).abs() < *epsilon {
                    break;
                }
            }
        }
    }

    RootInfo {
        x: last_x,
        y: function(last_x),
        iterations,
    }
}
