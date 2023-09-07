use rs_macros::macros;

fn main() {
    println!("{:?}",macros::vec_comprehension!(2*x, x in vec![1,2,3,4,6,7,7,5,45,6,1,645,4,2,5],));
}
