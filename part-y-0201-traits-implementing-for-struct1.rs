fn main() {

    let mut dep_fintech = Department::new(Departments::FinTech, "Aykan Bilir".to_string(), "03_dep_fintech", 12_500_000, 17);

    dep_fintech.set_yearly_budget(500_000);

    dep_fintech.print_dep_info(); // Department: FinTech - Head: Aykan Bilir - Department Yearly Budget $: 13000000
}

trait Managerial {
    
    fn set_head(&mut self, head_name: String);

    fn set_yearly_budget(&mut self, amount: u64);

    fn set_ep_num(&mut self, num_0f_emp: u8);
}

#[derive(Debug)]
enum Departments {

    Sales,
    Finance,
    FinTech,
    Operations,
    HR,
    Auditing
}

#[derive(Debug)]
struct Department {

    dep_name: Departments,
    dep_head: String,
    dep_id: &'static str,
    dep_yearly_budget: u64,
    dep_emp_counts: u8
}

impl Department {
    
    fn new(dep_name: Departments, dep_head: String, dep_id: &'static str, dep_yearly_budget: u64, dep_emp_counts: u8) -> Self {

        Department { dep_name, dep_head, dep_id, dep_yearly_budget, dep_emp_counts }
    }

    fn print_dep_info(&self) {

        println!("Department: {:?} - Head: {} - Department Yearly Budget $: {}", self.dep_name, self.dep_head, self.dep_yearly_budget)
    }
}

impl Managerial for Department {
    
    fn set_head(&mut self, head_name: String) {
        
        self.dep_head = head_name
    }

    fn set_ep_num(&mut self, num_0f_emp: u8) {

           self.dep_emp_counts += num_0f_emp
    }

    fn set_yearly_budget(&mut self, amount: u64) {
        
        self.dep_yearly_budget += amount
    }
}
