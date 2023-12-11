use std::fmt::Write;

struct EncodingError;
struct DecodingError;

trait Field: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(field: &str) -> Result<Self, DecodingError>;
}

impl Field for String {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        if self.contains("\n") || self.contains(",") {
            return Err(EncodingError);
        }
        target.push_str(self);
        
        return Ok(());
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        return Ok(field.to_string());
    }
}

impl<T: Field> Field for Option<T> {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        match self {
            Some(v) => v.encode(target),
            None => Ok(())
        }
        
    }

    fn decode(field: &str) -> Result<Self, DecodingError> {
        if field.is_empty() {
            return Ok(None)
        } else {
            match T::decode(field) {
                Ok(t) => Ok(Some(t)),
                Err(err) => Err(err),
            }
        }
    }
}

macro_rules! impl_field_for_int {
    ($($t:ty),*) => {
        $(impl Field for $t {
            fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
                match write!(target, "{self}") {
                    Ok(()) => Ok(()),
                    Err(_) => Err(EncodingError),
                }
            }
        
            fn decode(field: &str) -> Result<Self, DecodingError> {
                match field.parse() {
                    Ok(t) => Ok(t),
                    Err(_) => Err(DecodingError)
                }
            }
        })*
    };
}

impl_field_for_int!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);


trait Record: Sized {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError>;
    fn decode(line: &str) -> Result<Self, DecodingError>;
}

impl Record for Field {
    fn encode(&self, target: &mut String) -> Result<(), EncodingError> {
        
    }

    fn decode(line: &str) -> Result<Self, DecodingError> {

    }
    

}

fn main() {

    let mut loader = String::new();
    let f1: String = "yoo".to_string();
    let f2: String = "yoo".to_string();
    let f3: String = "yoo".to_string();
    let f4: i128 = -987426412;
    let f5: u8 = 124;

    f1.encode(&mut loader);
    f2.encode(&mut loader);
    f3.encode(&mut loader);
    f4.encode(&mut loader);
    f5.encode(&mut loader);

    println!("loader= {}", loader);
}
