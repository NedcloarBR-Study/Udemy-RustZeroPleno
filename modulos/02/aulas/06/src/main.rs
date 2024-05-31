static mut STATIC_VAR: i32 = 15;

fn main() {
  unsafe {
    println!("O valor de STATIC_VAR é: {}", STATIC_VAR);
  }
  // println!("O valor de STATIC_VAR é: {}", STATIC_VAR);
}
