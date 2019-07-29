// ############################################################################################################################################
// Database Module
// database::mod.rs
// 
// Copyright :      Copyright PME.
// License :        Not for any use by anyone other than verifie ... for now.
// 
// Description :   
//                  1.  
// 
// Status :         10 - First draft code. Incomplete and likely not to function
// 
// Version History :        
//                  v1.00 PME 2019/07/29 10:54 - PME Creation.
// 
// Useful Rust References:
//                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
//
// Key notes
//                   
//

// Status Codes Reference
//
// 00  Non-Functional   Stub        No functioning code. Returns True.
// 10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
// 20  Basic Function   Dev         Some inputs and outputs should function in line with spec, but unlikely to function in entirety.
// 30  Trial Function   Dev         Functions as spec requirements, but pre-release quality / tested only by coder for basic function.
// 40  Dev Tested       Dev         Coder has run some exception testing. pre-release Quality. No peer testing.
// 60  Peer Reviewed    Alpha       Code reviewed by a second person. Testing process reviewed and verified. Suitable for Alpha release.
// 70  Third Party      External    Code reviewed by an external team. Approved for beta release "into the wild".
// 80  Beta Release     Beta        Code reviewed by Alpha testers (in-situ environments). Code fixed.  Ready for public Beta release.
// 90  Stable Release   Stable      Beta fixes cycled and a stable version released.
// 100 Obsolete         Obsolete    Obsolete either through general obsolecense, no longer needed or superceeded by newer version or method.
//


// ----------------------------------------------------------------------------------------------------
// verifie_functions
// We'll pop our own libraries in here for now.


pub mod verifie_database_functions {

    // External libraries we use in the following functions.
    use std::process::Command;
    use std::str;
    use mysql as mysql_database;
    

    // ################################################################################################
    // is_internet_on(ip)
    // database::verifie_database_functions::is_internet_on(ip)
    // 
    // Copyright :      Copyright PME.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Internet Checker - Do we have an outside line?
    //                  We can ping any IP address by calling the function and passing it a string containing the IP address.
    //                  If the packet is returned from the ping, we reached the outside and the function returns "true". If the
    //                  ping fails, the function returns false.  We would (currently) ping 8.8.8.8 (Google DNS) for a strong
    //                  indicator, but indeed any major or owned IP would suffice.
    // 
    // Status :         40 - Coder has run some exception testing. pre-release Quality. No peer testing.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/29 2252 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                   
    //

    pub fn is_internet_on(ping_address: &str) -> bool {


    // Debug Mode:
    let debug_mode = true;

    // Print the IP address we intend to test
    if debug_mode {
        println!("\n\n Testing to see if the internet is on or off. Pinging external IP address once: {}", ping_address);
    }

    // Ping test command (Windows - not tested on linux.) Store the text output in
    let ping_command_utf = {
    Command::new("ping")                                // The principal command, without any arguments...
            .args(&[ping_address, "-n", "1"])           // Now present the arguments with "arg", as separators.
            .output()                                   // Return output data.
            .expect("\n\n Internet test PING command failed to execute. \n\n")      // Notify on error.
    };

    // Receive the text output from the ping command.
    
    // Convert UTF Characters to AlpaNumerics.
    // Reference:       https://doc.rust-lang.org/std/str/fn.from_utf8.html
    // Description:     Now use the UTF converter function to reveal the contents of our previous command in human readable form.
    let ping_command = str::from_utf8(&ping_command_utf.stdout).unwrap();


    // Test the resulting text of the ping for "lost = 0".  If this is not found, then one or more packets were lost.

    // Set the relevant search term.
    let search_for_this = "Lost = 0";


    // And then print the results to the screen.
    if debug_mode {
        println!("\nPing results: {} \n", ping_command);
    } 


    // Conduct an IF statement test.
    if ping_command.contains(&search_for_this) {

        // If it does have the search phrase, the internet is connected, so do this...
        println!("\n The internet is connected.");
        return true;

    } else {

        // Uh oh... no internet. So do this...
        println!("\n WARNING : The internet is NOT connected.");
        return false;
    };
}

    // ---------------------------------------------------------------------------------------------END




    // ################################################################################################
    // my_sql_logon()
    // database::verifie_database_functions::my_sql_logon()
    // 
    // Copyright :      Copyright PME.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Log on to MySQL.
    // 
    // Status :         40 - Coder has run some exception testing. pre-release Quality. No peer testing.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/29 2252 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                   
    //

    pub fn my_sql_logon() {

        // Debug Mode:
        let debug_mode = true;

        // MySQL Secret passwords and access credentials.
        let my_sql_username =   "root";
        let my_sql_password =   "d4tabasePW";
        let my_sql_port =       "3307";
        let my_sql_ip =         "localhost";

        // MySQL String construct.
        let my_sql_access_content = ["mysql://", my_sql_username, ":", my_sql_password, ":@", my_sql_ip, ":", my_sql_port, "/mysql"];

        let mut my_sql_access = String::from("\u{22}");             // Start a mutable string, append " to the start.  \u{22}  is "
        my_sql_access.push_str(&my_sql_access_content.join(""));    // Now add the contents of the array, no separation.
        my_sql_access.push_str("\u{22}");                           // End the string with "]



        if debug_mode {
            println!("\n My_SQL access string {}", &my_sql_access);
        }

        // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
        let connect_to_sql = mysql_database::Pool::new(my_sql_access).unwrap();

        println!("End of connnection attempt. \n\n")
    }

    // ---------------------------------------------------------------------------------------------END














}


