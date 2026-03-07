// scientific_computing.rs
// Scientific computing examples in Rust

use std::collections::HashMap;

// Simple 2D array for scientific computing
#[derive(Debug, Clone)]
pub struct Matrix2D {
    data: Vec<Vec<f64>>,
    rows: usize,
    cols: usize,
}

impl Matrix2D {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix2D {
            data: vec![vec![0.0; cols]; rows],
            rows,
            cols,
        }
    }
    
    pub fn from_vec(data: Vec<Vec<f64>>) -> Result<Self, String> {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        
        for row in &data {
            if row.len() != cols {
                return Err("Inconsistent row lengths".to_string());
            }
        }
        
        Ok(Matrix2D { data, rows, cols })
    }
    
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row).and_then(|row| row.get(col)).copied()
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
        if row >= self.rows || col >= self.cols {
            return Err("Index out of bounds".to_string());
        }
        self.data[row][col] = value;
        Ok(())
    }
    
    pub fn rows(&self) -> usize {
        self.rows
    }
    
    pub fn cols(&self) -> usize {
        self.cols
    }
    
    pub fn transpose(&self) -> Matrix2D {
        let mut result = Matrix2D::new(self.cols, self.rows);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j).unwrap()).unwrap();
            }
        }
        
        result
    }
    
    pub fn multiply(&self, other: &Matrix2D) -> Result<Matrix2D, String> {
        if self.cols != other.rows {
            return Err("Matrix dimensions incompatible for multiplication".to_string());
        }
        
        let mut result = Matrix2D::new(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k).unwrap() * other.get(k, j).unwrap();
                }
                result.set(i, j, sum).unwrap();
            }
        }
        
        Ok(result)
    }
    
    pub fn add(&self, other: &Matrix2D) -> Result<Matrix2D, String> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions incompatible for addition".to_string());
        }
        
        let mut result = Matrix2D::new(self.rows, self.cols);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                let sum = self.get(i, j).unwrap() + other.get(i, j).unwrap();
                result.set(i, j, sum).unwrap();
            }
        }
        
        Ok(result)
    }
    
    pub fn scale(&self, scalar: f64) -> Matrix2D {
        let mut result = Matrix2D::new(self.rows, self.cols);
        
        for i in 0..self.rows {
            for j in 0..self.cols {
                let scaled = self.get(i, j).unwrap() * scalar;
                result.set(i, j, scaled).unwrap();
            }
        }
        
        result
    }
    
    pub fn determinant(&self) -> f64 {
        if self.rows != self.cols {
            return f64::NAN;
        }
        
        if self.rows == 1 {
            return self.get(0, 0).unwrap();
        }
        
        if self.rows == 2 {
            return self.get(0, 0).unwrap() * self.get(1, 1).unwrap() - 
                   self.get(0, 1).unwrap() * self.get(1, 0).unwrap();
        }
        
        // Recursive Laplace expansion (not efficient for large matrices)
        let mut det = 0.0;
        for j in 0..self.cols {
            let cofactor = self.cofactor(0, j);
            det += self.get(0, j).unwrap() * cofactor;
        }
        
        det
    }
    
    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
        sign * self.minor(row, col)
    }
    
    fn minor(&self, row: usize, col: usize) -> f64 {
        let mut minor_matrix = Matrix2D::new(self.rows - 1, self.cols - 1);
        
        let mut minor_row = 0;
        for i in 0..self.rows {
            if i == row { continue; }
            
            let mut minor_col = 0;
            for j in 0..self.cols {
                if j == col { continue; }
                
                let value = self.get(i, j).unwrap();
                minor_matrix.set(minor_row, minor_col, value).unwrap();
                minor_col += 1;
            }
            minor_row += 1;
        }
        
        minor_matrix.determinant()
    }
}

