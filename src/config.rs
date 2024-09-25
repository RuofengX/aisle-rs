use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct Root{
    ins: Vec<In>,
    outs: Vec<Out>,
    routes: HashMap<String, String>
}

#[derive(Deserialize)]
struct In{
    trans: String,
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
struct Out{
    trans: String,
    ip: String,
    port: u16,
}

