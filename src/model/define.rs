use std::fmt;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum QuestionNumber {
    Ipip300 = 300,
    Ipip120 = 120,
}

impl fmt::Display for QuestionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Ipip300 => 300,
                Self::Ipip120 => 120,
            }
        )
    }
}

pub enum FacetScale {
    IpipMax = 30,
    Ipip300 = 10,
    Ipip120 = 4,
}

impl fmt::Display for FacetScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::IpipMax => 30,
                Self::Ipip300 => 10,
                Self::Ipip120 => 4,
            }
        )
    }
}

pub enum FacetLevel {
    High = 55,
    Low = 45,
}

impl fmt::Display for FacetLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::High => 55,
                Self::Low => 45,
            }
        )
    }
}

pub enum NormScale {
    ConstMax,
    ConstMin,
}

impl fmt::Display for NormScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ConstMax => 73,
                Self::ConstMin => 32,
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Big5Neuroticism {
    Trait1,
    Trait2,
    Trait3,
    Trait4,
    Trait5,
    Trait6,
}

impl fmt::Display for Big5Neuroticism {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trait1 => "anxiety",
                Self::Trait2 => "anger",
                Self::Trait3 => "depression",
                Self::Trait4 => "self_consciousness",
                Self::Trait5 => "immoderation",
                Self::Trait6 => "vulnerability",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Big5Extraversion {
    Trait1,
    Trait2,
    Trait3,
    Trait4,
    Trait5,
    Trait6,
}

impl fmt::Display for Big5Extraversion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trait1 => "friendliness",
                Self::Trait2 => "gregariousness",
                Self::Trait3 => "assertiveness",
                Self::Trait4 => "activity_level",
                Self::Trait5 => "excitement_seeking",
                Self::Trait6 => "cheerfulness",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Big5Openness {
    Trait1,
    Trait2,
    Trait3,
    Trait4,
    Trait5,
    Trait6,
}

impl fmt::Display for Big5Openness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trait1 => "imagination",
                Self::Trait2 => "artistic_interests",
                Self::Trait3 => "emotionality",
                Self::Trait4 => "adventurousness",
                Self::Trait5 => "intellect",
                Self::Trait6 => "liberalism",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Big5Agreeableness {
    Trait1,
    Trait2,
    Trait3,
    Trait4,
    Trait5,
    Trait6,
}

impl fmt::Display for Big5Agreeableness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trait1 => "trust",
                Self::Trait2 => "morality",
                Self::Trait3 => "altruism",
                Self::Trait4 => "cooperation",
                Self::Trait5 => "modesty",
                Self::Trait6 => "sympathy",
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Big5Conscientiousness {
    Trait1,
    Trait2,
    Trait3,
    Trait4,
    Trait5,
    Trait6,
}

impl fmt::Display for Big5Conscientiousness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Trait1 => "self_efficacy",
                Self::Trait2 => "orderliness",
                Self::Trait3 => "dutifulness",
                Self::Trait4 => "achievement_striving",
                Self::Trait5 => "self_discipline",
                Self::Trait6 => "cautiousness",
            }
        )
    }
}
