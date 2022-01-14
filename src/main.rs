use ndarray::prelude::*;
use slammers::{draw, predict, print};

fn main() {
    // initial state
    let x0 = array![3., 3., 0., 0.];

    // initial covariance of state
    let p0 = array![
        [0.9, 0.0, 0.0, 0.0],
        [0.0, 0.9, 0.0, 0.0],
        [0.0, 0.0, 0.9, 0.0],
        [0.0, 0.0, 0.0, 0.9],
    ];

    // Dynamics Model
    let f = array![
        [1., 0., 1., 0.],
        [0., 1., 0., 1.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.]
    ];
    // How do control inputs apply to state?
    let b = array![[0., 0.], [0., 0.], [1., 0.], [0., 1.],];

    // process noise
    let q = Array2::from_diag(&array![0.01, 0.01, 0.01, 0.01]);

    let board = Array2::zeros((16, 32));

    let u = array![1., 0.]; // Control inputs

    let mut belief = (x0, p0);

    println!("Step 0:");
    draw(&board, &belief.0, &belief.1);
    print(&belief.0, &belief.1, &u);
    println!("------------------------");

    println!("Step 1:");
    belief = predict(&belief.0, &belief.1, &f, &u, &b, &q);
    draw(&board, &belief.0, &belief.1);
    print(&belief.0, &belief.1, &u);
    println!("------------------------");

    println!("Step 2:");
    belief = predict(&belief.0, &belief.1, &f, &u, &b, &q);
    draw(&board, &belief.0, &belief.1);
    print(&belief.0, &belief.1, &u);
    println!("------------------------");
}
