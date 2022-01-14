use ndarray::{concatenate, prelude::*};
use slammers::{draw, predict, Matrix, Vector};
use std::io::stdin;
use termion::input::TermRead;

fn main() {
    const K: usize = 3; // number of landmarks
    const N: usize = K * 2; // number of landmark states (x,y per landmark)

    // initial state
    let x0 = concatenate![
        Axis(0),
        array![
            3.0f32, 3., // position
            0., 0., // velocity
        ],
        array![
            1., 1., // landmark 0
            10., 2., // landmark 1
            11., 11., // landmark 2
        ],
    ];
    println!("X: {}", x0);

    // initial covariance of robot state
    let prr0 = array![
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
    ];
    let p0 = concatenate![
        Axis(0),
        concatenate![Axis(1), prr0, Matrix::zeros((4, N))],
        concatenate![Axis(1), Matrix::zeros((N, 4)), Matrix::zeros((N, N))]
    ];
    println!("P: {}", p0);

    // Dynamics Model
    let f = concatenate![
        Axis(0),
        concatenate![
            Axis(1),
            array![
                [1., 0., 1., 0.],
                [0., 1., 0., 1.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]
            ],
            Matrix::zeros((4, N)),
        ],
        concatenate![Axis(1), Matrix::zeros((N, 4)), Matrix::eye(N)],
    ];
    println!("F: {}", f);

    // How do control inputs apply to state?
    let b = concatenate![
        Axis(0),
        array![[1., 0.], [0., 1.], [0., 0.], [0., 0.],],
        Matrix::zeros((N, 2)),
    ];
    println!("B: {}", b);

    // Observation Model
    // TODO

    // observation noise
    // TODO
    // let v = array![[1., 0., 0., 0.], [0., 1., 0., 0.]];

    // process noise
    let w = concatenate![
        Axis(0),
        array![0.01, 0.01, 0.01, 0.01], // robot state
        0.01 * Vector::ones(N),         // landmarks
    ];
    println!("w: {}", w);

    // Add some obstacles to the world
    let mut world = Array2::zeros((16, 32));
    for i in 0..K {
        let coord = (
            x0[4 + 2 * i].round() as usize,
            x0[4 + 2 * i + 1].round() as usize,
        );
        world[coord] = 1;
    }
    world[[10, 2]] = 2;
    world[[11, 11]] = 2;

    let u = array![1., 0.]; // Control inputs

    let mut belief = (x0, p0);

    loop {
        draw(0, &world, &belief.0, &belief.1, &u);
        stdin().keys().next();

        belief = predict(&belief.0, &belief.1, &f, &u, &b, &w);
    }
}
