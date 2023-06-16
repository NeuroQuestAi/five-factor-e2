use std::collections::HashMap;
use std::error::Error;

mod facet;
mod model;
mod norm;

fn main() {
    let question_number = model::QuestionNumber::Ipip120;
    println!("Question: {question_number}");

    let facet = facet::Facet::new(model::QuestionNumber::Ipip120);

    let mut answers = vec![
        5, 2, 3, 2, 5, 4, 1, 3, 5, 5, 3, 2, 2, 4, 5, 2, 5, 4, 2, 5, 2, 4, 5, 2, 4, 1, 2, 5, 4, 4,
        2, 3, 4, 5, 5, 4, 2, 2, 5, 5, 2, 2, 4, 3, 4, 4, 4, 4, 5, 4, 2, 4, 3, 3, 4, 5, 2, 4, 1, 3,
        1, 3, 1, 4, 4, 2, 2, 3, 5, 5, 2, 4, 4, 4, 3, 1, 2, 4, 5, 5, 3, 2, 2, 2, 5, 1, 4, 4, 2, 1,
        4, 2, 2, 4, 3, 2, 1, 3, 5, 5, 1, 4, 2, 2, 4, 4, 3, 4, 4, 5, 3, 2, 4, 5, 5, 1, 4, 2, 4, 4,
    ];
    println!("Answers = {:?}", answers);

    let score = facet.expect("Booommm").score(&mut answers);
    println!("Score = {:?}", score);

    let res: Vec<i32> = match score {
        Ok(inner_vec) => inner_vec,
        Err(err) => {
            println!("Error: {:?}", err);
            Vec::new()
        }
    };

    //let facet = facet::Facet::new(model::QuestionNumber::Ipip120);
    //let b5create = facet.expect("Booommm-1").b5create(&res);
    //println!("B5 = {:?}", b5create);
    println!(
        "Data {:?}",
        res[0] + res[5] + res[10] + res[15] + res[20] + res[25]
    );

    let facet1 = facet::Facet::new(model::QuestionNumber::Ipip120);
    let domain = facet1.expect("Error create domain").domain(&res);
    println!("Domain = {:?}", domain);

    let mut normc: HashMap<char, f64> = HashMap::new();

    // Norms
    match norm::Norm::new("M", 38, norm::NormType::Item120) {
        Ok(norm) => {
            println!("Norm ID: {}", norm.get_id());
            println!("Norm Category: {}", norm.get_category());
            println!("Norm Values: {:?}", norm.get_ns());

            normc = norm.calc(&domain);
            println!("Norm Calc: {:?}", normc);

            let percent = norm.percent(&normc);
            println!("Norm Percent: {:?}", percent);

            let normalize = norm::NormScale.normalize(&normc, &percent);
            println!("Norm Scale: {:?}", normalize)
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

}
