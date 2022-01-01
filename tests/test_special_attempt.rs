/* add test special attempt to test behaviour when list have 'any' */
#[cfg(test)]
mod tests{
    use rust_eval::attempt::*;
    use rust_eval::util::Any;
   

    //speacial value tests 
    #[test]
    fn test_special_attempt_1(){
        let any:usize=Any::Any.into_usize();
        let available=vec![240,360,720];
        let allowed=vec![360,any];
        let preferred = vec![360,720];
        let result=attempt(available, preferred, allowed);
        let expected:Vec<usize>= vec![360,720];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);

    }
    #[test]
    fn test_special_attempt_2(){
        let any:usize=Any::Any.into_usize();
        let available=vec![240,360,720];
        let allowed=vec![240,360,720];
        let preferred = vec![any,720];
        let result=attempt(available, preferred, allowed);
        let expected:Vec<usize>= vec![240,360,720];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);

    }
    

    #[test]
    fn test_special_attempt_3(){
        let any:usize=Any::Any.into_usize();
        let available=vec![240,360,720];
        let allowed=vec![360,1080];
        let preferred = vec![any,720];
        let result=attempt(available, preferred, allowed);
        let expected= vec![360];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);

    }

    #[test]
    fn test_special_attempt_4(){
        let any:usize=Any::Any.into_usize();
        let available=vec![240,360,720];
        let allowed=vec![1080];
        let preferred = vec![any,720];
        let result=attempt(available, preferred, allowed);
        let expected:Vec<usize>= vec![];
        //println!("result {:#?}",result);
        assert_eq!(expected,result);

    }
}


