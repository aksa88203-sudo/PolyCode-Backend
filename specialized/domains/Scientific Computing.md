# Scientific Computing in Rust

## Overview

Rust's performance, memory safety, and growing ecosystem make it an excellent choice for scientific computing. This guide covers numerical methods, data analysis, visualization, and building scientific applications in Rust.

---

## Scientific Computing Crates

| Crate | Purpose | Features |
|-------|---------|----------|
| `ndarray` | N-dimensional arrays | Array operations, linear algebra |
| `nalgebra` | Linear algebra | Vectors, matrices, transformations |
| `plotters` | Plotting | 2D/3D visualization |
| `statrs` | Statistics | Statistical functions |
| `rustfft` | FFT operations | Fast Fourier Transform |
| `peroxide` | Numerical methods | ODEs, optimization |
| `faer` | Linear algebra | High-performance LA |
| `criterion` | Benchmarking | Performance measurement |

---

## Numerical Arrays

### NDArray Operations

```rust
use ndarray::{Array2, Array1, Array, Ix2, Ix1, s};
use ndarray_linalg::{LinalgError, Lapack};

#[derive(Debug, Clone)]
pub struct ScientificArray<T> {
    data: Array<T, ndarray::IxDyn>,
    metadata: ArrayMetadata,
}

#[derive(Debug, Clone)]
pub struct ArrayMetadata {
    pub name: String,
    pub units: String,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl<T: Clone> ScientificArray<T> {
    pub fn new(data: Array<T, ndarray::IxDyn>, name: String, units: String) -> Self {
        ScientificArray {
            data,
            metadata: ArrayMetadata {
                name,
                units,
                description: String::new(),
                created_at: chrono::Utc::now(),
            },
        }
    }
    
    pub fn from_vec(vec: Vec<T>, shape: &[usize], name: String, units: String) -> Self {
        let array = Array::from_shape_vec(shape, vec).unwrap();
        Self::new(array.into_dyn(), name, units)
    }
    
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn get(&self, index: &[usize]) -> Option<&T> {
        self.data.get(index)
    }
    
    pub fn set(&mut self, index: &[usize], value: T) {
        self.data[index] = value;
    }
    
    pub fn sum(&self) -> T 
    where 
        T: std::ops::Add<Output = T> + Clone + Default,
    {
        self.data.iter().cloned().fold(Default::default(), |acc, x| acc + x)
    }
    
    pub fn mean(&self) -> f64 
    where 
        T: Clone + Into<f64>,
    {
        let sum: f64 = self.data.iter().map(|x| x.clone().into()).sum();
        sum / self.len() as f64
    }
    
    pub fn max(&self) -> Option<&T> 
    where 
        T: PartialOrd,
    {
        self.data.iter().max()
    }
    
    pub fn min(&self) -> Option<&T> 
    where 
        T: PartialOrd,
    {
        self.data.iter().min()
    }
    
    pub fn normalize(&self) -> ScientificArray<f64> 
    where 
        T: Clone + Into<f64>,
    {
        let mean = self.mean();
        let std_dev = self.std_dev();
        
        let normalized_data: Vec<f64> = self.data.iter()
            .map(|x| (x.clone().into() - mean) / std_dev)
            .collect();
        
        ScientificArray::new(
            Array::from_vec(normalized_data).unwrap().into_dyn(),
            format!("{}_normalized", self.metadata.name),
            "normalized".to_string(),
        )
    }
    
    pub fn std_dev(&self) -> f64 
    where 
        T: Clone + Into<f64>,
    {
        let mean = self.mean();
        let variance: f64 = self.data.iter()
            .map(|x| {
                let val = x.clone().into() - mean;
                val * val
            })
            .sum::<f64>() / (self.len() as f64 - 1.0);
        variance.sqrt()
    }
}

// Matrix operations
pub struct MatrixOperations;

impl MatrixOperations {
    pub fn matrix_multiply(a: &Array2<f64>, b: &Array2<f64>) -> Result<Array2<f64>, LinalgError> {
        a.dot(b)
    }
    
    pub fn matrix_inverse(a: &Array2<f64>) -> Result<Array2<f64>, LinalgError> {
        a.inv()
    }
    
    pub fn matrix_transpose(a: &Array2<f64>) -> Array2<f64> {
        a.t().to_owned()
    }
    
    pub fn eigenvalues(a: &Array2<f64>) -> Result<(Array1<f64>, Array2<f64>), LinalgError> {
        a.eig()
    }
    
    pub fn singular_value_decomposition(a: &Array2<f64>) -> Result<(Array2<f64>, Array1<f64>, Array2<f64>), LinalgError> {
        a.svd()
    }
    
    pub fn solve_linear_system(a: &Array2<f64>, b: &Array1<f64>) -> Result<Array1<f64>, LinalgError> {
        a.solve(b)
    }
    
    pub fn matrix_power(a: &Array2<f64>, power: u32) -> Array2<f64> {
        let mut result = Array2::eye(a.nrows());
        let mut current = a.clone();
        
        let mut power_remaining = power;
        while power_remaining > 0 {
            if power_remaining % 2 == 1 {
                result = Self::matrix_multiply(&result, &current).unwrap();
            }
            current = Self::matrix_multiply(&current, &current).unwrap();
            power_remaining /= 2;
        }
        
        result
    }
}

// Vector operations
pub struct VectorOperations;

impl VectorOperations {
    pub fn dot_product(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
        a.dot(b)
    }
    
    pub fn cross_product(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
        assert!(a.len() == 3 && b.len() == 3);
        
        Array1::from_vec(vec![
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
    
    pub fn vector_norm(a: &Array1<f64>) -> f64 {
        a.dot(a).sqrt()
    }
    
    pub fn normalize_vector(a: &Array1<f64>) -> Array1<f64> {
        let norm = Self::vector_norm(a);
        a / norm
    }
    
    pub fn angle_between(a: &Array1<f64>, b: &Array1<f64>) -> f64 {
        let dot_product = Self::dot_product(a, b);
        let norm_a = Self::vector_norm(a);
        let norm_b = Self::vector_norm(b);
        
        (dot_product / (norm_a * norm_b)).acos()
    }
    
    pub fn projection(a: &Array1<f64>, b: &Array1<f64>) -> Array1<f64> {
        let dot_product = Self::dot_product(a, b);
        let norm_b_squared = Self::dot_product(b, b);
        
        b * (dot_product / norm_b_squared)
    }
}
```

