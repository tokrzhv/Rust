unsafe trait RM {}
unsafe impl RM for i32 {}


extern "C" {
    fn abs(inout: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello world!";
//have fixed address in memory
//CONST -> allowed duplicated their data whenever they're used
static mut NUM: u32 = 5; //unsafe

fn add_to_num(inc: u32) {
    unsafe {
        NUM += inc
    }
}

fn main() {
    add_to_num(6);
    unsafe {
        println!("NUM: {}", NUM);
    }


    println!("name is {}", HELLO_WORLD);


    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }


    let mut num = 10;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    unsafe fn dangerous() {
        println!("Unsafe");
    }
    unsafe { dangerous(); }






}
