fn main() {

}

trait Financial {
    
    fn set_budget(&mut self, initial_budget: u64);

    fn update_budget(&mut self, budget_inc: u64);
}

struct City {

    city_name: &'static str,
    population: u64,
    general_budget: u64
}

impl City {

    fn new(city_name: &'static str, population: u64, general_budget: u64) -> Self {

        Self { city_name, population, general_budget }
    }
}

impl Financial for City {

    fn set_budget(&mut self, initial_budget: u64) {
        self.general_budget = initial_budget;
    }

    fn update_budget(&mut self, budget_inc: u64) {
        
        self.general_budget += budget_inc;
    }
}

struct Firm {

    firm_name: &'static str,
    num_of_emps: u16,
    general_budget: u64
}

impl Firm {
    
    fn new(firm_name: &'static str, num_of_emps: u16, general_budget: u64) -> Self {

        Self { firm_name, num_of_emps, general_budget }
    }
}

impl Financial for Firm {
    
    fn set_budget(&mut self, initial_budget: u64) {
        
        self.general_budget = initial_budget;
    }

    fn update_budget(&mut self, budget_inc: u64) {
        
        self.general_budget += budget_inc;
    }
}
