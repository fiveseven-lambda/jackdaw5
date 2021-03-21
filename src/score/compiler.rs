use super::parser::{Map, Notes};
use super::Score;
use crate::pos::Pos;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum CompileError<'s, 'p> {
    #[error("illegal literal {0} at {1} ({2})")]
    IllegalLiteral(&'s str, &'p Pos, std::num::ParseFloatError),
    #[error("undefined variable {0} at {1}")]
    UndefinedVariable(&'s str, &'p Pos),
    #[error("no `main` score")]
    NoMainScore,
}

fn compile_map<'s, 'p, 'm>(
    map: &'m Map<'s, 'p>,
    variables: &HashMap<&'s str, Score<'s>>,
) -> Result<Score<'s>, CompileError<'s, 'p>> {
    let mut score = match &map.notes {
        Notes::Note(numerator, denominator) => {
            let mut parameters = HashMap::new();
            let numerator = match numerator {
                Some((s, pos)) => match s.parse() {
                    Ok(val) => val,
                    Err(err) => return Err(CompileError::IllegalLiteral(s, pos, err)),
                },
                None => 1.,
            };
            let denominator = match denominator {
                Some((s, pos)) => match s.parse() {
                    Ok(val) => val,
                    Err(err) => return Err(CompileError::IllegalLiteral(s, pos, err)),
                },
                None => 1.,
            };
            parameters.insert("f", numerator / denominator);
            Score::Note(parameters)
        }
        Notes::Identifier(identifier, pos) => match variables.get(identifier) {
            Some(score) => score.clone(),
            None => return Err(CompileError::UndefinedVariable(identifier, pos)),
        },
        Notes::Row(maps) => {
            let mut vec = Vec::new();
            for map in maps {
                vec.push(compile_map(map, variables)?);
            }
            Score::Row(vec)
        }
        Notes::Column(maps) => {
            let mut vec = Vec::new();
            for map in maps {
                vec.push(compile_map(map, variables)?);
            }
            Score::Column(vec)
        }
    };
    for function in &map.functions {
        score.map(function)?;
    }
    Ok(score)
}

pub fn compile<'s, 'p, 'm>(
    maps: &'m [(&'s str, Map<'s, 'p>)],
) -> Result<Score<'s>, CompileError<'s, 'p>> {
    let mut ret = HashMap::new();
    for (variable, map) in maps {
        let score = compile_map(map, &ret)?;
        ret.insert(variable, score);
    }
    match ret.remove("main") {
        Some(main) => Ok(main),
        None => Err(CompileError::NoMainScore),
    }
}
