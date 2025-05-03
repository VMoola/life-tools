/*
 * Supresses a bunch of expected warnings (since this is an learning file
 * the code is not nice)
 */
#![allow(warnings)]

/*
 * Define a macro, This ends up being used later
 */
#[macro_export]
macro_rules! wwe{
    ($value:expr) => { $value * 2}
}

/*
 * Main function, this runs to completion
 */
fn main() {
    let _x = test(); //runs the bulk of our example
    let g = 7;
    let test;

    //This next portion is a lifetime notifier example
    let z = 5;
    {
        //let z = 5;

        test = clo(&g, &z); // test = (x, x)
    }

    println!("{:?}", test); // test = (x, x)
}

/*
 * Defines 2 lifetimes, and pairs them to the references
 * we were passed. The way it stands, if 'tt is used for the
 * return, our code won't compile
 */
fn clo<'t, 'tt>(x: &'t i32, y: &'tt i32) -> (&'t i32, &'t i32) {
    return (x, x);
    //return (y, y); -> (&'tt, &'tt) is appropriate, but won't compile
    //without expanding z to where our result is used.
}

fn test() -> () {
    /*
     * The first chunk of this is just showcasing scope and shadowing
     */
    let mut x : String = String::from("haha Naval is overthinking");
    let statement: String;
    let g = &mut x;
    let no : bool = false;
    {
        statement = "dude what did I do".to_string();
        println!("{g}");
        *g = String::from("Nesting mut");
        let x = &g;
        /*
         * This showcases if else assignment to a variable
         */
        let statement = if no {
            x.to_string();
            12
        } else {
            x.to_string();
            100
        };
        println!("{g}");
        //g is a mutable reference to x, so we're changing x here
        *g = String::from("This guy");
        println!("{statement}");
    }

    let result;
    println!("{x}");

    {
        let y: String = String::from("don't get confused");
        /*
         * See the example in main(), this works because we told
         * the compiler we won't use y's limited scope in fn clown()
         */
        result = clown(x.as_str(), y.as_str());
    }
    println!("{result}");

    let a = 1;
    /*
     * Create a Naval Object using constructor nava()
     * Then copy a mutatable Naval object from n
     */
    let n = Naval::nava(1);
    let mut q = Naval {
        ..n
    };
    /*
     * Call get_big using the standard, then 'pure' approach
     */
    q.get_big(Thinking::Work(10));
    println!("{:?}", q);
    Naval::get_big(&mut q, Thinking::Uncertain);
    let q = Point {x: 5, y: 6.0};
    q.tes();
    println!("{:?}", q);

    let p : *const i32;
    {
        let m : i32 = 42;
        p = &m;
    }
    //this is risky b/c m will go out of scope
    //it may potentially be freed by the time of read
    unsafe {
    println!("{:?}", *p);
    };
    
}

/*
 * Tell rust to generate the implementation so we can print this struct
 * with {:?}
 */
#[derive(Debug)]
struct Point <X1, Y1>{
    x: X1,
    y: Y1,
}

/*
 * Implement some functionality for struct point
 */
impl<X1, Y1> Point<X1, Y1> {

    fn tes(self: &Self)
    {
        println!("wwww");
    }
}

/*
 * Enum example, self-explanatory
 */
enum Thinking {
    OfCode(bool),
    None,
    Work(i64),
    Uncertain,
}

/*
 * Implement a trait for struct Naval<generic_type>
 */
impl<U> Naval<U>{
    fn get_big(self: &mut Self, t : Thinking) -> () {
        let mut new_value : i64 = 0;
        match t {
            Thinking::OfCode(woah) => {
                    //if Naval is thinking of nips, get big
                    new_value = self.value;
                    //if Naval is well rested, get bigger
                    if woah {
                        new_value = wwe!(new_value);
                    }

                }
            Thinking::None => {
                    //navals doesnt exist?!
                    new_value = 0;
            }
            Thinking::Work(time) => {
                    //shrink according to time slaving away
                    new_value = -1 * wwe!(time);
            }
            _ => {
                println!("nothing to match");
            }
        }
        (*self).value = new_value; //implied dereferencing
    }
    fn nava(ty : U) -> Naval<U>{
        Naval {
            value: 5,
            age: ty,
            size : 1,
        }
    }
}

#[derive(Debug)]
struct Naval<T>{
    value : i64,
    age : T,
    size : u64,
}

fn clown<'a>(_x: &'a str, _y: &str) -> &'a str {
    "Stop overthinking"
}
