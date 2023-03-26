use std::io;

fn main(){
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let mut t = String::new();
    f(&s,&mut t);
    print!("{}",t);
}

fn f(s:&str,t:&mut String){
    if s.len()<=10 {
        t.push_str(s);
    }
    else{
        t.push_str(&s[..10]);
        t.push_str("\n");
        f(&s[10..],t);
    }
}