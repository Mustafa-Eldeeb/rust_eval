

/* func to get common values in available and prefered lists */
fn available_prefered(available:Vec<usize>,prefered:Vec<usize>)->Vec<usize>{
let mut res :Vec<usize>=vec![];
for p in prefered{
    if available.contains(&p){
        res.push(p)
    }
}
res
}



fn main() {
    println!("Hello, world!");
}
#[test]
fn test_available_prefered(){
   let prefered = vec![1,45,1240];
    let available=vec![3,54,45,12,1];
    let expected=vec![1,45];
    let res=available_prefered(available, prefered);
    println!("{:#?}",res);
    assert_eq!(expected,res);
}


