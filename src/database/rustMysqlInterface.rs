// src/database/rustMysqlInterface.rs
// Nased on crate https://docs.rs/mysql/16.0.2/mysql/

// 0.01 00 20190729-1054 PME Creation.
//

// Status Codes
// 00  Non-Functional   Stub        No functioning code. Returns True.
// 10  Non-Functional  Dev         First draft code. Incomplete and likely not to function
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

// ----------------------------------------------------------------------------------------------------
// verifie_functions

// We'll pop our own libraries in here for now.
pub mod verifie_functions {


    // Module within a module. We make this public.
    pub mod hosting {



    // External libraries we use in the following functions.
    use std::process::Command;
    use std::str;

    pub fn is_internet_on(ping_address: &str) -> bool {

        // BUGFIX : Turn on or off.
        let internet_verbose = false;

        // Print the IP address we intend to test
        if internet_verbose {
            println!("\n\n Testing to see if the internet is on or off. Using external IP address : {}", ping_address);
        }

        // Ping test command (Windows - not tested on linux.)
        let show_command = {
        Command::new("ping")                                // The principal command, without any arguments...
                .args(&[ping_address, "-n", "1"])           // Now present the arguments with "arg", as separators.
                .output()                                   // Return output data.
                .expect("\n\n Internet test PING command failed to execute. \n\n")      // Notify on error.
        };


        // UTF Characters to AlpaNumerics.
        // Reference:       https://doc.rust-lang.org/std/str/fn.from_utf8.html
        // Description:     Now use the UTF converter function to reveal the contents of our previous command in human readable form.
        let utf_result = str::from_utf8(&show_command.stdout).unwrap();


        // Test the resulting text of the ping for "lost = 0".  If this is not found, then one or more packets were lost.

        // Set the relevant search term.
        let search_for_this = "Lost = 0";


        // And then print the results to the screen.
        if internet_verbose {
            println!("\nPing results: {} \n", utf_result);
        } 


        // Conduct an IF statement test.
        if utf_result.contains(&search_for_this) {

            // If it does have the search phrase, the internet is connected, so do this...
            println!("\n The internet is connected.");
            return true;

        } else {

            // Uh oh... no internet. So do this...
            println!("\n WARNING : The internet is NOT connected.");
            return false;
        };



        // Now test to see if the word "lost" appears in the resulting string of text (text output from command).
        // If the word does appear, then packets were lost, amd we have an internet connection failure.
        //println!("     There are {} {}s in the list.", count_this.iter().filter(|&n| *n == look_for).count(), look_for);

    }
}