// Vector operations
#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn new(data: Vec<f64>) -> Self {
        Vector { data }
    }
    
    pub fn zeros(size: usize) -> Self {
        Vector::new(vec![0.0; size])
    }
    
    pub fn ones(size: usize) -> Self {
        Vector::new(vec![1.0; size])
    }
    
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    pub fn get(&self, index: usize) -> Option<f64> {
        self.data.get(index).copied()
    }
    
    pub fn set(&mut self, index: usize, value: f64) -> Result<(), String> {
        if index >= self.data.len() {
            return Err("Index out of bounds".to_string());
        }
        self.data[index] = value;
        Ok(())
    }
    
    pub fn dot(&self, other: &Vector) -> Result<f64, String> {
        if self.len() != other.len() {
            return Err("Vector dimensions incompatible".to_string());
        }
        
        let mut sum = 0.0;
        for i in 0..self.len() {
            sum += self.get(i).unwrap() * other.get(i).unwrap();
        }
        
        Ok(sum)
    }
    
    pub fn norm(&self) -> f64 {
        self.dot(self).unwrap().sqrt()
    }
    
    pub fn normalize(&self) -> Vector {
        let norm = self.norm();
        if norm == 0.0 {
            return self.clone();
        }
        
        let normalized_data: Vec<f64> = self.data.iter().map(|&x| x / norm).collect();
        Vector::new(normalized_data)
    }
    
    pub fn add(&self, other: &Vector) -> Result<Vector, String> {
        if self.len() != other.len() {
            return Err("Vector dimensions incompatible".to_string());
        }
        
        let result_data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a + b)
            .collect();
        
        Ok(Vector::new(result_data))
    }
    
    pub fn subtract(&self, other: &Vector) -> Result<Vector, String> {
        if self.len() != other.len() {
            return Err("Vector dimensions incompatible".to_string());
        }
        
        let result_data: Vec<f64> = self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| a - b)
            .collect();
        
        Ok(Vector::new(result_data))
    }
    
    pub fn scale(&self, scalar: f64) -> Vector {
        let scaled_data: Vec<f64> = self.data.iter().map(|&x| x * scalar).collect();
        Vector::new(scaled_data)
    }
}

// Statistical functions
pub struct Statistics;

impl Statistics {
    pub fn mean(data: &[f64]) -> f64 {
        if data.is_empty() {
            return f64::NAN;
        }
        
        data.iter().sum::<f64>() / data.len() as f64
    }
    
    pub fn median(data: &[f64]) -> f64 {
        if data.is_empty() {
            return f64::NAN;
        }
        
        let mut sorted_data = data.to_vec();
        sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let n = sorted_data.len();
        if n % 2 == 0 {
            (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
        } else {
            sorted_data[n / 2]
        }
    }
    
    pub fn variance(data: &[f64]) -> f64 {
        if data.len() < 2 {
            return f64::NAN;
        }
        
        let mean = Self::mean(data);
        let sum_squared_diff: f64 = data.iter()
            .map(|x| (x - mean).powi(2))
            .sum();
        
        sum_squared_diff / (data.len() - 1) as f64
    }
    
    pub fn std_deviation(data: &[f64]) -> f64 {
        Self::variance(data).sqrt()
    }
    
    pub fn min(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
        }
    }
    
