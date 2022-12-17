 fn main()
{
    my_numbers();
    my_vectors();
    my_enumerate();
}

fn my_numbers()
{
    let x = 34..43;
    for i in x{
        println!("{}",i);
    }
}

fn my_vectors()
{
    let x = vec!["Hello","World","Name","Avatar"];
    for i in x.iter(){
        print!("{} ",i);
    }
    println!();
}


fn my_enumerate()
{
    let x = vec!["Hello","World","Name","Avatar"];
    for (index,a) in x.iter().enumerate(){
        println!("{}  -- {}",index,a);
    }
}