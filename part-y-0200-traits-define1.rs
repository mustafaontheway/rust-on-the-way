fn main() {


}

trait Managerial {
    
    fn set_head(&mut self, dep: Departments, head_name: String);

    fn set_yearly_budget(&mut self, dep: Departments, amount: u64);

    fn set_ep_num(&mut self, dep: Departments, num_0f_emp: u8);
}

enum Departments {

    Sales,
    Finance,
    FinTech,
    Operations,
    HR,
    Auditing
}
