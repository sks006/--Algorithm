
fn merge_sort(arr:&mut[i32] ,left:usize ,right:usize ){

    if left<right{
let mid=left+(right-left)/2;
merge_sort(arr, left, mid);
merge_sort(arr, mid+1, right);
merge(arr,left,mid,right);
}}
fn merge(arr:&mut[i32],left:usize,mid:usize,right:usize){
let mut temp=arr.to_vec();
let (mut left_index,mut right_index,mut merged_index)=(left,mid+1,left);

while left_index<=mid && right_index<=right {

    if arr[left_index]<=arr[right_index]{
        temp[merged_index]=arr[left_index];
          left_index += 1;
    }else {
         temp[merged_index] = arr[right_index];
            right_index += 1;
    }

    merged_index+=1
}
  while left_index <= mid {
        temp[merged_index] = arr[left_index];
        left_index += 1;
        merged_index += 1;
    }
 arr[left..=right].copy_from_slice(&temp[left..=right]);


}

fn main() {
    let mut arr = [1, 4, 6, 8, 3, 1, 34, 45, 76, 2, 46, 8, 2, 1, 5, 6];
    let n = arr.len();
    merge_sort(&mut arr, 0, n - 1);
    println!("{:?}", arr);
}