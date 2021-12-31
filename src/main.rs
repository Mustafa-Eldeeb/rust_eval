/* func to get common values in available and prefered lists */
fn available_prefered(available:Vec<usize>,preferred:Vec<usize>)->Vec<usize>{
let mut res :Vec<usize>=vec![];
for p in preferred{
    if available.contains(&p){
        res.push(p)
    }
}
res
}
pub fn attempt(available:Vec<usize>,preferred:Vec<usize>,allowed:Vec<usize>)->Vec<usize>{
    let mut res :Vec<usize>=vec![];
    let mut available=available;
    available.sort();
    for p in preferred.iter(){
        if available.contains(p){
            res.push(*p)
        }
        else{
            match closest_not_smaller(*p, &available) {
                Some(val)=>res.push(val),
                None=>{
                    match closest_smaller(*p, &available) {
                        Some(val)=>res.push(val),
                        None=>()
                    }
                }
            }
            
        }
    }

    allowed_result_list(res, allowed)   
}


//func to get the closest not smaller for prefered value

fn main() {

 
    
    let available=vec![720];
    let allowed=vec![240,360,1080];
    let preferred = vec![240,360];
  let result=attempt(available, preferred, allowed);
  println!("result {:#?}",result);
}

//func to get the available closest not smaller value to each value in prefered list
fn closest_not_smaller(p_num:usize,available:&Vec<usize>)->Option<usize>{
    for e in available.iter() { 
        if e>&p_num
        {
            return Some(*e);
        }
    }
None

}
//func to get the available closest smaller value to each value in prefered list
fn closest_smaller(p_num:usize,available:&Vec<usize>)->Option<usize>{

    for e in available.iter().rev() { 
        if e <&p_num 
        {
            return Some(*e);
        }
    }
None

}

// func that return a list of allowed values
fn allowed_result_list(result:Vec<usize>,allowed:Vec<usize>)->Vec<usize>{
    let mut confirmed_res=vec![];
    for res in result.iter(){
        if allowed.contains(res){
            confirmed_res.push(*res)
        }
    }
    confirmed_res.dedup();
    confirmed_res

}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_available_prefered(){
    let prefered = vec![1,45,1240];
        let available=vec![3,54,45,12,1];
        let expected=vec![1,45];
        let res=available_prefered(available, prefered);
       //println!("{:#?}",res);
        assert_eq!(expected,res);

    }
    #[test]
    fn test_closest_not_smaller() {
        let available=vec![3,4,5,12,45,60];
        let num=4;
        let expected=5;
        let res= closest_not_smaller(num, &available).unwrap();
        //println!("{:#?}",res);
        assert_eq!(expected,res);
    }
    #[test]
    fn test_closest_smaller() {
        let available=vec![3,4,5,12,45,60];
        let num=12;
        let expected=5;
        let res= closest_smaller(num, &available).unwrap();
        //println!("{:#?}",res);
        assert_eq!(expected,res);
    }

    #[test]
    fn test_allowed_result_list() {
        let result=vec![3,4,12,60];
        let allowed=vec![32,4,5,33,45,60];
        let expected=vec![4,60];
        let res= allowed_result_list(result, allowed);
        //println!("{:#?}",res);
        assert_eq!(expected,res);
    }

    #[test]
    fn test_attempt_1(){
        let available=vec![240,360,720];
        let allowed=vec![360,720];
        let preferred = vec![1080];
      let result = attempt(available, preferred, allowed);
      let expected= vec![720];
      assert_eq!(expected,result);
      //println!("result {:#?}",result);
    } 
    #[test]
    fn test_attempt_2(){
        let available=vec![240,720];
        let allowed=vec![360,720];
        let preferred = vec![1080];
        let result=attempt(available, preferred, allowed);
        let expected= vec![720];
        assert_eq!(expected,result);
      //println!("result {:#?}",result);
    }
    #[test]
    fn test_attempt_3(){
        let available=vec![240];
        let allowed=vec![360,720];
        let preferred = vec![1080];
      let result=attempt(available, preferred, allowed);
      let expected:Vec<usize>= vec![];
      assert_eq!(expected,result);
      //println!("result {:#?}",result);
    }
    #[test]
    fn test_attempt_4(){
        let available=vec![240,360,720];
        let allowed=vec![240,360,720,1080];
        let preferred = vec![240,360];
        let result=attempt(available, preferred, allowed);
        let expected= vec![240,360];
        assert_eq!(expected,result);
      //println!("result {:#?}",result);
    }
    
    
    #[test]
    fn test_attempt_5(){
        let available=vec![240,720];
        let allowed=vec![240,360,720,1080];
        let preferred = vec![240,360];
        let result=attempt(available, preferred, allowed);
        let expected= vec![240,720];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);
    }
    #[test]
    fn test_attempt_6(){
        let available=vec![240,720];
        let allowed=vec![240,360,1080];
        let preferred = vec![240,360];
        let result=attempt(available, preferred, allowed);
        let expected= vec![240];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);
    }

    #[test]
    fn test_attempt_7(){
        let available=vec![720];
        let allowed=vec![240,360,1080];
        let preferred = vec![240,360];
        let result=attempt(available, preferred, allowed);
        let expected:Vec<usize>= vec![];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);
    }    
    
    #[test]
    fn test_attempt_8(){
        let available=vec![240,360];
        let allowed=vec![240,360];
        let preferred = vec![720,1080];
        let result=attempt(available, preferred, allowed);
        let expected= vec![360];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);

    }
    
   
}
