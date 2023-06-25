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


use crate::{model::norm::Norm, model::define::NormType, model::define::QuestionNumber, model::define::FacetLevel};
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
                println!("Error: {}", err);
            }
        }

        match Norm::new(sex, age, NormType::Item120) {
            Ok(norm) => {
                println!("Norm ID: {}", norm.get_id());
                println!("Norm Category: {}", norm.get_category());
                println!("Norm Values: {:?}", norm.get_ns());

                let normc: HashMap<char, f64> = norm.calc(&score);
                println!("Norm Calc: {:?}", normc);

                let percent: HashMap<char, f64> = norm.percent(&normc);
                println!("Norm Percent: {:?}", percent);

                // let normalize = norm::NormScale.normalize(&normc, &percent);
                // println!("Norm Scale: {:?}", normalize)
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        println!("My age is {}", age);
        println!("My list is: {:?}", score);

        let mut result: HashMap<String, u32> = HashMap::new();
        result.insert("O".to_string(), 1);
        result.insert("C".to_string(), 2);

        Ok(result)
    }
}
