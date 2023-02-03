fn main() {
    let penguin_data = "\
    common name, length(cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i==0 || record.trim().len() == 0 {
            continue;
        }

        // Vec<_>은 Vector긴 한데 타입은 너가 유추해 라는 뜻
        let fields: Vec<_> = record.split(',')
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        // fields의 1번째 인덱스를 float 32bit로 parsing
        // parse함수 리턴값은 OK(value) or Err(E)
        // if let 으로 length에 파싱한 값 넣기
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
