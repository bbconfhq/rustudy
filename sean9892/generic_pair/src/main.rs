struct Pair<T> {
  first  : T,
  second : T
}

/*
impl Pair<i32> {
  fn l2_square(&self) -> i32 {
    &self.first*&self.first+&self.second*&self.second
  }
}

impl Pair<f32> {
  fn l2_square(&self) -> f32 {
    &self.first*&self.first+&self.second*&self.second
  }
}
*/

impl<T> Pair<T> where T: std::ops::Mul<T, Output = T>+std::ops::Add<T, Output = T>+Copy{
    fn l2_square(&self) -> T{
        self.first*self.first+self.second*self.second
    }
}

fn main(){
  let p = Pair { first: 3, second: 4 };
  println!("p = ({},{})", p.first, p.second);
  println!("L2Square = {}", p.l2_square());

  let p = Pair { first: 6.9, second: 7.4 };
  println!("p = ({},{})", p.first, p.second);
  println!("L2Square = {}", p.l2_square());
  
  let p = Pair { first: "KKD", second: "KHS" };
  println!("p = ({},{})", p.first, p.second);
}