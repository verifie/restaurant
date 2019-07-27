// crate root
// Example from https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

// 20190727 1012 PME Learning about modules.


// Top module
mod front_of_house {

    // Module within a module
    mod hosting {

        // Function within a module
        fn add_to_waitlist() {}

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