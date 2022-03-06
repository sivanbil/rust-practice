fn main() {
    let outer = String::from("Hello, world!");
    println!("outer: {}", outer);

    let longer = &outer;

    {
        let inner = String::from("Hello, world!!!!");

        let shorter = &inner;
        println!("shorter: {}", shorter);
        assert_eq!(select(shorter, longer, false), shorter);
        assert_eq!(select(shorter, longer, false), longer);
    }

    println!("longer: {}", longer);



}

fn select<'short, 'long>(s1:&'short str, s2:&'long str, second: bool) -> &'long str 
where 'short: 'long {
    if second {s2} else {s1}
}
