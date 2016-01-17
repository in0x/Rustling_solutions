pub fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {

    let mut _vec = Vec::new();

    for x in vec.iter() {
        _vec.push(*x);
    }

    _vec
}
