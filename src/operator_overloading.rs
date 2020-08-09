use std::ops::{Add, AddAssign, Neg};
use std::cmp::PartialEq;
#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T> 
    where T: Add<Output = T> 
{
    type Output = Complex<T>;

    //fn a+b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> Neg for Complex<T> 
    where T: Neg<Output = T> 
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> PartialEq for Complex<T>
    where  T: PartialEq 
{
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

impl<T: Eq> Eq for Complex<T> where T: Eq {}

pub fn main_operator_o() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);

    println!("a : {:?} ", a);
    println!("b : {:?} ", b);
    // a += b;
    // println!(" a = {:?}", -a);
    // println!("a + b = {:?}", a + b);
    println!("a == b is {}", a == b);

}