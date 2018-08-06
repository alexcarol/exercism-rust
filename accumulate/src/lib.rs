/// What should the type of _function be?
pub fn map_function<F>(input: Vec<i32>, _function: F) -> Vec<i32>
    where F: Fn(i32) -> i32
{
    let mut output = Vec::new();

    for i in 0..input.len() {
        output.push(_function(input[i]));
    }

    output
}

/// What should the type of _closure be?
pub fn map_closure<F>(input: Vec<i32>, _closure: F) -> Vec<i32>
    where F: Fn(i32) -> i32 {
    map_function(input, _closure)
}