### Statistical Functions

```rust
use statrs::distribution::{Continuous, Normal, Uniform};
use statrs::statistics::*;

pub struct StatisticalAnalysis;

impl StatisticalAnalysis {
    pub fn descriptive_stats(data: &[f64]) -> DescriptiveStats {
        let mean = data.iter().sum::<f64>() / data.len() as f64;
        let variance = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (data.len() as f64 - 1.0);
        let std_dev = variance.sqrt();
        
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let median = if sorted_data.len() % 2 == 0 {
            (sorted_data[sorted_data.len() / 2 - 1] + sorted_data[sorted_data.len() / 2]) / 2.0
        } else {
            sorted_data[sorted_data.len() / 2]
        };
        
        let q1 = sorted_data[sorted_data.len() / 4];
        let q3 = sorted_data[3 * sorted_data.len() / 4];
        let iqr = q3 - q1;
        
        let min = sorted_data[0];
        let max = sorted_data[sorted_data.len() - 1];
        
        DescriptiveStats {
            count: data.len(),
            mean,
            median,
            std_dev,
            variance,
            min,
            max,
            q1,
            q3,
            iqr,
            skewness: Self::calculate_skewness(data, mean, std_dev),
            kurtosis: Self::calculate_kurtosis(data, mean, std_dev),
        }
    }
    
    fn calculate_skewness(data: &[f64], mean: f64, std_dev: f64) -> f64 {
        let n = data.len() as f64;
        let skewness = data.iter()
            .map(|x| ((x - mean) / std_dev).powi(3))
            .sum::<f64>() / n;
        
        skewness
    }
    
    fn calculate_kurtosis(data: &[f64], mean: f64, std_dev: f64) -> f64 {
        let n = data.len() as f64;
        let kurtosis = data.iter()
            .map(|x| ((x - mean) / std_dev).powi(4))
            .sum::<f64>() / n - 3.0;
        
        kurtosis
    }
    
    pub fn correlation_coefficient(x: &[f64], y: &[f64]) -> f64 {
        assert_eq!(x.len(), y.len());
        
        let x_mean = x.iter().sum::<f64>() / x.len() as f64;
        let y_mean = y.iter().sum::<f64>() / y.len() as f64;
        
        let x_std = (x.iter().map(|xi| (xi - x_mean).powi(2)).sum::<f64>() / (x.len() as f64 - 1.0)).sqrt();
        let y_std = (y.iter().map(|yi| (yi - y_mean).powi(2)).sum::<f64>() / (y.len() as f64 - 1.0)).sqrt();
        
        let covariance = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
            .sum::<f64>() / (x.len() as f64 - 1.0);
        
        covariance / (x_std * y_std)
    }
    
    pub fn linear_regression(x: &[f64], y: &[f64]) -> LinearRegressionResult {
        assert_eq!(x.len(), y.len());
        
        let n = x.len() as f64;
        let x_mean = x.iter().sum::<f64>() / n;
        let y_mean = y.iter().sum::<f64>() / n;
        
        let numerator = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
            .sum::<f64>();
        
        let denominator = x.iter()
            .map(|xi| (xi - x_mean).powi(2))
            .sum::<f64>();
        
        let slope = numerator / denominator;
        let intercept = y_mean - slope * x_mean;
        
        // Calculate R-squared
        let y_pred: Vec<f64> = x.iter().map(|xi| slope * xi + intercept).collect();
        let ss_total = y.iter().map(|yi| (yi - y_mean).powi(2)).sum::<f64>();
        let ss_residual = y.iter().zip(y_pred.iter())
            .map(|(yi, ypi)| (yi - ypi).powi(2))
            .sum::<f64>();
        
        let r_squared = 1.0 - (ss_residual / ss_total);
        
        LinearRegressionResult {
            slope,
            intercept,
            r_squared,
            correlation: Self::correlation_coefficient(x, y),
        }
    }
    
    pub fn hypothesis_test_two_sample(sample1: &[f64], sample2: &[f64], alpha: f64) -> HypothesisTestResult {
        let n1 = sample1.len() as f64;
        let n2 = sample2.len() as f64;
        
        let mean1 = sample1.iter().sum::<f64>() / n1;
        let mean2 = sample2.iter().sum::<f64>() / n2;
        
        let var1 = sample1.iter()
            .map(|x| (x - mean1).powi(2))
            .sum::<f64>() / (n1 - 1.0);
        let var2 = sample2.iter()
            .map(|x| (x - mean2).powi(2))
            .sum::<f64>() / (n2 - 1.0);
        
        let pooled_var = ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0);
        let standard_error = (pooled_var * (1.0 / n1 + 1.0 / n2)).sqrt();
        
        let t_statistic = (mean1 - mean2) / standard_error;
        let degrees_of_freedom = n1 + n2 - 2.0;
        
        // For simplicity, we'll use a normal approximation
        let normal = Normal::new(0.0, 1.0).unwrap();
        let p_value = 2.0 * (1.0 - normal.cdf(t_statistic.abs()));
        
        let reject_null = p_value < alpha;
        
        HypothesisTestResult {
            t_statistic,
            p_value,
            degrees_of_freedom,
            reject_null,
            confidence_interval: (
                mean1 - mean2 - 1.96 * standard_error,
                mean1 - mean2 + 1.96 * standard_error,
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DescriptiveStats {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub q1: f64,
    pub q3: f64,
    pub iqr: f64,
    pub skewness: f64,
    pub kurtosis: f64,
}

#[derive(Debug, Clone)]
pub struct LinearRegressionResult {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub correlation: f64,
}

#[derive(Debug, Clone)]
pub struct HypothesisTestResult {
    pub t_statistic: f64,
    pub p_value: f64,
    pub degrees_of_freedom: f64,
    pub reject_null: bool,
    pub confidence_interval: (f64, f64),
}
```

