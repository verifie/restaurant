// crate root
// Example from https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// Expanded to trial Git and GitHub.

// 20190728 1040 PME 00 Learning about modules.
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

// ----------------------------------------------------------------------------------------------------
// Declare the modules in use.

mod lib;
mod database;


// ----------------------------------------------------------------------------------------------------
// Main Function.
fn main() {

    // SET GLOBAL DEBUG MODE:
    // Options: true / false.
    // NOTE: This variable must be immutable for two reasons.  Firstly to prevent attackers changing the state to read verbose output 
    // and 2 because the software does not handle an {unsafe} immutable variable.
    static DEBUG_MODE : bool = false;


    // TEST verifie FUNCTION : is_internet_on

    // Internet test: try the first module library function created by verifie.
    let test_ip = "8.8.8.8";
    let internet_test_results: bool = database::verifie_database_functions::is_internet_on(test_ip);
    
    if DEBUG_MODE {
        println!(" Result of internet test : {} \n\n", internet_test_results);
    }

    // Check this works by testing a non-existant IP address.  It should fail.
    //let test_false_ip = "8.8.8.9";
    //let internet_test_results_false: bool = database::verifie_database_functions::is_internet_on(test_false_ip);
    //println!(" Result of FALSE internet test : {}", internet_test_results_false);



    // Database login
    // This doesnt seem to work.... we still need to log in prior to each action.
    database::verifie_database_functions::my_sql_logon();

    // Do something in the database
    let amount = "160";
    let payee = "john";
    let my_sql_users = database::verifie_database_functions::my_sql_insert_payee(amount, payee);


    // Developing sanitizer
    sanitize_this("hello Alex");


    // ------------------------------------------------------------------------------------
    println!("\n Hello World!");

    // ------------------------------------------------------------------------------------
    // Try calling a function in a module.
    let my_data = "bobby";

    // Try using a function within two levels.  Call with local data.
    lib::front_of_house::hosting::add_to_waitlist(my_data);

    // Try using a top level function from lib.  Use functions own data.  
    // Note this calls a module which calls a further function with specified data.
    lib::eat_at_restaurant();


    // Now try a function which in turn calls other private modules in the lib module
    lib::serve_order(true);

}



// ----------------------------------------------------------------------------------------------------
// Support Functions.
