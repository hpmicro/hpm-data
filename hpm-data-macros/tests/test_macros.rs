#![allow(dead_code)]

use hpm_data_macros::EnumDebug;

#[derive(Debug)]
struct A {
    pub b: String,
}

#[derive(EnumDebug)]
enum C {
    D(A),
    E,
}
