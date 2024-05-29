fn main() {
        let mut a = 5;
        let mut b = 10;
        swap(&mut a, &mut b);
        println!("Swapped numbers: a = {}, b = {}", a, b);
    }
    
    fn swap(x: &mut i32, y: &mut i32) {
        let temp = *x;
        *x = *y;
        *y = temp;
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn test_swap() {
            let mut a = 1;
            let mut b = 2;
            swap(&mut a, &mut b);
            assert_eq!((a, b), (2, 1));
    
            let mut a = -1;
            let mut b = -2;
            swap(&mut a, &mut b);
            assert_eq!((a, b), (-2, -1));
    
            let mut a = 0;
            let mut b = 0;
            swap(&mut a, &mut b);
            assert_eq!((a, b), (0, 0));
    
            let mut a = 100;
            let mut b = 200;
            swap(&mut a, &mut b);
            assert_eq!((a, b), (200, 100));
        }
    }