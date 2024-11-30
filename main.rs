use std::io;

fn main() {
    println!("Select a calculation:");
    println!("1: Area of Trapezium");
    println!("2: Area of Rhombus");
    println!("3: Area of Parallelogram");
    println!("4: Area of Cube");
    println!("5: Volume of Cylinder");

    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read input");
    let choice: u32 = input1.trim().parse().expect("Invalid choice. Please enter a number.");

    if choice == 1 {
        let mut base1 = String::new();
        println!("Enter the first base of the trapezium:");
        io::stdin()
            .read_line(&mut base1)
            .expect("Failed to read input");
        let base1: f64 = base1.trim().parse().expect("Invalid input");

        let mut base2 = String::new();
        println!("Enter the second base of the trapezium:");
        io::stdin()
            .read_line(&mut base2)
            .expect("Failed to read input");
        let base2: f64 = base2.trim().parse().expect("Invalid input");

        let mut height = String::new();
        println!("Enter the height of the trapezium:");
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read input");
        let height: f64 = height.trim().parse().expect("Invalid input");

        let area = ((base1 + base2) / 2.0) * height;
        println!("The area of the trapezium is: {:.2}", area);
    } else if choice == 2 {
        let mut diag1 = String::new();
        println!("Enter the first diagonal of the rhombus:");
        io::stdin()
            .read_line(&mut diag1)
            .expect("Failed to read input");
        let diag1: f64 = diag1.trim().parse().expect("Invalid input");

        let mut diag2 = String::new();
        println!("Enter the second diagonal of the rhombus:");
        io::stdin()
            .read_line(&mut diag2)
            .expect("Failed to read input");
        let diag2: f64 = diag2.trim().parse().expect("Invalid input");

        let area = (diag1 * diag2) / 2.0;
        println!("The area of the rhombus is: {:.2}", area);
    } else if choice == 3 {
        let mut base = String::new();
        println!("Enter the base of the parallelogram:");
        io::stdin()
            .read_line(&mut base)
            .expect("Failed to read input");
        let base: f64 = base.trim().parse().expect("Invalid input");

        let mut height = String::new();
        println!("Enter the height of the parallelogram:");
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read input");
        let height: f64 = height.trim().parse().expect("Invalid input");

        let area = base * height;
        println!("The area of the parallelogram is: {:.2}", area);
    } else if choice == 4 {
        let mut side = String::new();
        println!("Enter the side length of the cube:");
        io::stdin()
            .read_line(&mut side)
            .expect("Failed to read input");
        let side: f64 = side.trim().parse().expect("Invalid input");

        let area = 6.0 * (side * side);
        println!("The surface area of the cube is: {:.2}", area);
    } else if choice == 5 {
        let mut radius = String::new();
        println!("Enter the radius of the cylinder:");
        io::stdin()
            .read_line(&mut radius)
            .expect("Failed to read input");
        let radius: f64 = radius.trim().parse().expect("Invalid input");

        let mut height = String::new();
        println!("Enter the height of the cylinder:");
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read input");
        let height: f64 = height.trim().parse().expect("Invalid input");

        let volume = std::f64::consts::PI * radius.powi(2) * height;
        println!("The volume of the cylinder is: {:.2}", volume);
    } else {
        println!("Invalid choice. Please restart and select a valid option.");
    }
}
