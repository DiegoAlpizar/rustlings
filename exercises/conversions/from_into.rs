// The From trait is used for value-to-value conversions.
// If From is implemented correctly for a type, the Into trait should work conversely.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.From.html
// Execute `rustlings hint from_into` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

// Your task is to complete this implementation
// in order for the line `let p = Person::from("Mark,20")` to compile
// Please note that you'll need to parse the age component into a `usize`
// with something like `"4".parse::<usize>()`. The outcome of this needs to
// be handled appropriately.
//
// Steps:
// 1. If the length of the provided string is 0, then return the default of Person
// 2. Split the given string on the commas present in it
// 3. Extract the first element from the split operation and use it as the name
// 4. If the name is empty, then return the default of Person
// 5. Extract the other element from the split operation and parse it into a `usize` as the age
// If while parsing the age, something goes wrong, then return the default of Person
// Otherwise, then return an instantiated Person object with the results

// I AM NOT DONE

fn checkName (firstTk : Option <&str>) -> Option <String> {

    firstTk.filter( |s| !s.is_empty() )
            .map( |s| s.to_string() )
            
    //firstTk.and_then( |s| (!s.is_empty()).then( || s.to_string() ) )

    //firstTk.filter( |s| !s.is_empty() )
    //        .and_then( |s| Some( s.to_string() ) )
            
    //let nameStr =   firstTk ? ;
    //
    //{!nameStr.is_empty()}
    //                    .then( || nameStr.to_string() )

}


fn checkAge (secondTk : Option <&str>) -> Option <usize> {

    //secondTk.

    secondTk.and_then( |s| s.parse().ok() )

    //let ageStr =   secondTk ? ;
    //let age =   ageStr.parse().ok() ;
    //
    //age

}

impl From<&str> for Person {

    fn from(s: &str) -> Person {

        if s.is_empty() { return  Person::default(); }
        
        let mut tokens  =   s.split( ',' ) ;
        let firstToken  =   tokens.next() ;
        let secondToken =   tokens.next_back() ;
        
        //let nameStr    =   firstToken.unwrap() ;
        
        //if nameStr.is_empty()       { return  Person::default(); }
        //if secondToken.is_none()    { return  Person::default(); }
        
        //let parsedAge: Result<usize, std::num::ParseIntError> =   secondToken.unwrap().parse() ;
        
        //if parsedAge.is_err() { return  Person::default(); }
        
        let checkedName =   checkName( firstToken ) ;
        let checkedAge  =   checkAge( secondToken ) ;

        if let None = checkedAge    { return  Person::default(); }
        if checkedName.is_none()    { return  Person::default(); }

        let name    =   checkedName.unwrap() ;
        let age =   checkedAge.unwrap() ;

        Person  { name
                , age
                }
        
        /*
        let mut tokens  =   s.split( ',' ) ;
        let firstToken  =   tokens.next() ;
        let secondToken =   tokens.next() ;

        if firstToken.is_none() || secondToken.is_none()
        {
            return  Person::default() ;
        }

        let name    =   firstToken.unwrap().to_string() ;
        let age =   secondToken.unwrap().parse().unwrap() ;
        
                */
        
        /*
        let mut tokens   =   s.split( ',' ) ;
        let firstToken    =   tokens.next() ;
        let name    =   firstToken.unwrap_or( "John" ).to_string() ;
        let secondToken  =   tokens.next() ;
        let age  =   secondToken.unwrap_or( "30" ).parse().unwrap_or( 30 ) ;
        
        */
        
    }

}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
