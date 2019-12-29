// // //This codes illustrate the difference between making an instance in main function directly 
// // // and making it via associated fucntion.

// // mod alpha {
// //     #[derive(Debug)]
// //     pub struct Student {
// //          name: String,
// //     }
    
// //     impl Student {
// //        pub fn new(name: String) -> Student {
// //             Student {
// //                 name,
// //             }
// //         }
// //     }
// //     }
    
    
// //     fn main () {
// //         // let student01 = Student {
// //         //     name: String::from("Areeb"),
// //         // };
// //         // println!("{:#?}",student01);
    
// //         let student_02 = alpha::Student::new(String::from("Areeb"));
// //         println!("{:#?}",student_02);
        
// //     }
    




    fn main () {
        println!("{}",add(9.8,9,'+'));
    }

    //Into is a trait, into is the method in Into trait.

    fn add <T,U> (x:T, y:U,c: char) -> f64
//     where T: Into<f64>,
//           U: Into<f64>
//     {   match c {
//         '-' => x.into() * y.into(),
//         '+' => x.into() + y.into(),
//         '*' => x.into() / y.into(),
//         '/' => x.into() - y.into(),
//         '%' => x.into() + y.into(),
//         _ => {
//             println!("Choose Correct Operation.");
//             0.0
//         }

//     }
        
//  }





























