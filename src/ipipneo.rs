// Author: Ederson Corbari
// Email: e@NeuroQuest.ai
// Copyright: Copyright NeuroQuest 2022-2023, Big 5 Personality Traits
// Credits: John A. Johnson, Dhiru Kholia
// License: MIT
// Version: 1.1.0
// Status: development

use std::collections::HashMap;
// use std::convert::From;
use std::error::Error;

use crate::model::facet::Facet;
use crate::{
    model::define::FacetLevel, model::define::NormType, model::define::QuestionNumber,
    model::norm::Norm, model::norm::NormScale,
};
use crate::{
    utility::common::err_if_age_is_invalid, utility::common::err_if_sex_is_invalid,
    utility::common::to_hashmap_char_veci32, utility::common::to_hashmap_str_veci32,
};

pub struct IpipNeo {
    nquestion: QuestionNumber,
}

impl IpipNeo {
    pub fn new(question: i32) -> Result<Self, &'static str> {
        let nquestion = match question {
            120 => QuestionNumber::Ipip120,
            300 => QuestionNumber::Ipip300,
            _ => return Err("Invalid question type!"),
        };

        Ok(IpipNeo { nquestion })
    }

    pub fn evaluator(
        &self,
        sex: &str,
        age: i32,
        score: HashMap<char, f64>,
    ) -> Result<HashMap<String, u32>, &'static str> {
        match err_if_sex_is_invalid(sex) {
            Ok(_valid) => {}
            Err(err) => {
                println!("Ops... sex is invalid: {}", err);
            }
        }

        match err_if_age_is_invalid(age) {
            Ok(_valid) => {}
            Err(err) => {
                println!("Ops... age is invalid: {}", err);
            }
        }

        let mut normc: HashMap<char, f64> = HashMap::new();

        match Norm::new(sex, age, NormType::Item120) {
            Ok(norm) => {
                normc = norm.calc(&score);
            }
            Err(err) => {
                println!("An error occurred while applying the Norm: {}", err);
            }
        }

        let facet: Result<Facet, Box<dyn Error>> = Facet::new(QuestionNumber::Ipip120);

        let res: Vec<i32> = score.values().map(|value| *value as i32).collect();

        let b5: HashMap<String, Vec<i32>> = facet.expect("Error trying: b5create").b5create(&res);
        println!("b5create = {:?}", b5);

        let b52: HashMap<char, Vec<i32>> = to_hashmap_char_veci32(b5);
        let normc2: HashMap<String, Vec<i32>> = to_hashmap_str_veci32(normc);

        let facet2: Result<Facet, Box<dyn Error>> = Facet::new(QuestionNumber::Ipip120);
        let distrib: HashMap<char, Vec<i32>> = facet2.expect("Error create distrib").distrib(&b52, &normc2);
        println!("Domain = {:?}", distrib);

        let mut result: HashMap<String, u32> = HashMap::new();
        result.insert("O".to_string(), 1);
        result.insert("C".to_string(), 2);

        Ok(result)
    }
}
