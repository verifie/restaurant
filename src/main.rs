// crate root
// Example from https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
// Expanded to trial Git and GitHub.

// 0.02 201908011218 PME - Learning about database interaction. General code tidy.
// 0.01 201907281040 PME - Learning about modules.
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




// 0. Setup / configuration for function.

    // SET GLOBAL DEBUG MODE:
    // Options: true / false.
    // NOTE: This variable must be immutable for two reasons.  Firstly to prevent attackers changing the state to read verbose output 
    // and 2 because the software does not handle an {unsafe} immutable variable.
    static DEBUG_MODE : bool = false;




// 1. General experimentation with libraries (modules).
// ----------------------------------------------------

    println!("\n -------------------------------------------------------------------------- ");

    println!("\n 1. General experimentation with libraries (modules).");

    println!("\n Hello World!");

    // Try calling a function in a module.
    let my_data = "bobby";

    // Try using a function within two levels.  Call with local data.
    lib::front_of_house::hosting::add_to_waitlist(my_data);

    // Try using a top level function from lib.  Use functions own data.  
    // Note this calls a module which calls a further function with specified data.
    lib::eat_at_restaurant();

    // Now try a function which in turn calls other private modules in the lib module
    lib::serve_order(true);

    println!("\n -------------------------------------------------------------------------- ");




// 2. TEST verifie FUNCTION : is_ip_address_reachable.
// ---------------------------------------------------

    println!("\n 2. TEST verifie FUNCTION : is_ip_address_reachable.");

    // Internet test: try the first module library function created by verifie.
    let test_ip = "8.8.8.8";
    let internet_test_results: bool = database::verifie_database_functions::is_ip_address_reachable(test_ip);
    
    if DEBUG_MODE {
        println!(" Result of internet test : {} \n\n", internet_test_results);
    }

    // Check this works by testing a non-existant IP address.  It should fail. Note, failure results in a long pause!
    //let test_false_ip = "8.8.8.9";
    //let internet_test_results_false: bool = database::verifie_database_functions::is_ip_address_reachable(test_false_ip);
    //println!(" Result of FALSE internet test : {}", internet_test_results_false);

    println!("\n -------------------------------------------------------------------------- ");




// 3. Database interaction.
// ------------------------

    println!("\n 3. Database interaction.");


    // This doesnt seem to work.... we still need to log in prior to each action.
    //database::verifie_database_functions::my_sql_logon();


    // Database entry: Enter some data into the table "payment".
    let payee  = database::verifie_database_functions::get_user_input(" Who is money owed to? ");
    let amount = database::verifie_database_functions::get_user_input(" How much is owed? ");
    

    // Database interaction, insert data into MySQL 'payment' table.  The function actions shall first sanitize, then insert the data into 
    // the database, returning True or false, depending on the outcome of both actions. Alert or action as appropriate.
    if database::verifie_database_functions::my_sql_insert_payee(&amount, &payee) {
        println!(" INFO. The database submission was successful.");
    } else {
        println!(" WARNING! The database submission FAILED.");
    };

    database::verifie_database_functions::my_sql_read_table_payments();


    // Cannot make this work! but table has escape strings in it....
    let find_this = "tom";
    database::verifie_database_functions::my_sql_payments_account(find_this);


    let _amount = 50;
    database::verifie_database_functions::my_sql_payments_due_report_value(_amount, true);


    
    println!("\n -------------------------------------------------------------------------- ");




}



// ----------------------------------------------------------------------------------------------------
// Support Functions.


