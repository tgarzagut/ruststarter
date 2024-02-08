//Question 1
// fn concat_strings(s1: &String, s2: &String) -> String {
//     let concat = format!("{}{}", s1, s2);
//     return concat;
     
// }
    
// fn main() {
//     let s1 = String::from("Hello, ");
//     let s2 = String::from("World!");
//     let result = concat_strings(&s1, &s2);
//     println!("{}", result); // Should print: "Hello, World!"
// }

//Question 2
// fn clone_and_modify(s: &String) -> String {
//     let sclone = s.clone();
//     let newstring = format!("{}you.", sclone);
//     return newstring;
// }

// fn main() {
//     let s = String::from("Hello, ");
//     let modified = clone_and_modify(&s);
//     println!("Original: {}", s); // Should print: "Original: Hello, "
//     println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
// }

//Question 3
// #[allow(unused_variables, unused_mut)]
// fn sum(total: &mut i32, low: i32, high: i32) {
//     for i in low..=high {
//         *total +=i;
//     }
// }

// fn main(){
//     let mut total = 0;
//     let low:i32 = 0;
//     let high:i32 = 100;
//     sum(&mut total, low, high);
//     println!("Total: {}", total);
// }
    
