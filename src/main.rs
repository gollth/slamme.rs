use ndarray::prelude::*;
use slammers::draw;
use slammers::predict;

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
    let q = Array2::from_diag(&array![0.05, 0.05, 0.01, 0.01]);

    let board = array![
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let u = array![1., 0.]; // Control inputs

    let mut belief = (x0, p0);

    draw(&board, &belief.0, &belief.1);
    belief = predict(&belief.0, &belief.1, &f, &u, &b, &q);
    draw(&board, &belief.0, &belief.1);
    belief = predict(&belief.0, &belief.1, &f, &u, &b, &q);
    draw(&board, &belief.0, &belief.1);
}
