{   //Comments (1)
    
    // damn i really like these types of comments

    // Command Line
    // everything after `//` will get ignored when compiled

    /*
        Command Block
        everything between `* *` will get ignored when compiled
    */

}

{   // Variables , Consts , Annotating , Mutation , Shadowing (2)
    

    // Variable And Constants
        let age_1 = 11; // use `let` keyworld for declaring a variable
        // age_1 = 12; // => you cant change age_1 value Because  || variables Are `Imutable` by default ||  that means once you assign a value to a immutable variable you cant change it  
        // use  lower case and _ for naming variables && avoid numbers at the start of a name

        let mut age_2 = 21; // use `mut` keyword for making mutable variables
        age_2 = 22; // => you can change age_2 value 

        const AGE_3 = 31; // use `const` keyworld for declaring a constants
        // AGE_3 = 32; // => you cant change AGE_3 value because constants are ALWAYS imutable
        // use  capitals(upper case)  and _ for naming variables && avoid numbers at the start of a name

        // Variable And Constants Differences
            // Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.
                // 1) Variables are imutable by default but Constants aren’t just immutable by default—they’re always immutable.
                // 2) Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.
                // 3) Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

        // NOTE:  YOU CANT CHANGE A VARIABLE OR A CONSTANT TYPE AFTER YOU ASSIGNED A VALUE 
        let age_5 = 51;
        // age_5 = 'c'; you cant do this as said above

    //

    // Annotating
        
        let float_1: f64 = 1.0;  // Regular annotation
        let int_1   = 5i32; // Suffix annotation

        // Or a default will be used.
        let float_default_1   = 3.0; // `f64`
        let int_default_1 = 7;   // `i32`
    //

    //shadowing
        let age_4 = 41;
        { // fake scope , kinda usefull in some situations
            let age_4 = 42; // you can shadow a variable
                            // it doesnt affect the outer age_4 value its just shadowing it

                            // _TODO_
                            // Add more details
        }
    //
}

{   // Data Types

    // Rust is a `statically typed` language, which means that it must know the types of all variables at compile time.
    // Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. 
    // The compiler can usually infer what type we want to use based on the value and how we use it.In cases when many types are possible You as the developer should specify the type.

    //Scalar Type
    
        //A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

        // Integers 

            //An integer is a number without a fractional component. 

            //There is 2 types of integer signed one and unsigned one
            //signed integer types start with i, and unsigned starts with u
            

            // Number literals can also use _ as a visual separator to make the number easier to read
            
            // also if youre intrested in overflows
            // Integer Overflow : https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
            
            // Numeric Operations
            // https://doc.rust-lang.org/book/ch03-02-data-types.html#numeric-operations
        //

        // Floating Points
            // A flot is a number with a fractional component.

            // Number literals can also use _ as a visual separator to make the number easier to read
            // Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision.
        //

        // Bools
            // Booleans only has 2 possible values ` true / false `
            let bool_true: bool = true;
            let bool_false: bool = false;

        //

        // Chars
            // So far we’ve worked only with numbers, but Rust supports letters too. Rust’s char type is the language’s most primitive alphabetic type, and the following code shows one way to use it.
            // Note : char litterals use single quotes '' , string litterals use double quotes ""

        //
    //
        
    // Compound Types
        // Compound types can group multiple values into one type. 
    
        // Tuples
            // A tuple is a collection of values of different types. 
            // Tuples are constructed using parentheses ()
            let long_tuple = ('a',1,true);


            // Tuples can be tuple members
            let tuple_in_a_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

            //tuples can be destructured to create bindings
            let tuple = (1, "hello", 4.5, true);
            let (a, b, c, d) = tuple;
            // a=1 , b= "hello" , c =4.5 , d = true


            // Functions can use tuples to return multiple values, as tuples can hold any number of values.
            // Tuples can be used as function arguments and as return values
            fn get_num_1(a:i32 , b:u32) -> (i32 , u32){
                (a,b) // return (a,b);

                //TODO FIX THIS IM LAZY RN
            }
        //

        // Arrays
            // An array is a collection of objects of the same type T, stored in contiguous memory.
            // Arrays are created using brackets []
            // Arrays' length, which is known at compile time, is part of their type signature [T; length].
            

            // TODO
    
        //

        // Slices
            // TODO

        //

        // Vectors
            // TODO

        //

    //
}
{   // Functions , Methods

}
{   // Control Flow

}