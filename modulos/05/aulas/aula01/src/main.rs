fn swap_array(list: &mut [i32; 7], i: usize, j: usize) {
  let temp: i32 = list[i];
  list[i] = list[j];
  list[j] = temp;
}

fn main() {
  let mut array: [i32; 7] = [10, 23, 4, 5, 66, 7, -3];
  println!("{:?}", array);
  for i in 0..array.len() {
    for j in ((i + 1)..array.len()).rev() {
      if array[j - 1] > array[j] {
        swap_array(&mut array, j - 1, j);
      }

      println!("{:?}", array);
    }
  }
}
