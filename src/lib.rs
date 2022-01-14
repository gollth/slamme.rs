use ndarray::prelude::*;
use ndarray_linalg::Inverse;
use rand::thread_rng;
use rand_distr::{Distribution, Normal};

pub type Matrix = Array2<f32>;
pub type Vector = Array1<f32>;

pub type Belief = (Vector, Matrix);
pub type World = Array2<u8>; // TODO u8 -> ???

// hidden real observation noise
const SENSOR_LANDMARK_NOISE: f32 = 0.2425;
const SENSOR_WHEEL_NOISE: f32 = 0.1313;

pub fn predict(
    x_hat: &Vector,
    p_hat: &Matrix,
    f: &Matrix,
    u: &Vector,
    b: &Matrix,
    q: &Vector,
) -> Belief {
    let x_hat_prime = f.dot(x_hat) + b.dot(u);
    let p_hat_prime = f.dot(p_hat).dot(&f.t()) + Matrix::from_diag(q);
    (x_hat_prime, p_hat_prime)
}

pub fn update(z: &Vector, h: &Matrix, x_hat: &Vector, p: &Matrix, v: &Vector) -> Belief {
    let y_hat = z - h.dot(x_hat); // noisy measurement
    let s = h.dot(p).dot(&h.t()) + Matrix::from_diag(v);
    let k = p.dot(&h.t()).dot(&s.inv().unwrap()); // optimal Kalman gain
    println!("Kalman Gain:\n{}", k);
    let x_hat_prime = x_hat + k.dot(&y_hat);
    let p_prime = (Matrix::eye(p.dim().0) - k.dot(h)).dot(p);
    (x_hat_prime, p_prime)
}

pub fn measure(world: &World, x_hat: &Vector) -> Vector {
    let mut rng = thread_rng();
    let mut z = Vec::new();
    let landmarks = Normal::new(0., SENSOR_LANDMARK_NOISE).unwrap();
    let wheels = Normal::new(0., SENSOR_WHEEL_NOISE).unwrap();

    // Add the robot state to the measurement
    for i in 0..4 {
        z.push(x_hat[i] + wheels.sample(&mut rng));
    }

    // Add the landmarks to the measurement
    for j in 0..world.nrows() {
        for i in 0..world.ncols() {
            if world[[j, i]] > 0 {
                z.push(i as f32 + landmarks.sample(&mut rng));
                z.push(j as f32 + landmarks.sample(&mut rng));
            }
        }
    }
    Vector::from_vec(z)
}

fn inside_ellipse(point: &Vector, center: &Vector, axes: &ArrayView1<f32>) -> bool {
    ((point[0] - center[0]).powf(2.) / axes[0].powf(2.)
        + (point[1] - center[1]).powf(2.) / axes[1].powf(2.))
        <= 1.
}

pub fn draw(k: usize, world: &World, x: &Vector, p: &Matrix, u: &Vector) {
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    println!(" Step {}:", k);
    print!("┌");
    for _ in 0..world.ncols() {
        print!("─");
    }
    println!("┐");
    for j in 0..world.nrows() {
        print!("│");
        for i in 0..world.ncols() {
            if x[0].round() as usize == i && x[1].round() as usize == j {
                print!("⊕");
                continue;
            }
            if world[[j, i]] > 0 {
                print!("●");
                continue;
            }
            if inside_ellipse(&array![i as f32, j as f32], &x, &p.diag().slice(s![0..=1])) {
                print!("·");
                continue;
            }
            print!(" ");
        }
        println!("│");
    }
    print!("└");
    for _ in 0..world.ncols() {
        print!("─");
    }
    println!("┘");

    // diagnostics
    println!("x̂: {}", x);
    println!("P̂: {}", p.diag().slice(s![0..]));
    println!("û: {}", u);
}
