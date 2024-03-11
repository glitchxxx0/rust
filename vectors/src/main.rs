fn main() {
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third{
        Some(third)=>println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    for i in &v{
        println!("{i}");
    }

    let mut v = vec![100,50,200];
    for i in &mut v{
        *i+=100;
    }
    for i in &mut v{
        println!("{i}");
    }

    enum Cell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Cell::Int(3),
        Cell::Float(0.55),
        Cell::Text(String::from("yo")),
    ];

//    for i in &row{
//        println!("{i}");
//    }
}
