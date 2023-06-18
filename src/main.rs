use std::collections::HashMap;
use std::error::Error;

mod facet;
mod ipipneo;
mod model;
mod norm;
mod reverse;

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

    // ------------- IPIP-NEO ---------------------------
    match ipipneo::IpipNeo::new(120, false) {
        Ok(ipip) => {
            println!("==> Data");
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    // -------------- REVERSE -----------------------------
    let input = r#"
    {
        "answers":[
           {
              "id_question":1,
              "id_select":5
           },
           {
              "id_question":2,
              "id_select":2
           },
           {
              "id_question":3,
              "id_select":3
           },
           {
              "id_question":4,
              "id_select":2
           },
           {
              "id_question":5,
              "id_select":5
           },
           {
              "id_question":6,
              "id_select":4
           },
           {
              "id_question":7,
              "id_select":1
           },
           {
              "id_question":8,
              "id_select":3
           },
           {
              "id_question":9,
              "id_select":5
           },
           {
              "id_question":10,
              "id_select":5
           },
           {
              "id_question":11,
              "id_select":3
           },
           {
              "id_question":12,
              "id_select":2
           },
           {
              "id_question":13,
              "id_select":2
           },
           {
              "id_question":14,
              "id_select":4
           },
           {
              "id_question":15,
              "id_select":5
           },
           {
              "id_question":16,
              "id_select":2
           },
           {
              "id_question":17,
              "id_select":5
           },
           {
              "id_question":18,
              "id_select":4
           },
           {
              "id_question":19,
              "id_select":2
           },
           {
              "id_question":20,
              "id_select":5
           },
           {
              "id_question":21,
              "id_select":2
           },
           {
              "id_question":22,
              "id_select":4
           },
           {
              "id_question":23,
              "id_select":5
           },
           {
              "id_question":24,
              "id_select":2
           },
           {
              "id_question":25,
              "id_select":4
           },
           {
              "id_question":26,
              "id_select":1
           },
           {
              "id_question":27,
              "id_select":2
           },
           {
              "id_question":28,
              "id_select":5
           },
           {
              "id_question":29,
              "id_select":4
           },
           {
              "id_question":30,
              "id_select":4
           },
           {
              "id_question":31,
              "id_select":2
           },
           {
              "id_question":32,
              "id_select":3
           },
           {
              "id_question":33,
              "id_select":4
           },
           {
              "id_question":34,
              "id_select":5
           },
           {
              "id_question":35,
              "id_select":5
           },
           {
              "id_question":36,
              "id_select":4
           },
           {
              "id_question":37,
              "id_select":2
           },
           {
              "id_question":38,
              "id_select":2
           },
           {
              "id_question":39,
              "id_select":5
           },
           {
              "id_question":40,
              "id_select":5
           },
           {
              "id_question":41,
              "id_select":2
           },
           {
              "id_question":42,
              "id_select":2
           },
           {
              "id_question":43,
              "id_select":4
           },
           {
              "id_question":44,
              "id_select":3
           },
           {
              "id_question":45,
              "id_select":4
           },
           {
              "id_question":46,
              "id_select":4
           },
           {
              "id_question":47,
              "id_select":4
           },
           {
              "id_question":48,
              "id_select":4
           },
           {
              "id_question":49,
              "id_select":5
           },
           {
              "id_question":50,
              "id_select":4
           },
           {
              "id_question":51,
              "id_select":2
           },
           {
              "id_question":52,
              "id_select":4
           },
           {
              "id_question":53,
              "id_select":3
           },
           {
              "id_question":54,
              "id_select":3
           },
           {
              "id_question":55,
              "id_select":4
           },
           {
              "id_question":56,
              "id_select":5
           },
           {
              "id_question":57,
              "id_select":2
           },
           {
              "id_question":58,
              "id_select":4
           },
           {
              "id_question":59,
              "id_select":1
           },
           {
              "id_question":60,
              "id_select":3
           },
           {
              "id_question":61,
              "id_select":1
           },
           {
              "id_question":62,
              "id_select":3
           },
           {
              "id_question":63,
              "id_select":1
           },
           {
              "id_question":64,
              "id_select":4
           },
           {
              "id_question":65,
              "id_select":4
           },
           {
              "id_question":66,
              "id_select":2
           },
           {
              "id_question":67,
              "id_select":2
           },
           {
              "id_question":68,
              "id_select":3
           },
           {
              "id_question":69,
              "id_select":5
           },
           {
              "id_question":70,
              "id_select":5
           },
           {
              "id_question":71,
              "id_select":2
           },
           {
              "id_question":72,
              "id_select":4
           },
           {
              "id_question":73,
              "id_select":4
           },
           {
              "id_question":74,
              "id_select":4
           },
           {
              "id_question":75,
              "id_select":3
           },
           {
              "id_question":76,
              "id_select":1
           },
           {
              "id_question":77,
              "id_select":2
           },
           {
              "id_question":78,
              "id_select":4
           },
           {
              "id_question":79,
              "id_select":5
           },
           {
              "id_question":80,
              "id_select":5
           },
           {
              "id_question":81,
              "id_select":3
           },
           {
              "id_question":82,
              "id_select":2
           },
           {
              "id_question":83,
              "id_select":2
           },
           {
              "id_question":84,
              "id_select":2
           },
           {
              "id_question":85,
              "id_select":5
           },
           {
              "id_question":86,
              "id_select":1
           },
           {
              "id_question":87,
              "id_select":4
           },
           {
              "id_question":88,
              "id_select":4
           },
           {
              "id_question":89,
              "id_select":2
           },
           {
              "id_question":90,
              "id_select":1
           },
           {
              "id_question":91,
              "id_select":4
           },
           {
              "id_question":92,
              "id_select":2
           },
           {
              "id_question":93,
              "id_select":2
           },
           {
              "id_question":94,
              "id_select":4
           },
           {
              "id_question":95,
              "id_select":3
           },
           {
              "id_question":96,
              "id_select":2
           },
           {
              "id_question":97,
              "id_select":1
           },
           {
              "id_question":98,
              "id_select":3
           },
           {
              "id_question":99,
              "id_select":5
           },
           {
              "id_question":100,
              "id_select":5
           },
           {
              "id_question":101,
              "id_select":1
           },
           {
              "id_question":102,
              "id_select":4
           },
           {
              "id_question":103,
              "id_select":2
           },
           {
              "id_question":104,
              "id_select":2
           },
           {
              "id_question":105,
              "id_select":4
           },
           {
              "id_question":106,
              "id_select":4
           },
           {
              "id_question":107,
              "id_select":3
           },
           {
              "id_question":108,
              "id_select":4
           },
           {
              "id_question":109,
              "id_select":4
           },
           {
              "id_question":110,
              "id_select":5
           },
           {
              "id_question":111,
              "id_select":3
           },
           {
              "id_question":112,
              "id_select":2
           },
           {
              "id_question":113,
              "id_select":4
           },
           {
              "id_question":114,
              "id_select":5
           },
           {
              "id_question":115,
              "id_select":5
           },
           {
              "id_question":116,
              "id_select":1
           },
           {
              "id_question":117,
              "id_select":4
           },
           {
              "id_question":118,
              "id_select":2
           },
           {
              "id_question":119,
              "id_select":4
           },
           {
              "id_question":120,
              "id_select":1
           }
        ]
     }     
    "#;

    // let data: serde_json::Value = serde_json::from_str(input).expect("Failed to parse JSON");
    // let answers = data["answers"].as_array().expect("'answers' should be an array");

    let data: serde_json::Value = serde_json::from_str(input).expect("Failed to parse JSON");
    let answers = data["answers"].as_array().expect("'answers' should be an array");

    let answers_vec: Vec<HashMap<String, u32>> = answers
        .iter()
        .map(|answer| {
            let id_question = answer["id_question"].as_u64().expect("'id_question' should be an integer") as u32;
            let id_select = answer["id_select"].as_u64().expect("'id_select' should be an integer") as u32;
            
            let mut map = HashMap::new();
            map.insert("id_question".to_string(), id_question);
            map.insert("id_select".to_string(), id_select);
            map
        })
        .collect();

    let reversed_answers = reverse::reverse_scored_120(answers_vec);

    match reversed_answers {
        Ok(answers) => println!("Reversed answers: {:?}", answers),
        Err(error) => println!("Error: {}", error),
    }

}
