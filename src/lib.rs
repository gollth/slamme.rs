use ndarray::prelude::*;

pub type State = Array1<f32>;
pub type World = Array2<u8>;

pub fn draw(world: &World, x: &State) {
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
            } else {
                print!("·");
            }
        }
        println!("│");
    }
    print!("└");
    for _ in 0..world.ncols() {
        print!("─");
    }
    print!("┘");
}