    pub fn max(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
        }
    }
    
    pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
        if x.len() != y.len() || x.len() < 2 {
            return f64::NAN;
        }
        
        let n = x.len() as f64;
        let x_mean = Self::mean(x);
        let y_mean = Self::mean(y);
        
        let covariance: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
            .sum();
        
        let x_var = Self::variance(x);
        let y_var = Self::variance(y);
        
        covariance / (x_var * y_var).sqrt()
    }
    
    pub fn linear_regression(x: &[f64], y: &[f64]) -> Result<LinearRegressionResult, String> {
        if x.len() != y.len() || x.len() < 2 {
            return Err("Insufficient data".to_string());
        }
        
        let n = x.len() as f64;
        let x_mean = Self::mean(x);
        let y_mean = Self::mean(y);
        
        let numerator: f64 = x.iter().zip(y.iter())
            .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
            .sum();
        
        let denominator: f64 = x.iter()
            .map(|xi| (xi - x_mean).powi(2))
            .sum();
        
        let slope = numerator / denominator;
        let intercept = y_mean - slope * x_mean;
        
        // Calculate R-squared
        let y_pred: Vec<f64> = x.iter().map(|xi| slope * xi + intercept).collect();
        let ss_total: f64 = y.iter()
            .map(|yi| (yi - y_mean).powi(2))
            .sum();
        let ss_residual: f64 = y.iter().zip(y_pred.iter())
            .map(|(yi, ypi)| (yi - ypi).powi(2))
            .sum();
        
        let r_squared = 1.0 - (ss_residual / ss_total);
        
        Ok(LinearRegressionResult {
            slope,
            intercept,
            r_squared,
            correlation: Self::correlation(x, y),
        })
    }
}

#[derive(Debug, Clone)]
pub struct LinearRegressionResult {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub correlation: f64,
}

// Numerical methods
pub struct NumericalMethods;

impl NumericalMethods {
    // Euler's method for ODEs
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
    
    // Newton's method for root finding
    pub fn newton_method<F, G>(f: F, f_prime: G, x0: f64, tolerance: f64, max_iterations: usize) -> Option<f64>
    where 
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
    {
        let mut x = x0;
        
        for _ in 0..max_iterations {
            let fx = f(x);
            let fpx = f_prime(x);
            
            if fpx.abs() < 1e-10 {
                return None; // Derivative too small
            }
            
            let x_new = x - fx / fpx;
            
            if (x_new - x).abs() < tolerance {
                return Some(x_new);
            }
            
            x = x_new;
        }
        
        None
    }
    
    // Bisection method for root finding
    pub fn bisection_method<F>(f: F, a: f64, b: f64, tolerance: f64) -> Option<f64>
    where 
        F: Fn(f64) -> f64,
    {
        let fa = f(a);
        let fb = f(b);
        
        if fa * fb > 0.0 {
            return None; // Root not bracketed
        }
        
        let mut left = a;
        let mut right = b;
        
        while (right - left).abs() > tolerance {
            let mid = (left + right) / 2.0;
            let fmid = f(mid);
            
            if fa * fmid <= 0.0 {
                right = mid;
            } else {
                left = mid;
            }
        }
        
        Some((left + right) / 2.0)
    }
    
    // Numerical integration using Simpson's rule
    pub fn simpson_integration<F>(f: F, a: f64, b: f64, n: usize) -> f64
    where 
        F: Fn(f64) -> f64,
    {
        if n % 2 != 0 {
            return f64::NAN; // n must be even
        }
        
        let h = (b - a) / n as f64;
        let mut sum = f(a) + f(b);
        
        for i in 1..n {
            let x = a + i as f64 * h;
            let coefficient = if i % 2 == 0 { 2.0 } else { 4.0 };
            sum += coefficient * f(x);
        }
        
        sum * h / 3.0
    }
}

// Optimization
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
        }
        
        (x, previous_value)
    }
    
    // Golden section search for 1D optimization
    pub fn golden_section_search<F>(f: F, a: f64, b: f64, tolerance: f64) -> f64
    where 
        F: Fn(f64) -> f64,
    {
        let golden_ratio = (5.0_f64.sqrt() - 1.0) / 2.0;
        
        let mut left = a;
        let mut right = b;
        
        let mut x1 = right - golden_ratio * (right - left);
        let mut x2 = left + golden_ratio * (right - left);
        
        let mut f1 = f(x1);
        let mut f2 = f(x2);
        
        while (right - left).abs() > tolerance {
            if f1 < f2 {
                right = x2;
                x2 = x1;
                f2 = f1;
                x1 = right - golden_ratio * (right - left);
                f1 = f(x1);
            } else {
                left = x1;
                x1 = x2;
                f1 = f2;
                x2 = left + golden_ratio * (right - left);
                f2 = f(x2);
            }
        }
        
        (left + right) / 2.0
    }
}

