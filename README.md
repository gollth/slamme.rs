# SLAMme.RS

Playground for 2D EKF-SLAM as TUI in Rust

## Installation

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  # install RUST
$ sudo apt install libopenblas-dev # install dependencies for linalg crate
```

## Demo

```console
$ cd slamme.rs
$ cargo run
```

This brings renders the state of the world to the terminal (must support UTF8). Pressing enter will
advance the simulation. Currently the robot's movement is hardcoded to move just slowly to the right.

```
┌────────────────────────────────┐
│         ·                      │
│ ●     ·····                    │
│       ·····                    │
│      ···⊕···                   │
│       ·····                    │
│       ·····                    │
│         ·                      │
│                                │
│                                │
│                                │
│  ●                             │
│           ●                    │
│                                │
│                                │
│                                │
│                                │
└────────────────────────────────┘
x̂: [9, 3, 0, 0, 1.1639982, 1.0156813, 2.0108616, 9.93173, 11.211845, 10.725612]
P̂: [3.05, 3.05, 0.3, 0.3, 0.058541022, 0.058541022, 0.058541022, 0.058541022, 0.058541022, 0.058541022]
û: [1, 0]
```


## Legend

* `⊕` is the most likely position of the robot, i.e. x̂[0..1] or the mean of the _belief_
* `.` is other likely positions of the robot, i.e. inside the standard deviation ellipse of P̂[(0,0),(1,1)] (_belief_)
* `●` are obstacles in the world which are measured by the robot with a certain uncertainty
* `─` is the edge of the world (no obstacle)

## State Space

The state `x` is make up of the 2D robot's position, velocity and the 2D positions of K landmarks (currently still hardcoded in `main.rs`)


## References

* https://en.wikipedia.org/wiki/Kalman_filter
* https://www.iri.upc.edu/people/jsola/JoanSola/objectes/curs_SLAM/SLAM2D/SLAM%20course.pdf


## TODOs

* Make the controls configurable in the TUI, e.g. by pressing arrow keys
* Show the _belief_ of the landmarks, i.e. with `⃝` next to their actual position in the world
* Make the world configurable