---

## Numerical Methods

### Differential Equations

```rust
pub struct ODESolver;

impl ODESolver {
    // Euler's method
    pub fn euler_method<F>(f: F, y0: f64, x0: f64, h: f64, n: usize) -> Vec<(f64, f64)>
    where 
        F: Fn(f64, f64) -> f64,
    {
        let mut points = Vec::new();
        let mut x = x0;
        let mut y = y0;
        
        points.push((x, y));
        
        for _ in 0..n {
            y = y + h * f(x, y);
            x = x + h;
            points.push((x, y));
        }
        
        points
    }
    
    // Runge-Kutta 4th order
    pub fn runge_kutta_4<F>(f: F, y0: f64, x0: f64, h: f64, n: usize) -> Vec<(f64, f64)>
    where 
        F: Fn(f64, f64) -> f64,
    {
        let mut points = Vec::new();
        let mut x = x0;
        let mut y = y0;
        
        points.push((x, y));
        
        for _ in 0..n {
            let k1 = f(x, y);
            let k2 = f(x + h / 2.0, y + h * k1 / 2.0);
            let k3 = f(x + h / 2.0, y + h * k2 / 2.0);
            let k4 = f(x + h, y + h * k3);
            
            y = y + h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            x = x + h;
            points.push((x, y));
        }
        
        points
    }
    
    // System of ODEs using RK4
    pub fn runge_kutta_4_system<F>(f: F, y0: &[f64], x0: f64, h: f64, n: usize) -> Vec<(f64, Vec<f64>)>
    where 
        F: Fn(f64, &[f64]) -> Vec<f64>,
    {
        let mut points = Vec::new();
        let mut x = x0;
        let mut y = y0.to_vec();
        
        points.push((x, y.clone()));
        
        for _ in 0..n {
            let k1 = f(x, &y);
            
            let y_temp: Vec<f64> = y.iter()
                .zip(k1.iter())
                .map(|(yi, k1i)| yi + h * k1i / 2.0)
                .collect();
            let k2 = f(x + h / 2.0, &y_temp);
            
            let y_temp: Vec<f64> = y.iter()
                .zip(k2.iter())
                .map(|(yi, k2i)| yi + h * k2i / 2.0)
                .collect();
            let k3 = f(x + h / 2.0, &y_temp);
            
            let y_temp: Vec<f64> = y.iter()
                .zip(k3.iter())
                .map(|(yi, k3i)| yi + h * k3i)
                .collect();
            let k4 = f(x + h, &y_temp);
            
            for i in 0..y.len() {
                y[i] = y[i] + h * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]) / 6.0;
            }
            
            x = x + h;
            points.push((x, y.clone()));
        }
        
        points
    }
    
    // Adaptive step size RK45
    pub fn runge_kutta_45<F>(f: F, y0: f64, x0: f64, x_end: f64, tol: f64) -> Vec<(f64, f64)>
    where 
        F: Fn(f64, f64) -> f64,
    {
        let mut points = Vec::new();
        let mut x = x0;
        let mut y = y0;
        let mut h = (x_end - x0) / 100.0; // Initial step size
        
        points.push((x, y));
        
        while x < x_end {
            if x + h > x_end {
                h = x_end - x;
            }
            
            // RK4 step
            let k1 = f(x, y);
            let k2 = f(x + h / 2.0, y + h * k1 / 2.0);
            let k3 = f(x + h / 2.0, y + h * k2 / 2.0);
            let k4 = f(x + h, y + h * k3);
            
            let y_rk4 = y + h * (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0;
            
            // RK5 step (simplified)
            let y_rk5 = y + h * (k1 + 3.0 * k2 + 3.0 * k3 + k4) / 8.0;
            
            // Error estimation
            let error = (y_rk5 - y_rk4).abs();
            
            // Adjust step size
            if error < tol {
                y = y_rk4;
                x = x + h;
                points.push((x, y));
                h = h * 1.5; // Increase step size
            } else {
                h = h * 0.5; // Decrease step size
            }
        }
        
        points
    }
}

// Partial differential equations
pub struct PDESolver;

impl PDESolver {
    // Heat equation using finite differences
    pub fn heat_equation_1d(initial_condition: &[f64], alpha: f64, dx: f64, dt: f64, 
                            t_end: f64) -> Vec<Vec<f64>> {
        let nx = initial_condition.len();
        let nt = (t_end / dt) as usize;
        
        let mut solution = Vec::new();
        let mut current = initial_condition.to_vec();
        solution.push(current.clone());
        
        for _ in 0..nt {
            let mut next = vec![0.0; nx];
            let r = alpha * dt / (dx * dx);
            
            // Interior points
            for i in 1..nx - 1 {
                next[i] = current[i] + r * (current[i + 1] - 2.0 * current[i] + current[i - 1]);
            }
            
            // Boundary conditions (Dirichlet: u(0,t) = u(L,t) = 0)
            next[0] = 0.0;
            next[nx - 1] = 0.0;
            
            current = next;
            solution.push(current.clone());
        }
        
        solution
    }
    
    // Wave equation using finite differences
    pub fn wave_equation_1d(initial_displacement: &[f64], initial_velocity: &[f64], 
                          c: f64, dx: f64, dt: f64, t_end: f64) -> Vec<Vec<f64>> {
        let nx = initial_displacement.len();
        let nt = (t_end / dt) as usize;
        
        let mut solution = Vec::new();
        let mut u_prev = initial_displacement.to_vec();
        let mut u_curr = initial_displacement.to_vec();
        
        solution.push(u_curr.clone());
        
        let r = (c * dt / dx).powi(2);
        
        // First time step
        let mut u_next = vec![0.0; nx];
        for i in 1..nx - 1 {
            u_next[i] = u_curr[i] + dt * initial_velocity[i] + 
                      r * 0.5 * (u_curr[i + 1] - 2.0 * u_curr[i] + u_curr[i - 1]);
        }
        
        // Boundary conditions
        u_next[0] = 0.0;
        u_next[nx - 1] = 0.0;
        
        u_prev = u_curr;
        u_curr = u_next;
        solution.push(u_curr.clone());
        
        // Subsequent time steps
        for _ in 1..nt {
            let mut u_next = vec![0.0; nx];
            
            for i in 1..nx - 1 {
                u_next[i] = 2.0 * u_curr[i] - u_prev[i] + 
                          r * (u_curr[i + 1] - 2.0 * u_curr[i] + u_curr[i - 1]);
            }
            
            // Boundary conditions
            u_next[0] = 0.0;
            u_next[nx - 1] = 0.0;
            
            u_prev = u_curr;
            u_curr = u_next;
            solution.push(u_curr.clone());
        }
        
        solution
    }
}
```

