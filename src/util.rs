
pub enum Any{
     Any
}

impl Any {
    pub fn into_usize(self)->usize{   
        match self{
           Self::Any =>00000   
        }
    }
}

//func to get the available closest not smaller value to each value in prefered list
pub fn closest_not_smaller(p_num:usize,available:&Vec<usize>)->Option<usize>{
    for e in available.iter() { 
        if e>&p_num
        {
            return Some(*e);
        }
    }
None

}
//func to get the available closest smaller value to each value in prefered list
pub fn closest_smaller(p_num:usize,available:&Vec<usize>)->Option<usize>{

    for e in available.iter().rev() { 
        if e <&p_num 
        {
            return Some(*e);
        }
    }
None

}

// func that return a list of allowed values
pub fn allowed_result_list(result:Vec<usize>,allowed:Vec<usize>)->Vec<usize>{
    let mut confirmed_res=vec![];
    let any:usize=Any::Any.into_usize();
    if allowed.contains(&any){
         confirmed_res=result;
         return confirmed_res;
    }
    for res in result.iter(){
        if allowed.contains(res){
            confirmed_res.push(*res)
        }
    }
    confirmed_res.dedup();
    confirmed_res

}

    
/* func to get common values in available and prefered lists */
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
pub fn available_prefered(available:Vec<usize>,preferred:Vec<usize>)->Vec<usize>{
    let mut res :Vec<usize>=vec![];
    for p in preferred{
        if available.contains(&p){
            res.push(p)
        }
    }
    res
    }
 */
