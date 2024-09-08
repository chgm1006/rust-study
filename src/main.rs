fn main() {
    let nums: Vec<i32> = vec![1, 2, 3];

    let f = |x: &i32| x + 1;

    let maps: Vec<i32> = nums.iter().map(f).collect();
    println!("{:?}", maps);

    let filters: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 1).collect();
    println!("{:?}", filters);
}
