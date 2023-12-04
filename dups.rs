fn main() {
    let male_group = vec![
        Person {
            name: Some("John"),
            age: Some(28),
        },
        Person {
            name: Some("Bob"),
            age: Some(35),
        },
        Person {
            name: Some("Charlie"),
            age: Some(22),
        },
        Person {
            name: Some("David"),
            age: Some(30),
        },
    ];

    let female_group = vec![
        Person {
            name: Some("Alice"),
            age: Some(25),
        },
        Person {
            name: Some("Eve"),
            age: Some(28),
        },
        Person {
            name: Some("Grace"),
            age: Some(32),
        },
        Person {
            name: Some("Helen"),
            age: Some(29),
        },
    ];

    // Combine, Transform, and Take
    let result: Vec<(String, u32)> = male_group
        .iter()
        .take(2)
        .chain(female_group.iter().take(2))
        .map(|person| {
            let name = person.name.unwrap_or("Lorem");
            let age = person.age.unwrap_or(99);
            (name.to_string(), age)
        })
        .take(5)
        .collect();

    println!("{:?}", result);

    // PART 2

    let _words = vec!["apple", "banana", "cherry", "date", "elderberry", "fig"];

    // Example 5: Enumerate and Collect into a HashMap
    // let word_map: std::collections::HashMap<usize, &str> = _words.iter()
    //     .enumerate()
    //     .collect();
    // println!("{:?}", word_map);
    //
    // Example 6: Flatten and Collect
    let matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flat_matrix = matrix.iter().flat_map(|row| row.iter());
    println!("{:?}", flat_matrix);

    // Example 7: Step By and Sum
    let evens: i32 = (0..10).step_by(2).sum();
    println!("Sum of evens: {}", evens);

    // Example 8: Unzip and Zip
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    let (numbers, letters): (Vec<_>, Vec<_>) = pairs.iter().cloned().unzip();
    let zipped_pairs: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    println!("{:?}", numbers);
    println!("{:?}", letters);
    println!("{:?}", zipped_pairs);

    // part 3

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Example 1: Mapping and Filtering
    let result1: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|x| *x % 3 == 0)
        .collect();
    println!("aymen said 6 12 and 18{:?}", result1); // Output: [6, 12, 18]

    // Example 2: Sum and Find
    let sum: i32 = numbers.iter().sum();
    let found: Option<&i32> = numbers.iter().find(|&&x| x == 7);
    println!("Sum: {}, Found: {:?}", sum, found); // Output: Sum: 55, Found: Some(7)

    // Example 3: Chaining and Reversing
    let vec2: Vec<i32> = vec![20, 30];
    let result2: Vec<i32> = numbers
        .clone()
        .into_iter()
        .skip(2)
        .take(4)
        .chain(vec2.into_iter())
        .rev()
        .collect();
    println!(" is that it .{:?}", result2); // Output: [40, 30, 9, 8, 7]

    // Example 4: Combining all, partition, and max
    let (even, odd): (Vec<i32>, Vec<i32>) =
        numbers.iter().map(|x| x * 3).partition(|x| *x % 2 == 0);
    let max_value = numbers.iter().max();
    println!("Even: {:?}, Odd: {:?}, Max: {:?}", even, odd, max_value);
}

#[derive(Clone, Debug)]
struct Person {
    name: Option<&'static str>,
    age: Option<u32>,
}
