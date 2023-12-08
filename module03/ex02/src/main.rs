use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Binary;
use std::fmt::Alignment;

struct John;

// impl Display for John {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
//         if let Some(precision) = f.precision() {
//             if precision == 0 {
//                 write!(f, "Don't try to silence me!")
//             }
//             else {
//                 // If we received a precision, we use it.
//                 write!(f, "{1:.*}", precision, "Hey! I'm John.")
//             }

//         } else {
//             // Otherwise we default to 2.
//            f.pad("Hey! I'm John.")
//         }

//     }
// }

impl Display for John {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.precision() == Some(0) {
            f.write_str("Don't try to silence me!")
        } else {
            f.pad("Hey! I'm John.")
        }
    }
}

impl Debug for John {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "John, the man himself. He's handsome AND formidable.")
        } else {
            write!(f, "John, the man himself.")

        }
    }
}

impl Binary for John {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bip Boop?")
    }
}


fn main() {
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}