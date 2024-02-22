use dotenv::dotenv;
 #[test]
fn works(){
    assert!(false)
}
#[test]
fn check_env(){
   dotenv().ok();
    let _abc = std::env::var("DATABASE").expect("u");
    println!("{:?}", _abc)
 }