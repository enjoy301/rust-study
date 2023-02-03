fn main() {
    println!("Hello, World!");

    let korean = "안녕 월드";
    let japanese = "헤로우 워르드";

    let regions = [korean, japanese];

    for region in regions.iter() {
        println!("{}", region);
    }
}
