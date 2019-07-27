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
mod front_of_house {

    // Module within a module
    mod hosting {

        // Function within a module
        // Note, we define a function type when taking in data to a function.
        fn add_to_waitlist(data_input: &str) {
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

// This is a public function.  This means it and anything it pulls is seen by public (out of the here).
pub fn eat_at_restaurant() {

    let data_1 = "eat";
    let data_2 = "cake";

    // Absolute path - This is always done first.  Note we are calling the function here...  We don't specify a data type when calling a function.
    crate::front_of_house::hosting::add_to_waitlist(data_1);

    // Relative path - We can do this second, onwards.  Note we are calling the function here...
    front_of_house::hosting::add_to_waitlist(data_2);
}