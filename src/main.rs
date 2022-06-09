mod mpsc;
mod seq;

fn main() {
    seq::sequential_execution();
    println!("");
    mpsc::parallel_execution();
}
