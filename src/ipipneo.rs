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

//use crate::utility::err_if_sex_is_invalid;
//use utility::err_if_age_is_invalid;
//use crate::utility::err_if_sex_is_invalid;

//use crate::reverse::scored_120;
//mod reverse;

// use crate::model::norm::Norm;

use crate::{model::norm::Norm, model::define::NormType};
//use crate::model::norm::Norm;
//use crate::model::define::NormType;


//use crate::error::AppError;
//use crate::model::norm::Norm;
//use crate::model::norm::NormType;


#[derive(Debug, PartialEq, Eq)]
enum QuestionNumber {
    Ipip300 = 300,
    Ipip120 = 120,
}

#[derive(Debug, PartialEq, Eq)]
enum FacetLevel {
    High = 55,
    Low = 45,
}

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
        age: u32,
        score: &Vec<u32>,
    ) -> Result<HashMap<String, u32>, &'static str> {
        // norm = Norm(sex=sex, age=age, nquestion=self._nquestion)
        // assert isinstance(norm, dict), "norm must be a dict"

        // Norms
        match Norm::new("M", 38, NormType::Item120) {
            Ok(norm) => {
                println!("Norm ID: {}", norm.get_id());
                println!("Norm Category: {}", norm.get_category());
                println!("Norm Values: {:?}", norm.get_ns());

                // normc = norm.calc(&domain);
                // println!("Norm Calc: {:?}", normc);

                // let percent = norm.percent(&normc);
                // println!("Norm Percent: {:?}", percent);

                // let normalize = norm::NormScale.normalize(&normc, &percent);
                // println!("Norm Scale: {:?}", normalize)
            }
            Err(err) => {
                println!("Error: {}", err);
            }
        }

        if sex == "M" {
            println!("My age is {}", age);
            println!("My list is: {:?}", score);
        }

        let mut result: HashMap<String, u32> = HashMap::new();
        result.insert("O".to_string(), 1);
        result.insert("C".to_string(), 2);

        Ok(result)
    }
}
