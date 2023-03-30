
fn fun_test_impl(value: i32, f: impl Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn fun_test_dyn(value: i32, f: &dyn Fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn fun_test_ptr(value: i32, f: fn(i32) -> i32) -> i32 {
    println!("{}", f(value));
    value
}

fn times2(value: i32) -> i32 {
    2 * value
}

fn make_adder(a: i32) -> Box<dyn Fn(i32) -> i32> {
	Box::new(move |b| a*b) 
}

fn takes_fn_returning_closure(value i32, f: fn(i32) -> i32) -> i32 { 
	f(value)
}

fn main() {
    let y = 2;
    //static dispatch
    fun_test_impl(5, times2);
    fun_test_impl(5, |x| 2*x);
    fun_test_impl(5, |x| y*x);

    //dynamic dispatch
    fun_test_dyn(5, &times2);
    fun_test_dyn(5, &|x| 2*x);
    fun_test_dyn(5, &|x| y*x);

    //C-like pointer to function
    fun_test_ptr(5, times2);
    fun_test_ptr(5, |x| 2*x); //ok: empty capture set

    //fun_test_ptr(5, |x| y*x); //error: expected fn pointer, found closure

    println!("adder {}", make_adder(5)(y));
	
}
