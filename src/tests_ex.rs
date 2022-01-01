#[cfg(test)]
mod tests{
    use super::*;

   

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
    //speacial value tests ===>
    #[test]
    fn test_special_attempt_1(){
        let any:usize=Any::Any.into_usize();
        let available=vec![240,360,720];
        let allowed=vec![360,any];
        let preferred = vec![360,720];
        let result=attempt(available, preferred, allowed);
        let expected:Vec<usize>= vec![360,720];
        println!("result {:#?}",result);
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
        println!("result {:#?}",result);
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
        println!("result {:#?}",result);
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
        println!("result {:#?}",result);
        assert_eq!(expected,result);

    }
}


