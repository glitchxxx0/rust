use std::io;
use std::collections::HashMap;

fn avg(numbers:&Vec<i32>)->f64{
    let sum:i32 = numbers.iter().sum();
    let length = numbers.len() as f64;
    sum as f64/length
}

fn median(numbers:&mut Vec<i32>)->f64{
    numbers.sort();
    let length = numbers.len();
    if length%2==0{
        let mid = length/2;
        (numbers[mid-1]+numbers[mid]) as f64/2.0
    }else{
        numbers[length/2] as f64
    }
}

fn mode(numbers: &Vec<i32>)->Option<i32>{
    let mut freq_map:HashMap<i32,usize> = HashMap::new();

    for num in numbers{
        *freq_map.entry(*num).or_insert(0)+=1;
    }
    let mut max_freq = 0;
    let mut ans = None;
    for(num, freq) in freq_map{
        if freq > max_freq{
            max_freq = freq;
            ans = Some(num);
        }
    }

    ans 

}

fn main() {
   let mut numbers:Vec<i32> = Vec::new();

   println!("How many integers would you like to enter");
   let mut input = String::new();
   io::stdin()
       .read_line(&mut input)
       .expect("failed to read input");
   
   let n:usize = match input.trim().parse(){
       Ok(num)=>num,
       Err(_)=>{
           println!("invalid input, please enter a positive integer");
           return;
       }
   };

   println!("enter {} integers:",n);

   for _ in 0..n{
       let mut input = String::new();

       io::stdin()
           .read_line(&mut input)
           .expect("failed to read input");

       let value:i32 = match input.trim().parse(){
           Ok(num)=>num,
           Err(_)=>{
               println!("invalid input, please enter an integer");
               continue;
           }
       };

       numbers.push(value);
   }
   println!("You entered {:?}", numbers);

   let average = avg(&numbers);
   println!("The average of numbers is: {}", average);
   let mut sorted_numbers = numbers.clone();
   let median = median(&mut sorted_numbers);
   println!("The median of numbers is {}", median);

   if let Some(mode) = mode(&numbers){
       println!("The mode of the numbres is {}", mode);
   }else{
       println!("There is no mode");
   }
}
