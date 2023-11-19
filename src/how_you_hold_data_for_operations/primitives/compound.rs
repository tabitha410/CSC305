pub fn run() {
    // Creating a tuple with two elements of different types
    let person: (String, i32) = ("Alice".to_string(), 30);

    // Destructuring the tuple with mutable variables
    let (mut name, mut age) = person;

    // Printing the destructured values
    println!("Name: {}", name);
    println!("Age: {}", age);

    // Modifying the values
    name.push_str(" Smith");
    age += 1;

    // Printing the modified values
    println!("Modified Name: {}", name);
    println!("Modified Age: {}", age);

    // Array Example
    // Creating an array of integers with a fixed size of 3
    let numbers: [i32; 3] = [1, 2, 3];

    // Accessing elements of the array
    let first_number = numbers[0];
    let second_number = numbers[1];
    let third_number = numbers[2];
    
    // Printing the array elements
    println!("First number: {}", first_number);
    println!("Second number: {}", second_number);
    println!("Third number: {}", third_number);
    
    // Iterating over the array
    println!("Iterating over the array:");
    for number in &numbers {
        println!("{}", number);
    }
    
    // Creating an array with a repeated value
    let repeated_numbers = [5; 4]; // This creates an array with four elements, all set to 5
    
    // Printing the array elements
    println!("Repeating number: {}", repeated_numbers[0]);
    println!("Repeating number: {}", repeated_numbers[1]);
    println!("Repeating number: {}", repeated_numbers[2]);
    println!("Repeating number: {}", repeated_numbers[3]);


    //Slice types
    // Creating an array
    let numbers = [1, 2, 3, 4, 5];

    // Creating a slice that includes elements from index 1 to 3 (exclusive)
    let slice = &numbers[1..4];

    // Printing the original array
    println!("Original array: {:?}", numbers);

    // Printing the slice
    println!("Slice: {:?}", slice);

    // Iterating over the slice
    println!("Iterating over the slice:");
    for &number in slice {
        println!("{}", number);
    }

    // Creating a slice of the whole array
    let whole_slice = &numbers[..];

    // Printing the whole slice
    println!("Whole slice: {:?}", whole_slice);
}
