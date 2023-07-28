extern crate rand;

use rand::seq::SliceRandom;
use rand::Rng;

fn main(){
let mut randarray = rand::thread_rng();
let mut cr = ["@","%", "#", "!"];
cr.shuffle(&mut randarray);

let mut rnm = rand::thread_rng();
let mut rnn = rand::thread_rng();
let letra_maiuscula: char = rnm.gen_range(b'A'..=b'Z') as char;
let letra_minuscula: char = rnn.gen_range(b'a'..=b'z') as char;

println!("{:?}{}`{:?}{}", cr, letra_maiuscula, cr, letra_minuscula);





}