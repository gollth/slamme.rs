use ndarray::prelude::*;

pub type Matrix = Array2<f32>;
pub type Vector = Array1<f32>;

pub type Belief = (Vector, Matrix);
pub type Controls = Vector;
pub type World = Array2<u8>; // TODO u8 -> ???

pub fn predict(
    x_hat: &Vector,
    p_hat: &Matrix,
    f: &Matrix,
    u: &Controls,
    b: &Matrix,
    q: &Matrix,
) -> Belief {
    let x_hat_prime = f.dot(x_hat) + b.dot(u);
    let p_hat_prime = f.dot(p_hat).dot(&f.t()) + q;
    (x_hat_prime, p_hat_prime)
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
            } else if (x[0] - i as f32).powf(2.) <= p[[0, 0]].powf(2.)
                || (x[1] - j as f32).powf(2.) <= p[[1, 1]].powf(2.)
            {
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
