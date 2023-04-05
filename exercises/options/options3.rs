// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

struct Point {
    x: i32,
    y: i32,
}

fn main(){
    let y: Option<&Point> = Some(&Point { x: 100, y: 200 });
    match y {
        // 这相当是Some(value)然后把&value绑定到了q上
        Some(p) => {
            println!("Co-ordinates are {},{} ", (*p).x, (*p).y)
        }
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
