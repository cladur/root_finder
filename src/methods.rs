use std::mem::discriminant;

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

pub fn find_root(
    function: &impl Fn(f64) -> f64,
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
    let bisection = bisection(&function, range_left, range_right, &stop_criteria);

    // Find root using Newton's method
    let newton = newton(&function, range_left, range_right, &stop_criteria);

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

pub fn bisection(
    function: &impl Fn(f64) -> f64,
    range_left: f64,
    range_right: f64,
    stop_criteria: &StopCriteria,
) -> RootInfo {
    let mut left = range_left;
    let mut right = range_right;

    let mut iterations = 1;
    let mut last_x = range_left;

    loop {
        // Compute midpoint.
        let middle = (left + right) / 2.0;

        // If function has the same sign in the middle as it has on left, we move the left end to the right
        if function(middle) * function(left) > 0.0 {
            left = middle;
        // and otherwise the right one to the left.
        } else {
            right = middle;
        }

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

        // Update iteration count and last x.
        iterations += 1;
        last_x = middle;
    }

    RootInfo {
        x: last_x,
        y: function(last_x),
        iterations,
    }
}

pub fn prime(x: f64) -> f64 {
    2.0 * x
}

pub fn newton(
    function: &impl Fn(f64) -> f64,
    range_left: f64,
    range_right: f64,
    stop_criteria: &StopCriteria,
) -> RootInfo {
    let mut x = (range_left + range_right) / 2.0;

    let mut iterations = 1;
    let mut last_x = x;
    loop {
        // If we're about to divide by zero, we break instead.
        if prime(x).abs() <= f64::MIN_POSITIVE {
            break;
        }

        // Compute new x
        x -= function(x) / prime(x);

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

        // Update iteration count and last x.
        iterations += 1;
        last_x = x;
    }

    RootInfo {
        x: last_x,
        y: function(last_x),
        iterations,
    }
}
