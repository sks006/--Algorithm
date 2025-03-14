mod counting_sort;
use crate::counting_sort::counting_sort;

fn get_max(arr: &[u32]) -> u32 {
    *arr.iter().max().unwrap()
}
fn radix_sort(arr:&mut Vec<u32>)
{
    let max= get_max(arr);
    let mut exp=1;
    while max/exp>0{
         counting_sort(arr, exp);
        exp *= 10;
    } 
}

fn main() {
   let mut arr=vec![170, 45, 75, 90, 802, 24, 2, 66];
   println!("Before search {:?}",arr);
      radix_sort(&mut arr);
    println!("After sorting: {:?}", arr);
}
