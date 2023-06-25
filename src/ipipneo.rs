// Author: Ederson Corbari
// Email: e@NeuroQuest.ai
// Copyright: Copyright NeuroQuest 2022-2023, Big 5 Personality Traits
// Credits: John A. Johnson, Dhiru Kholia
// License: MIT
// Version: 1.1.0
// Status: development

use std::collections::HashMap;
use std::convert::From;
use std::error::Error;


use crate::{model::norm::Norm, model::norm::NormScale, model::define::NormType, model::define::QuestionNumber, model::define::FacetLevel};
use crate::{model::facet::Facet};
use crate::{utility::common::err_if_sex_is_invalid, utility::common::err_if_age_is_invalid};

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
            Ok(_valid) => {
            }
            Err(err) => {
                println!("Ops... sex is invalid: {}", err);
            }
        }

        match err_if_age_is_invalid(age) {
            Ok(_valid) => {
            }
            Err(err) => {
                println!("Ops... age is invalid: {}", err);
            }
        }

        let mut normc: HashMap<char, f64> = HashMap::new();

        match Norm::new(sex, age, NormType::Item120) {
            Ok(norm) => {
                // println!("Norm id: {}", norm.get_id());
                // println!("Norm category: {}", norm.get_category());
                // println!("Norm ns: {:?}", norm.get_ns());

                normc = norm.calc(&score);
                println!("Norm calc: {:?}", normc);

                // let percent: HashMap<char, f64> = norm.percent(&normc);
                // println!("Norm percent: {:?}", percent);

                // let normalize: HashMap<char, f64> = NormScale.normalize(&normc, &percent);
                // println!("Norm scale: {:?}", normalize)
            }
            Err(err) => {
                println!("An error occurred while applying the Norm: {}", err);
            }
        }

        let facet: Result<Facet, Box<dyn Error>> = Facet::new(QuestionNumber::Ipip120);

        let res: Vec<i32> = score
        .values()
        .map(|value| *value as i32)
        .collect();
    
        let b5create: HashMap<String, Vec<i32>> = facet.expect("Error trying: b5create").b5create(&res);
        println!("b5create = {:?}", b5create);

        // let distrib = facet.expect("Error create distrib").distrib(&res);
        // println!("Domain = {:?}", distrib);
    

        let mut result: HashMap<String, u32> = HashMap::new();
        result.insert("O".to_string(), 1);
        result.insert("C".to_string(), 2);

        Ok(result)
    }
}
