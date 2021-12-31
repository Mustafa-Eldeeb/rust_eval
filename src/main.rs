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

//func to get the closest not smaller for prefered value

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
#[test]
fn test_closest_not_smaller() {
    let available=vec![3,4,5,12,45,60];
    let num=4;
    let expected=5;
    let res= closest_not_smaller(num, available);
    println!("{:#?}",res);
    assert_eq!(expected,res);
}
#[test]
fn test_closest_smaller() {
    let available=vec![3,4,5,12,45,60];
    let num=12;
    let expected=5;
    let res= closest_smaller(num, available);
    println!("{:#?}",res);
    assert_eq!(expected,res);
}

//func to get the available closest not smaller value to each value in prefered list
fn closest_not_smaller(p_num:usize,available:Vec<usize>)->usize{
    for e in available.iter() { 
        if e>&p_num
        {
            return *e;
        }
    }
0

}
//func to get the available closest smaller value to each value in prefered list
fn closest_smaller(p_num:usize,available:Vec<usize>)->usize{

    for e in available.iter().rev() { 
        if e <&p_num 
        {
            return *e;
        }
    }
0

}

