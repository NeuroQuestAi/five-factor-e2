use std::collections::HashMap;
use std::error::Error;

use crate::{model::define::FacetScale, model::define::QuestionNumber};

#[derive(Debug, PartialEq, Eq)]
enum FacetLevel {
    High = 55,
    Low = 45,
}

#[derive(Clone)]
pub struct Facet {
    scale: Option<i32>,
}

impl Facet {
    pub fn new(nquestion: QuestionNumber) -> Result<Self, Box<dyn Error>> {
        let scale_mapping: HashMap<QuestionNumber, i32> = [
            (QuestionNumber::Ipip300, FacetScale::Ipip300 as i32),
            (QuestionNumber::Ipip120, FacetScale::Ipip120 as i32),
        ]
        .iter()
        .cloned()
        .collect();

        let scale = scale_mapping.get(&nquestion).cloned();

        if scale.is_none() {
            return Err(format!("The available questions are: {:?}", 0).into());
        }

        Ok(Self { scale })
    }

    pub fn score(&self, answers: &mut Vec<i32>) -> Result<Vec<i32>, Box<dyn Error>> {
        if answers.len() < self.scale.unwrap() as usize {
            return Err("The number of questions setting is wrong".into());
        }

        answers.insert(0, 0);
        let mut ss: Vec<i32> = vec![0; answers.len()];

        for j in 0..FacetScale::IpipMax as usize {
            for i in 0..self.scale.unwrap() as usize {
                ss[j + 1] += answers[1 + i * FacetScale::IpipMax as usize + j];
            }
        }

        Ok(ss)
    }

    pub fn b5create(&self, score: &Vec<i32>) -> std::collections::HashMap<String, Vec<i32>> {
        let mut ss: Vec<i32> = score.to_owned();
        let mut j: usize = 0;

        let mut n: Vec<i32> = vec![0; ss.len()];
        let mut e: Vec<i32> = vec![0; ss.len()];
        let mut o: Vec<i32> = vec![0; ss.len()];
        let mut a: Vec<i32> = vec![0; ss.len()];
        let mut c: Vec<i32> = vec![0; ss.len()];

        for i in 1..=6 {
            n[i] = ss[i + j];
            e[i] = ss[i + j + 1];
            o[i] = ss[i + j + 2];
            a[i] = ss[i + j + 3];
            c[i] = ss[i + j + 4];
            j += 4;
        }

        let mut result = std::collections::HashMap::new();
        result.insert("O".to_string(), o);
        result.insert("C".to_string(), c);
        result.insert("E".to_string(), e);
        result.insert("A".to_string(), a);
        result.insert("N".to_string(), n);

        result
    }

    pub fn domain(&self, score: &Vec<i32>) -> HashMap<char, f64> {
        let n: i32 = score[1] + score[6] + score[11] + score[16] + score[21] + score[26];
        let e: i32 = score[2] + score[7] + score[12] + score[17] + score[22] + score[27];
        let o: i32 = score[3] + score[8] + score[13] + score[18] + score[23] + score[28];
        let a: i32 = score[4] + score[9] + score[14] + score[19] + score[24] + score[29];
        let c: i32 = score[5] + score[10] + score[15] + score[20] + score[25] + score[30];

        let result: HashMap<char, f64> = [
            ('O', o as f64),
            ('C', c as f64),
            ('E', e as f64),
            ('A', a as f64),
            ('N', n as f64),
        ]
        .iter()
        .cloned()
        .collect();

        result
    }

    pub fn distrib(
        &self,
        size: usize,
        b5: &HashMap<char, Vec<i32>>,
        norm: &HashMap<String, Vec<i32>>,
    ) -> HashMap<char, Vec<i32>> {
        let calculate_score = |score: i32, norm_values: &[i32], i: usize, j: usize| {
            50 + (10 * (score - norm_values[i + j]) / norm_values[i + j + 6])
        };

        let mut result: HashMap<char, Vec<i32>> = HashMap::new();

        let keys: [char; 5] = ['O', 'C', 'E', 'A', 'N'];

        for key in keys.iter() {
            let b5_scores = b5.get(key).unwrap();
            let norm_values = norm.get("ns").unwrap();

            let scores: Vec<i32> = (1..=6)
                .map(|i: usize| calculate_score(b5_scores[i], norm_values, (i - 1) * 6, 10))
                .collect();

            result.insert(*key, scores);
        }

        result
    }

    fn big_five_level(
        &self,
        big5: &HashMap<String, u32>,
        label: &str,
    ) -> Result<HashMap<String, u32>, &'static str> {
        let mut big5_clone = big5.clone();
        let score: &u32 = big5.get(label).ok_or("Invalid Big-Five label")?;

        let new_score: u32 = match *score {
            score if score < FacetLevel::Low as u32 => 0,
            score if score <= FacetLevel::High as u32 => 1,
            _ => 2,
        };

        big5_clone.insert("score".to_string(), new_score);

        Ok(big5_clone)
    }
}