### Optimization

```rust
pub struct Optimization;

impl Optimization {
    // Gradient descent
    pub fn gradient_descent<F, G>(f: F, gradient: G, x0: &[f64], learning_rate: f64, 
                                max_iterations: usize, tolerance: f64) -> (Vec<f64>, f64)
    where 
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
    {
        let mut x = x0.to_vec();
        let mut previous_value = f(&x);
        
        for iteration in 0..max_iterations {
            let grad = gradient(&x);
            
            // Update parameters
            for i in 0..x.len() {
                x[i] -= learning_rate * grad[i];
            }
            
            let current_value = f(&x);
            
            // Check convergence
            if (current_value - previous_value).abs() < tolerance {
                println!("Converged after {} iterations", iteration);
                break;
            }
            
            previous_value = current_value;
            
            // Adaptive learning rate
            if iteration % 100 == 0 && iteration > 0 {
                learning_rate *= 0.9;
            }
        }
        
        (x, previous_value)
    }
    
    // Newton's method for optimization
    pub fn newton_method<F, G, H>(f: F, gradient: G, hessian: H, x0: &[f64], 
                                 max_iterations: usize, tolerance: f64) -> (Vec<f64>, f64)
    where 
        F: Fn(&[f64]) -> f64,
        G: Fn(&[f64]) -> Vec<f64>,
        H: Fn(&[f64]) -> Vec<Vec<f64>>,
    {
        let mut x = x0.to_vec();
        let n = x.len();
        
        for iteration in 0..max_iterations {
            let grad = gradient(&x);
            let hess = hessian(&x);
            
            // Solve H * delta = -grad
            let delta = Self::solve_linear_system(&hess, &grad.iter().map(|g| -g).collect());
            
            // Update parameters
            for i in 0..n {
                x[i] += delta[i];
            }
            
            // Check convergence
            let grad_norm = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            if grad_norm < tolerance {
                println!("Converged after {} iterations", iteration);
                break;
            }
        }
        
        (x, f(&x))
    }
    
    fn solve_linear_system(a: &[Vec<f64>], b: &[f64]) -> Vec<f64> {
        // Simple Gaussian elimination (for demonstration)
        let n = a.len();
        let mut augmented = a.iter().zip(b.iter()).map(|(row, &b_val)| {
            let mut augmented_row = row.clone();
            augmented_row.push(b_val);
            augmented_row
        }).collect::<Vec<_>>();
        
        // Forward elimination
        for i in 0..n {
            // Find pivot
            let mut max_row = i;
            for k in i + 1..n {
                if augmented[k][i].abs() > augmented[max_row][i].abs() {
                    max_row = k;
                }
            }
            
            // Swap rows
            augmented.swap(i, max_row);
            
            // Eliminate column
            for k in i + 1..n {
                let factor = augmented[k][i] / augmented[i][i];
                for j in i..=n {
                    augmented[k][j] -= factor * augmented[i][j];
                }
            }
        }
        
        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            x[i] = augmented[i][n];
            for k in i + 1..n {
                x[i] -= augmented[i][k] * x[k];
            }
            x[i] /= augmented[i][i];
        }
        
        x
    }
    
    // Simulated annealing
    pub fn simulated_annealing<F>(f: F, x0: &[f64], bounds: &[(f64, f64)], 
                                  initial_temp: f64, cooling_rate: f64, 
                                  min_temp: f64) -> (Vec<f64>, f64)
    where 
        F: Fn(&[f64]) -> f64,
    {
        let mut x = x0.to_vec();
        let mut current_value = f(&x);
        let mut best_x = x.clone();
        let mut best_value = current_value;
        let mut temperature = initial_temp;
        
        while temperature > min_temp {
            // Generate neighboring solution
            let mut new_x = x.clone();
            for i in 0..new_x.len() {
                let perturbation = (rand::random::<f64>() - 0.5) * temperature;
                new_x[i] = (new_x[i] + perturbation).max(bounds[i].0).min(bounds[i].1);
            }
            
            let new_value = f(&new_x);
            
            // Accept or reject
            let delta = new_value - current_value;
            if delta < 0.0 || rand::random::<f64>() < (-delta / temperature).exp() {
                x = new_x;
                current_value = new_value;
                
                if current_value < best_value {
                    best_x = x.clone();
                    best_value = current_value;
                }
            }
            
            temperature *= cooling_rate;
        }
        
        (best_x, best_value)
    }
}
```

