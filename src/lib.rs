use ndarray::prelude::*;

pub type State = Array2<u8>;

pub fn draw(world: &State) {
    print!("┌");
    for _ in 0..world.ncols() {
        print!("─");
    }
    println!("┐");
    for _y in world.rows() {
        print!("│");
        for _x in world.columns() {
            print!("·");
        }
        println!("│");
    }
    print!("└");
    for _ in 0..world.ncols() {
        print!("─");
    }
    print!("┘");
}
