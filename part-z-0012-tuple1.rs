fn main() {

    let info_emp_ayhan: (&'static str, String, u16) = ("Ayhan Bilir", "Finance Department".to_string(), 4300);

    println!("Employee name: {}", info_emp_ayhan.0);
    println!("Employee department: {}", info_emp_ayhan.1);
    println!("Employee salary: ${}", info_emp_ayhan.2);

    let (_name, department, _salary_usd) = &info_emp_ayhan;

    println!("Employee department: {}", department);
}

// Employee name: Ayhan Bilir
// Employee department: Finance Department
// Employee salary: $4300
// Employee department: Finance Department
