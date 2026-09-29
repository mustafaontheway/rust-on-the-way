fn main() {

    let _birth_ayben = FamilyMemberBirths::Ayben {month: Months::Dec, year: Years::Y2024};
}

enum Months { Jan, Feb, Mar, Apr, May, June, July, Aug, Sep, Oct, Nove, Dec }

enum Years { Y2022 = 2022, Y2023 = 2023, Y2024 = 2024, Y2025 = 2025, Y2026 = 2026 }

enum FamilyMemberBirths {
    
    Mustafa {month: Months, year: Years},
    Aygun {month: Months, year: Years},
    Ayben {month: Months, year: Years},
    Aybuke {month: Months, year: Years},
    Aybilge {month: Months, year: Years},
}
