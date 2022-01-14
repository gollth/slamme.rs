use ndarray::prelude::*;

pub type Matrix = Array2<f32>;
pub type Vector = Array1<f32>;

pub type Belief = (Vector, Matrix);
pub type World = Array2<u8>; // TODO u8 -> ???

pub fn predict(
    x_hat: &Vector,
    p_hat: &Matrix,
    f: &Matrix,
    u: &Vector,
    b: &Matrix,
    q: &Matrix,
) -> Belief {
    let x_hat_prime = f.dot(x_hat) + b.dot(u);
    let p_hat_prime = f.dot(p_hat).dot(&f.t()) + q;
    (x_hat_prime, p_hat_prime)
}

fn inside_ellipse(point: &Vector, center: &Vector, axes: &ArrayView1<f32>) -> bool {
    ((point[0] - center[0]).powf(2.) / axes[0].powf(2.)
        + (point[1] - center[1]).powf(2.) / axes[1].powf(2.))
        <= 1.
}

pub fn print(x_hat: &Vector, p_hat: &Matrix, u: &Vector) {
    println!("x̂: {}", x_hat);
    println!("P̂: {}", p_hat.diag().slice(s![0..]));
    println!("û: {}", u);
}
pub fn draw(world: &World, x: &Vector, p: &Matrix) {
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
            } else if inside_ellipse(&array![i as f32, j as f32], &x, &p.diag().slice(s![0..=1])) {
                print!("·");
            } else {
                print!(" ");
            }
        }
        println!("│");
    }
    print!("└");
    for _ in 0..world.ncols() {
        print!("─");
    }
    println!("┘");
}
