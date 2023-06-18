use std::collections::HashMap;

const IPIP_NEO_ITEMS_REVERSED_120: [u32; 55] = [
    9, 19, 24, 30, 39, 40, 48, 49, 51, 53, 54, 60, 62, 67, 68, 69, 70, 73, 74, 75, 78, 79, 80, 81,
    83, 84, 85, 88, 89, 90, 92, 94, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108,
    109, 110, 111, 113, 114, 115, 116, 118, 119, 120,
];

fn scored(select: u32) -> Option<u32> {
    match select {
        1 => Some(5),
        2 => Some(4),
        3 => Some(3),
        4 => Some(2),
        5 => Some(1),
        _ => None,
    }
}

fn is_reversed_120(x: u32, y: u32) -> u32 {
    if IPIP_NEO_ITEMS_REVERSED_120.contains(&x) {
        scored(y).unwrap_or(y)
    } else {
        y
    }
}

pub fn scored_120(
    answers: Vec<HashMap<String, u32>>,
) -> Result<Vec<HashMap<String, u32>>, &'static str> {
    //assert_eq!(answers.len(), 120, "The number of answers should be 120!");

    let reversed_answers: Vec<HashMap<String, u32>> = answers
        .into_iter()
        .map(|mut answer| {
            let id_question = answer.get("id_question").ok_or("Missing id_question");
            let id_select = answer.get("id_select").ok_or("Missing id_select");

            match (id_question, id_select) {
                (Ok(&x), Ok(&y)) => {
                    let reversed_select = is_reversed_120(x, y);
                    answer.insert("id_select".to_string(), reversed_select);
                    answer
                }
                _ => answer,
            }
        })
        .collect();

    Ok(reversed_answers)
}
