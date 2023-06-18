use std::collections::HashMap;
use std::convert::From;

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
    test: bool,
}

impl IpipNeo {
    pub fn new(question: i32, test: bool) -> Result<Self, &'static str> {
        let nquestion = match question {
            120 => QuestionNumber::Ipip120,
            300 => QuestionNumber::Ipip300,
            _ => return Err("Invalid question type!"),
        };

        Ok(IpipNeo { nquestion, test })
    }

    pub fn evaluator(
        &self,
        sex: &str,
        age: u32,
        score: &Vec<u32>,
    ) -> Result<HashMap<String, u32>, &'static str> {
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
