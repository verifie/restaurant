// crate root
// Example from https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// Expanded to trial Git and GitHub.

// 20190727 1054 PME 10 Application of Git and Git Hub. Tutorial here: https://rogerdudler.github.io/git-guide/
// 20190727 1012 PME 00 Learning about modules.
//

// Status Codes
// 00  Non-Functional   Stub        No functioning code. Returns True.
// 10  Non-Functional   Dev         First draft code. Incomplete and likely not to function
// 20  Basic Function   Dev         Some inputs and outputs should function in line with spec, but unlikely to function in entirety
// 30  Trial Function   Dev         Functions as spec requirements, but pre-release quality / tested only by coder for basic function.
// 40  Dev Tested       Dev         Coder has run some exception testing. pre-release Quality. No peer testing
// 60  Peer Reviewed    Alpha       Code reviewed by a second person. Testing process reviewed and verified. Suitable for Alpha release.
// 70  Third Party      External    Code reviewed by an external team. Approved for beta release "into the wild".
// 80  Beta Release     Beta        Code reviewed by Alpha testers (in-situ environments). Code fixed.  Ready for public Beta release
// 90  Stable Release   Stable      Beta fixes cycled and a stable version released.
// 100 Obsolete         Obsolete    Obsolete either through general obsolecense, no longer needed or superceeded by newer version or method.


// Top module
pub mod front_of_house {

    // Module within a module. We make this public.
    pub mod hosting {

        // Function within a module
        // Note, we define a function type when taking in data to a function.
        // And we make the functions public that need to be accessed from the outside.
        pub fn add_to_waitlist(data_input: &str) {
            println!("Data in this time is {}", data_input)
        }

        // Function within a module
        fn seat_at_table() {}
    }


    // Module within a module
    mod serving {

        // Function within a module
        fn take_order() {}

        // Function within a module
        fn serve_order() {}

        // Function within a module
        fn take_payment() {}
    }
}

// This is a public function.  This means it and anything it pulls is seen by external code (public) (out of the here).
// "If you want to make an item like a function or struct private, you put it in a module."
// Source : https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html
pub fn eat_at_restaurant() {

    let data_1 = "eat";
    let data_2 = "cake";

    // Absolute path - This is always done first.  Note we are calling the function here...  We don't specify a data type when calling a function.
    //crate::front_of_house::hosting::add_to_waitlist(data_1);

    // Relative path - We can do this second, onwards.  Note we are calling the function here...
    front_of_house::hosting::add_to_waitlist(data_2);
}



pub fn serve_order(bool_data: bool) {

    println!("Data send to serve_order : {:?}", bool_data);


    println!("Called function - Start");
    back_of_house::fix_incorrect_order(bool_data);
    println!("Called function - END");
}



    mod back_of_house {
        pub fn fix_incorrect_order(bool_data: bool) {

            if bool_data == true {

                // We're not following the guide here.
                cook_order(2);
    
            } else {
                println!("FALSE!!");
            }
            //super::serve_order();
        }

    fn cook_order(number: i32) {
        // TODO ^ Try different values for `number`

        println!("Tell me about {}", number);
        match number {
            // Match a single value
            1 => println!("One!"),
            // Match several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // Match several values
            20 => println!("This is 20"),
            // Match an inclusive range
            13..=19 => println!("A teen"),
            // Handle the rest of cases
            _ => println!("Ain't special"),
        }

        let bob = true;
        // Match is an expression too
        let greg = match bob {
            // The arms of a match must cover all the possible values
            false => 0,
            true => 1,
            // TODO ^ Try commenting out one of these arms
        };

        println!("{} -> {}", bob, greg);


    }
}