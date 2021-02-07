use super::*;

pub fn first_clause(s :&Vec<String>)->Result<usize, &'static str> {
    if s.len() <= 1 { return Err("wrong clause"); }
    let subject = first_phrase(&s, true, false)?;

    if s.len() > subject + 2 && regi(&s[subject + 2], "^(to|of|with|about|for|:|->)$") {
        Ok(subject + first_phrase(&s[subject+3..].to_vec(), true, true)? + 3)
    }
    else {
        Ok(subject + 1)
    }
}