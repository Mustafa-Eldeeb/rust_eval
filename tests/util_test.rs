

#[cfg(test)]
mod tests{
    use rust_eval::util::*;

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
        let any:usize=Any::Any.into_usize();
        let result=vec![3,4,12,60];
        let allowed=vec![32,4,5,33,45,60];
        let allowed_with_any=vec![32,4,5,33,45,60,any];
        let expected_with_any=vec![3,4,12,60];
        let _expected=vec![4,60];

        let _res= allowed_result_list(result.to_owned(), allowed);
        //println!("{:#?}",_res);
        //assert_eq!(_expected,_res);
        let res_with_any= allowed_result_list(result, allowed_with_any);
        //println!("{:#?}",res_with_any);

        assert_eq!(expected_with_any,res_with_any);
    }

}