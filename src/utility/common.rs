
use chrono::prelude::*;
use std::format;
use std::convert::TryInto;
use std::collections::HashMap;
use maplit::hashmap;

use crate::{
    model::define::Big5Agreeableness, model::define::Big5Conscientiousness,
    model::define::Big5Extraversion, model::define::Big5Neuroticism, model::define::Big5Openness,
};


pub fn err_if_sex_is_invalid(sex: &str) -> Result<bool, Box<dyn std::error::Error>> {
    if sex.is_empty() {
        return Err("The (sex) field is required!".into());
    }

    assert!(
        sex == "M" || sex == "F",
        "The (sex) field must contain (M or F)!"
    );

    Ok(true)
}

pub fn raise_if_age_is_invalid(age: i32) -> Result<bool, Box<dyn std::error::Error>> {
    if age <= 0 {
        return Err("The (age) field is required!".into());
    }

    if !(age >= 10 && age <= 110) {
        return Err(format!("The age ({}) must be between 10 and 110!", age.to_string()).into());
    }

    Ok(true)
}

pub fn big5_ocean_is_valid(label: &str) -> Result<(), &'static str> {
    match label {
        "O" | "C" | "E" | "A" | "N" => Ok(()),
        _ => Err("The Big-Five label is invalid!"),
    }
}

pub fn big5_target(label: &str) -> Result<HashMap<String, String>, &'static str> {
    big5_ocean_is_valid(label)?;

    let traits = match label {
        "O" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Openness::Trait1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits

            // let mut traits = HashMap::new();
            // traits.insert("trait".to_string(), "1".to_string());
            // traits.insert(Big5Openness::TRAIT1.to_string(), "imagination".to_string());
            // traits.insert("score".to_string(), "y[1]".to_string());
            // traits
        }
        "C" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(
                Big5Conscientiousness::Trait1.to_string(),
                "x[1]".to_string(),
            );
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        }
        "E" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Extraversion::Trait1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        }
        "A" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Agreeableness::Trait1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        }
        "N" => {
            let mut traits = HashMap::new();
            traits.insert("trait".to_string(), "1".to_string());
            traits.insert(Big5Neuroticism::Trait1.to_string(), "x[1]".to_string());
            traits.insert("score".to_string(), "y[1]".to_string());
            traits
        }
        _ => return Err("The Big-Five label is invalid!"),
    };

    Ok(traits)
}

pub fn create_big5_dict(
    label: &str,
    big5: f32,
    x: &[f32],
    y: &[f32],
) -> Result<HashMap<String, Vec<HashMap<String, String>>>, &'static str> {
    let mut traits: Vec<HashMap<String, String>> = vec![];

    for i in 1..=6 {
        let trait_name: &str = match label {
            "O" => "1",
            "C" => "2", // &Big5Conscientiousness::from(i).to_string(),
            "E" => "3", // &Big5Extraversion::from(i).to_string(),
            "A" => "4", // &Big5Agreeableness::from(i).to_string(),
            "N" => "5", // &Big5Neuroticism::from(i).to_string(),
            _ => return Err("The Big-Five label is invalid!"),
        };

        let mut trait_entry = HashMap::new();
        trait_entry.insert("trait".to_string(), i.to_string());
        trait_entry.insert(trait_name.to_string(), x[i].to_string());
        trait_entry.insert("score".to_string(), y[i].to_string());

        traits.push(trait_entry);
    }

    let mut big5_dict: HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
    let mut inner_vec: Vec<HashMap<String, String>> = Vec::new();
    let mut inner_dict: HashMap<String, String> = HashMap::new();
    inner_dict.insert(label.to_string(), big5.to_string());
    inner_dict.insert("traits".to_string(), serde_json::to_string(&traits).unwrap());
    inner_vec.push(inner_dict);
    big5_dict.insert(label.to_string(), inner_vec);

    Ok(big5_dict)
}

pub fn organize_list_json(answers: &serde_json::Value) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    assert!(
        answers.is_object(),
        "The 'answers' field must be a dictionary!"
    );

    let answers_array: &serde_json::Value = answers
        .get("answers")
        .ok_or("The key named 'answers' was not found!")?;

    let mut sorted_answers: Vec<(i32, i32)> = answers_array
        .as_array()
        .ok_or("The key named (answers) must be an array!")?
        .iter()
        .filter_map(|x: &serde_json::Value| {
            let id_select: &serde_json::Value = x.get("id_select")?;
            let id_select: i32 = id_select.as_i64().and_then(|x: i64| x.try_into().ok())?;
            let id_question: &serde_json::Value = x.get("id_question")?;
            let id_question: i32 = id_question.as_i64().and_then(|x: i64| x.try_into().ok())?;
            if id_select >= 1 {
                Some((id_question, id_select))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    sorted_answers.sort_by_key(|x: &(i32, i32)| x.0);

    let organized_list: Vec<i32> = sorted_answers.into_iter().map(|x: (i32, i32)| x.1).collect::<Vec<_>>();

    Ok(organized_list)
}

pub fn add_dict_footer() -> HashMap<String, String> {
    hashmap! {
        "library".to_string() => "five-factor-e2".to_string(),
        "version".to_string() => "1.0.0".to_string(),
        "date".to_string() => Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }
}
