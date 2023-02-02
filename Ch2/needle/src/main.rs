fn main() {
    let needle = 0o204;
    let haystack = [1, 1, 2, 5];

    for item in &haystack {
        println!("{}", *item);
    }
}
