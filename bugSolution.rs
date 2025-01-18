fn main() {
    let vec = vec![1, 2, 3];

    // Using iter().cloned() to create a new iterator that clones elements.
    for element in vec.iter().cloned() {
        println!("Element: {}", element);
    }

    // Alternative using indexing, safe if you know the bounds
    for i in 0..vec.len(){
        println!("Element at index {} is: {}", i, vec[i]);
    }
} 