use std::collections::HashMap;

#[derive(Debug, Clone)] 
pub struct Bill {
    pub name: String,
    pub amount: f64
}

pub struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn add (&mut self, bill: Bill) {
        self.inner.insert(bill.name.to_string(), bill);
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.inner.values().collect()
    }

    pub fn remove(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
    }

    pub fn update(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name) {
            Some(bill) => {
                bill.amount = amount;
                true
            },
            None => false,
        }
    }
}
