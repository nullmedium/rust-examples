use std::os;

fn fib(x: int) -> int
{
   	match x {
        1 => {
        	println!("1");
         	return 1
        },
        2 => {
        	println!("1");
        	return 1
        },
		_ => {
			let x = fib(x-2) + fib(x-1);
        	println!("{}", x);
        	return x;
		}
    }
}

fn main()
{
    let arg : ~[~str] = os::args();

    if arg.len() > 1 {
	    match from_str(arg[1])
    	{
        	Some(x) => println!("{}", fib(x)),
         	None    => println!("I need a real number")
    	}
    }
    else
    {
    	println!("usage: fib number")
    }

}