---

## Data Visualization

### Plotting with Plotters

```rust
use plotters::prelude::*;
use plotters::coord::Shift;
use plotters::style::colors;

pub struct DataVisualizer;

impl DataVisualizer {
    pub fn plot_line_chart(data: &[(f64, f64)], title: &str, x_label: &str, y_label: &str, 
                          filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                data.iter().map(|(x, _)| *x).min(0.0)..data.iter().map(|(x, _)| *x).max(10.0),
                data.iter().map(|(_, y)| *y).min(0.0)..data.iter().map(|(_, y)| *y).max(10.0),
            )?;
        
        chart.configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()?;
        
        chart.draw_series(LineSeries::new(
            data.iter().map(|(x, y)| (*x, *y)),
            &BLUE,
        ))?
        .label("Data")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &BLUE));
        
        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
        
        Ok(())
    }
    
    pub fn plot_scatter_plot(data: &[(f64, f64)], title: &str, x_label: &str, y_label: &str, 
                            filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                data.iter().map(|(x, _)| *x).min(0.0)..data.iter().map(|(x, _)| *x).max(10.0),
                data.iter().map(|(_, y)| *y).min(0.0)..data.iter().map(|(_, y)| *y).max(10.0),
            )?;
        
        chart.configure_mesh()
            .x_desc(x_label)
            .y_desc(y_label)
            .draw()?;
        
        chart.draw_series(data.iter().map(|(x, y)| {
            Circle::new((*x, *y), 3, &RED.fill())
        }))?
        .label("Data Points")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], &RED));
        
        chart.configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;
        
        Ok(())
    }
    
    pub fn plot_histogram(data: &[f64], bins: usize, title: &str, x_label: &str, 
                        filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let bin_width = (max - min) / bins as f64;
        
        let mut histogram = vec![0; bins];
        for &value in data {
            let bin_index = ((value - min) / bin_width) as usize;
            if bin_index < bins {
                histogram[bin_index] += 1;
            }
        }
        
        let bin_centers: Vec<f64> = (0..bins)
            .map(|i| min + (i as f64 + 0.5) * bin_width)
            .collect();
        
        let data_points: Vec<(f64, f64)> = bin_centers.iter()
            .zip(histogram.iter())
            .map(|(x, &y)| (*x, y as f64))
            .collect();
        
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let max_count = *histogram.iter().max().unwrap();
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(
                min..max,
                0.0..max_count as f64,
            )?;
        
        chart.configure_mesh()
            .x_desc(x_label)
            .y_desc("Frequency")
            .draw()?;
        
        chart.draw_series(BarChart::default()
            .style(&GREEN.mix(0.5).filled())
            .data(data_points.iter().map(|(x, y)| (*x, *y, bin_width)))
        )?;
        
        Ok(())
    }
    
    pub fn plot_3d_surface(data: &[[f64]], title: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30).into_font())
            .build_cartesian_3d(
                0.0..data.len() as f64,
                0.0..data[0].len() as f64,
                data.iter().flat_map(|row| row.iter()).cloned().min(0.0)..data.iter().flat_map(|row| row.iter()).cloned().max(10.0),
            )?;
        
        chart.configure_axes()
            .draw()?;
        
        // Draw surface
        for (i, row) in data.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                chart.draw_series(PointSeries::of_element(
                    [(i as f64, j as f64, value)],
                    2,
                    &BLUE,
                    &|c, s, st| EmptyElement::at(c).into_dyn_pix(st).into_styled(s)
                ))?;
            }
        }
        
        Ok(())
    }
}
```

---

## Key Takeaways

- **NDArray** provides powerful n-dimensional array operations
- **Numerical methods** enable solving mathematical problems
- **Statistical analysis** provides data insights
- **ODE solvers** handle differential equations
- **Optimization** finds optimal solutions
- **Visualization** helps understand data
- **Performance** is critical for scientific computing

---

## Scientific Computing Best Practices

| Practice | Description | Implementation |
|----------|-------------|----------------|
| **Numerical stability** | Avoid numerical errors | Use appropriate algorithms |
| **Validation** | Verify results | Compare with known solutions |
| **Performance** | Optimize computations | Use efficient data structures |
| **Documentation** | Document mathematical methods | Include references |
| **Testing** | Test edge cases | Unit tests for numerical methods |
| **Visualization** | Plot results for verification | Use appropriate charts |
| **Error analysis** | Estimate numerical errors | Error bounds and convergence |
