pub fn diff(arr: &[i32]) -> Vec<i32> {
    arr.windows(2).map(|win| win[1] - win[0]).collect()
}

pub fn abs(arr: &[i32]) -> Vec<i32> {
    arr.iter().map(|val| val.abs()).collect()
}
