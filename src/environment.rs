use anyhow::{anyhow, Error, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{interpreter::RuntimeError, token::Token, types::LitVal};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, LitVal>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn get(&self, name: &Token) -> Result<LitVal, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(enc) => enc.borrow().get(name),
                None => Err(RuntimeError::new(
                    name.clone(),
                    &format!("Undefined variable '{}'.", name.lexeme),
                )),
            },
        }
    }

    pub fn assign(&mut self, name: &Token, value: &LitVal) -> Result<LitVal, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            Ok(value.clone())
        } else if let Some(enc) = &self.enclosing {
            enc.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError::new(
                name.clone(),
                &format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn define(&mut self, name: String, value: LitVal) {
        self.values.insert(name, value);
    }
}
