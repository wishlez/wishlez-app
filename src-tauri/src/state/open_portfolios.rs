use crate::data::portfolio::Portfolio;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type PortfolioEntry = (String, Arc<Mutex<Portfolio>>);

#[derive(Default)]
pub struct OpenPortfolios(Mutex<HashMap<String, PortfolioEntry>>);

impl OpenPortfolios {
    pub fn insert(&self, label: &str, path: &str, portfolio: Portfolio) {
        self.0.lock().unwrap().insert(
            label.to_string(),
            (path.to_string(), Arc::new(Mutex::new(portfolio))),
        );
    }

    pub fn remove(&self, label: &str) {
        self.0.lock().unwrap().remove(label);
    }

    pub fn remove_and_check_empty(&self, label: &str) -> bool {
        let mut open = self.0.lock().unwrap();
        open.remove(label);
        open.is_empty()
    }

    pub fn get(&self, label: &str) -> Option<Arc<Mutex<Portfolio>>> {
        self.0
            .lock()
            .unwrap()
            .get(label)
            .map(|(_, portfolio)| Arc::clone(portfolio))
    }

    pub fn get_label_from_path(&self, path: &str) -> Option<String> {
        let target = std::fs::canonicalize(path).ok()?;
        self.0
            .lock()
            .unwrap()
            .iter()
            .find(|(_, (open_path, _))| {
                std::fs::canonicalize(open_path).ok().as_ref() == Some(&target)
            })
            .map(|(label, _)| label.clone())
    }
}
