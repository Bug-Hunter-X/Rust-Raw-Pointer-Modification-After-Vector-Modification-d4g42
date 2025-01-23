fn main() {
    let mut v = vec![1, 2, 3];
    let first_element = v.get_mut(0);

    if let Some(element) = first_element {
        *element = 10;
    }

    println!("The first element is: {}", v[0]);
}