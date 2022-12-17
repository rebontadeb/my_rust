
fn main()
{
    myloop_break()
}

fn  myloop_break ()
{
    let mut b=45;

    loop{
        b=b+5;
        println!("{}",b);
        if b > 100{
            break;
        }

    }
}
