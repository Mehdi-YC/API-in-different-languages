fn is_palindrome(test : i32) -> bool{
    let mut reversed = 0;
    let mut next = test;
    
    while next  !=0 {
        reversed = reversed * 10 + next % 10;
        next= next / 10;
    }
   reversed == test
}

fn main(){
println!("{}",is_palindrome(125));
let mut x = vec![1,3,5,2];
for u in &x{
    println!("{}",u);

}
println!("my vec {:?}",x);
println!("double it and give it to the next process {:?}",x.iter().map(|it| it * 2).collect::<Vec<u32>>());
x.sort();
println!("sort {:?}",x);
println!("the summ {}",x.iter().sum::<u32>());
println!("min {:?}",x.iter().min().unwrap());
println!("max {:?}",x.iter().max().unwrap());

println!("crazy try {}",x.iter().map(|it| it * 2).filter(|it| it >= &6).fold(1,|acc,it| it*acc));

}
