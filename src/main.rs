mod util;
use util::*;

fn main() {
    let _any =Any::Any.into_usize();
    let available=vec![720];
    let allowed=vec![240,360,1080];
    let preferred = vec![240,360];
    let result=attempt(available, preferred, allowed);
    println!("result {:#?}",result);
}