// Main demonstration
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== SCIENTIFIC COMPUTING DEMONSTRATIONS ===\n");
    
    // Matrix operations
    println!("=== MATRIX OPERATIONS ===");
    let mut matrix_a = Matrix2D::new(2, 2);
    matrix_a.set(0, 0, 1.0)?;
    matrix_a.set(0, 1, 2.0)?;
    matrix_a.set(1, 0, 3.0)?;
    matrix_a.set(1, 1, 4.0)?;
    
    let mut matrix_b = Matrix2D::new(2, 2);
    matrix_b.set(0, 0, 5.0)?;
    matrix_b.set(0, 1, 6.0)?;
    matrix_b.set(1, 0, 7.0)?;
    matrix_b.set(1, 1, 8.0)?;
    
    println!("Matrix A:");
    for i in 0..matrix_a.rows() {
        for j in 0..matrix_a.cols() {
            print!("{:.1} ", matrix_a.get(i, j).unwrap());
        }
        println!();
    }
    
    let product = matrix_a.multiply(&matrix_b)?;
    println!("Matrix A * Matrix B:");
    for i in 0..product.rows() {
        for j in 0..product.cols() {
            print!("{:.1} ", product.get(i, j).unwrap());
        }
        println!();
    }
    
    let det = matrix_a.determinant();
    println!("Determinant of A: {:.1}", det);
    
    // Vector operations
    println!("\n=== VECTOR OPERATIONS ===");
    let vector_a = Vector::new(vec![1.0, 2.0, 3.0]);
    let vector_b = Vector::new(vec![4.0, 5.0, 6.0]);
    
    let dot_product = vector_a.dot(&vector_b)?;
    println!("Dot product: {:.1}", dot_product);
    
    let norm_a = vector_a.norm();
    println!("Norm of vector A: {:.3}", norm_a);
    
    let normalized_a = vector_a.normalize();
    println!("Normalized vector A: ({:.3}, {:.3}, {:.3})", 
             normalized_a.get(0).unwrap(), 
             normalized_a.get(1).unwrap(), 
             normalized_a.get(2).unwrap());
    
    // Statistics
    println!("\n=== STATISTICS ===");
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    
    let mean = Statistics::mean(&data);
    let median = Statistics::median(&data);
    let std_dev = Statistics::std_deviation(&data);
    
    println!("Data: {:?}", data);
    println!("Mean: {:.2}", mean);
    println!("Median: {:.2}", median);
    println!("Standard deviation: {:.2}", std_dev);
    
    let x_data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y_data = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    
    let correlation = Statistics::correlation(&x_data, &y_data);
    println!("Correlation between x and y: {:.3}", correlation);
    
    let regression = Statistics::linear_regression(&x_data, &y_data)?;
    println!("Linear regression: y = {:.2}x + {:.2}", regression.slope, regression.intercept);
    println!("R-squared: {:.3}", regression.r_squared);
    
    // Numerical methods
    println!("\n=== NUMERICAL METHODS ===");
    
    // ODE solver (dy/dx = x, y(0) = 0)
    let ode_solution = NumericalMethods::euler_method(|x, _y| x, 0.0, 0.0, 0.1, 10);
    println!("Euler method solution to dy/dx = x:");
    for (i, (x, y)) in ode_solution.iter().enumerate().take(5) {
        println!("  Step {}: x = {:.1}, y = {:.3}", i, x, y);
    }
    
    let rk4_solution = NumericalMethods::runge_kutta_4(|x, _y| x, 0.0, 0.0, 0.1, 10);
    println!("Runge-Kutta 4 solution to dy/dx = x:");
    for (i, (x, y)) in rk4_solution.iter().enumerate().take(5) {
        println!("  Step {}: x = {:.1}, y = {:.3}", i, x, y);
    }
    
    // Root finding (f(x) = x^2 - 4)
    let root = NumericalMethods::newton_method(
        |x| x * x - 4.0,
        |x| 2.0 * x,
        1.0,
        1e-6,
        100
    );
    println!("Newton's method root of x^2 - 4: {:?}", root);
    
    let root_bisection = NumericalMethods::bisection_method(
        |x| x * x - 4.0,
        0.0,
        3.0,
        1e-6
    );
    println!("Bisection method root of x^2 - 4: {:?}", root_bisection);
    
    // Numerical integration
    let integral = NumericalMethods::simpson_integration(|x| x * x, 0.0, 1.0, 100);
    println!("Integral of x^2 from 0 to 1: {:.6}", integral);
    
    // Optimization
    println!("\n=== OPTIMIZATION ===");
    
    // Minimize f(x) = (x - 2)^2
    let minimum = Optimization::golden_section_search(
        |x| (x - 2.0).powi(2),
        0.0,
        5.0,
        1e-6
    );
    println!("Minimum of (x - 2)^2: {:.6}", minimum);
    
    // Gradient descent for f(x,y) = (x-1)^2 + (y-2)^2
    let gradient_result = Optimization::gradient_descent(
        |params| {
            let x = params[0];
            let y = params[1];
            (x - 1.0).powi(2) + (y - 2.0).powi(2)
        },
        |params| {
            let x = params[0];
            let y = params[1];
            vec![2.0 * (x - 1.0), 2.0 * (y - 2.0)]
        },
        &[0.0, 0.0],
        0.1,
        1000,
        1e-6
    );
    println!("Gradient descent minimum: ({:.6}, {:.6}) with value {:.6}", 
             gradient_result.0[0], gradient_result.0[1], gradient_result.1);
    
    println!("\n=== SCIENTIFIC COMPUTING DEMONSTRATIONS COMPLETE ===");
    println!("Key concepts demonstrated:");
    println!("- Matrix operations (multiplication, determinant, transpose)");
    println!("- Vector operations (dot product, norm, normalization)");
    println!("- Statistical analysis (mean, median, correlation, regression)");
    println!("- Numerical methods (ODE solving, root finding, integration)");
    println!("- Optimization (gradient descent, golden section search)");
    println!("- Mathematical functions and algorithms");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matrix_operations() {
        let mut matrix = Matrix2D::new(2, 2);
        matrix.set(0, 0, 1.0).unwrap();
        matrix.set(0, 1, 2.0).unwrap();
        matrix.set(1, 0, 3.0).unwrap();
        matrix.set(1, 1, 4.0).unwrap();
        
        assert_eq!(matrix.determinant(), -2.0);
        
        let transposed = matrix.transpose();
        assert_eq!(transposed.get(0, 1).unwrap(), 3.0);
        assert_eq!(transposed.get(1, 0).unwrap(), 2.0);
    }
    
    #[test]
    fn test_vector_operations() {
        let vector = Vector::new(vec![3.0, 4.0]);
        assert_eq!(vector.norm(), 5.0);
        
        let normalized = vector.normalize();
        assert!((normalized.norm() - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_statistics() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::mean(&data), 3.0);
        assert_eq!(Statistics::median(&data), 3.0);
        
        let correlation = Statistics::correlation(&data, &data);
        assert!((correlation - 1.0).abs() < 1e-10);
    }
    
    #[test]
    fn test_numerical_methods() {
        let root = NumericalMethods::newton_method(
            |x| x * x - 4.0,
            |x| 2.0 * x,
            1.0,
            1e-6,
            100
        );
        assert!(root.is_some());
        assert!((root.unwrap() - 2.0).abs() < 1e-5);
    }
    
    #[test]
    fn test_optimization() {
        let minimum = Optimization::golden_section_search(
            |x| (x - 2.0).powi(2),
            0.0,
            5.0,
            1e-6
        );
        assert!((minimum - 2.0).abs() < 1e-5);
    }
}
