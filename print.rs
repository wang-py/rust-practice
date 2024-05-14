fn main() {
    println!("{} cookies", 3);
    println!("{0}, this is {1}, {1}, this is {0}", "Pog", "Poggers");

    println!("{subject} {verb} {object}", 
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    println!("Base 10:                      {}",   69420); // 69420
    println!("Base 2 (binary):              {:b}", 69420); // 69420
    println!("Base 8: (octal):              {:o}", 69420); // 69420
    println!("Base 16 (hexadecimal):        {:x}", 69420); // 69420

    // right justify
    println!("{number:>5}", number=1);
    // justify with zeros
    println!("{number:0>5}", number=1);
    // justify with zeros
    println!("{number:0<5}", number=1);
    // use arguments
    println!("{number:0>width$}", number=1, width=5);
    // checks number of arguments
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
    // printing pi
    /*
     If a format string in the fashion of {:<spec>.*} is used,
     then the first input holds the usize precision,
     and the second holds the value to print.
     If a format string in the fashion of {<arg>:<spec>.*} is used,
     then the <arg> part refers to the value to print,
     and the precision is taken like it was specified with
     an omitted positional parameter ({} instead of {<arg>:}).
     */
    // let pi = 3.1415926;
    println!("{:.*}", 3, 3.1415926);
}
