use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum Big5Neuroticism {
    TRAIT1,
    TRAIT2,
    TRAIT3,
    TRAIT4,
    TRAIT5,
    TRAIT6,
}

#[derive(Debug, PartialEq, Eq)]
enum Big5Extraversion {
    TRAIT1,
    TRAIT2,
    TRAIT3,
    TRAIT4,
    TRAIT5,
    TRAIT6,
}

#[derive(Debug, PartialEq, Eq)]
enum Big5Openness {
    TRAIT1,
    TRAIT2,
    TRAIT3,
    TRAIT4,
    TRAIT5,
    TRAIT6,
}

#[derive(Debug, PartialEq, Eq)]
enum Big5Agreeableness {
    TRAIT1,
    TRAIT2,
    TRAIT3,
    TRAIT4,
    TRAIT5,
    TRAIT6,
}

#[derive(Debug, PartialEq, Eq)]
enum Big5Conscientiousness {
    TRAIT1,
    TRAIT2,
    TRAIT3,
    TRAIT4,
    TRAIT5,
    TRAIT6,
}

fn big5_ocean_is_valid(label: &str) -> Result<(), &'static str> {
    match label {
        "O" | "C" | "E" | "A" | "N" => Ok(()),
        _ => Err("The Big-Five label is invalid!"),
    }
}

fn big5_target(label: &str) -> Result<HashMap<String, String>, &'static str> {
    big5_ocean_is_valid(label)?;

    let traits = match label {
        "O" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Openness::TRAIT1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits

            // let mut traits = HashMap::new();
            // traits.insert("trait".to_string(), "1".to_string());
            // traits.insert(Big5Openness::TRAIT1.to_string(), "imagination".to_string());
            // traits.insert("score".to_string(), "y[1]".to_string());
            // traits
        },
        "C" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Conscientiousness::TRAIT1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        },
        "E" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Extraversion::TRAIT1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        },
        "A" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Agreeableness::TRAIT1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        },
        "N" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Neuroticism::TRAIT1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        },
        _ => return Err("The Big-Five label is invalid!"),
    };

    Ok(traits)
}

fn create_big5_dict(label: &str, big5: f32, x: &[f32], y: &[f32]) -> Result<HashMap<String, Vec<HashMap<String, String>>>, &'static str> {
    let mut traits = vec![];

    for i in 1..=6 {
        let trait_name = match label {
            "O" => Big5Openness::from(i).to_string(),
            "C" => Big5Conscientiousness::from(i).to_string(),
            "E" => Big5Extraversion::from(i).to_string(),
            "A" => Big5Agreeableness::from(i).to_string(),
            "N" => Big5Neuroticism::from(i).to_string(),
            _ => return Err("The Big-Five label is invalid!"),
        };

        let trait_entry = hashmap!{
            "trait".to_string() => i.to_string(),
            trait_name => x[i].to_string(),
            "score".to_string() => y[i].to_string(),
        };

        traits.push(trait_entry);
    }

    let mut big5_dict = HashMap::new();
    big5_dict.insert(label.to_string(), vec![hashmap!{
        label.to_string() => big5.to_string(),
        "traits".to_string() => traits,
    }]);

    Ok(big5_dict)
}

fn organize_list_json(answers: &serde_json::Value) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    assert!(answers.is_object(), "The 'answers' field must be a dictionary!");

    let answers_array = answers.get("answers").ok_or("The key named 'answers' was not found!")?;

    let sorted_answers = answers_array
        .as_array()
        .ok_or("The key named 'answers' must be an array!")?
        .iter()
        .filter_map(|x| {
            let id_select = x.get("id_select")?;
            let id_select = id_select.as_i64().and_then(|x| x.try_into().ok())?;

            let id_question = x.get("id_question")?;
            let id_question = id_question.as_i64().and_then(|x| x.try_into().ok())?;

            if id_select >= 1 {
                Some((id_question, id_select))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sorted_answers.sort_by_key(|x| x.0);

    let organized_list = sorted_answers.into_iter().map(|x| x.1).collect::<Vec<_>>();

    Ok(organized_list)
}


fn add_dict_footer() -> HashMap<String, String> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    hashmap!{
        "library".to_string() => "five-factor-e".to_string(),
        "version".to_string() => "1".to_string(),
        "date".to_string() => now,
    }
}


