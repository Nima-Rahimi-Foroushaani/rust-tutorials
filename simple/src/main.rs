mod banner;
mod lib;
use lib::function;
fn main()
//@ requires true;
/*@ ensures true;
@*/
{
    //@ open true;
    let _x = 42;
    function();
    banner::print_banner();
}
