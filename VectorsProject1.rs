fn main() {
    // Define staff levels and corresponding roles
    let staff_levels = vec![
        ("APS 1-2", vec!["Intern", "Paralegal", "Placement"]),
        ("APS 3-5", vec!["Administrator", "Junior Associate", "Classroom Teacher"]),
        ("APS 5-8", vec![
            "Senior Administrator",
            "Associate",
            "Snr Teacher",
        ]),
        ("EL1 8-10", vec!["Office Manager", "Senior Associate 1-2", "Leading Teacher"]),
        ("EL2 10-13", vec!["Director", "Senior Associate 3-4", "Deputy Principal"]),
        ("SES", vec!["CEO", "Partner", "Principal"]),
    ];

    // Prompt user for role and experience
    let mut role_input = String::new();
    let mut experience_input = String::new();

    println!("Enter the staff role (e.g., Associate, Classroom Teacher, etc.):");
    std::io::stdin().read_line(&mut role_input).expect("Failed to read input");
    let role = role_input.trim();

    println!("Enter the years of experience:");
    std::io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience: i32 = experience_input
        .trim()
        .parse()
        .expect("Please enter a valid number for experience");

    // Validate the staff level
    let mut found = false;
    for (level, roles) in staff_levels.iter() {
        if roles.contains(&role) {
            let years_range: Vec<&str> = level.split_whitespace().last().unwrap().split('-').collect();
            let start_years: i32 = years_range[0].parse().unwrap();
            let end_years: i32 = years_range[1].parse().unwrap();

            if experience >= start_years && experience <= end_years {
                println!("The staff with role '{}' and {} years of experience holds position: {}", role, experience, level);
                found = true;
                break;
            }
        }
    }

    if !found {
        println!("No matching position found for role '{}' with {} years of experience.", role, experience);
    }
}