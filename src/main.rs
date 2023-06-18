// Author: Ederson Corbari
// Email: e@NeuroQuest.ai
// Copyright: Copyright NeuroQuest 2022-2023, Big 5 Personality Traits
// Credits: John A. Johnson, Dhiru Kholia
// License: MIT
// Version: 1.1.0
// Status: development

// use std::collections::HashMap;
// use std::error::Error;

mod ipipneo;

fn main() {
    match ipipneo::IpipNeo::new(120) {
        Ok(ipip) => {
            println!("==> Data");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
