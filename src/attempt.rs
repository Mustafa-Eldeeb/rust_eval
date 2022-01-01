
use crate::util::*;


pub fn attempt(available:Vec<usize>,preferred:Vec<usize>,allowed:Vec<usize>)->Vec<usize>{
    let mut res :Vec<usize>=vec![];
    let mut available=available;
    let any:usize=Any::Any.into_usize();
    available.sort();
    if preferred.contains(&any){
        return allowed_result_list(available, allowed)
    }
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

   let mut output= allowed_result_list(res, allowed)   ;
   output.dedup();
   output
}





/* 
fn main() {
    let _any =Any::Any.into_usize();
    let available=vec![720];
    let allowed=vec![_any,240,360,1080];
    let preferred = vec![240,360];
    let result=attempt(available, preferred, allowed);
    
    println!("result {:#?}",result);
}

*/