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

// Function Index
// ---------------

// is_internet_on           Public  40      Checks to see if an internet connection can be reached by pinging a supplied external IP address.
// is_ip_address_reachable  Public  40      Checks to see if a device IP address can be reached by pinging. Based on "is_internet_on". Can be used to check internet connection.

// my_sql_logon                 
// my_sql_create_a_table    Public  10      Creates a table in the database.
//                                          NOTE: we shoudln't need to use this? possibly for user tables? But likely a better way.



pub mod verifie_database_functions {

    // SET DEBUG MODE: true / false.
    static DEBUG_MODE : bool = false;

    // Call external modules used in the following within this library of modules.
    //
    // Module                           // Dependant to:  
    // ----------------------------------------------------------------------------              
    use std::process::Command;          // is_ip_address_reachable
    use std::str;                       //
    use mysql as mysql_database;        //
    use std::io;                        // get_user_input , 


    

    // ################################################################################################
    // is_ip_address_reachable
    // database::verifie_database_functions::is_ip_address_reachable(ip)
    //
    // Public function.
    // 
    // Copyright :      Copyright PME.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Device or Internet Checker - Do we have a connection to a device or an outside line?
    //                  We can ping any IP address by calling the function and passing it a string containing the IP address.
    //                  If the packet is returned from the ping, we reached the outside and the function returns "true". If the
    //                  ping fails, the function returns false.  We would (currently) ping 8.8.8.8 (Google DNS) for a strong
    //                  indicator, but indeed any major or owned IP would suffice.
    // 
    // Status :         40 - Coder has run some exception testing. pre-release Quality. No peer testing.
    // 
    // Version History :        
    //                  v1.00-201908011211 PME Renamed function from is_internet_on to is_ip_address_reachable
    //                  v0.01-201907292252 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                   
    //

    pub fn is_ip_address_reachable(ping_address: &str) -> bool {

        // If debug mode is on, announce.
        if DEBUG_MODE {
            println!("WARNING: Debug Mode : {}", DEBUG_MODE);
        }


        // Print the IP address we intend to test
        if DEBUG_MODE {
            println!("\n\n DEBUG: Testing to see if the internet is on or off. Pinging external IP address once: {}", ping_address);
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
        if DEBUG_MODE {
            println!("\n DEBUG: Ping results: {} \n", ping_command);
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
    // get_user_input
    // database::verifie_database_functions::get_user_input
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Take a user input.
    // 
    // Input :          Called with user information to clarify what user information is required.
    // Output / Action: Takes and returns a user input. No sanitization.
    // 
    // Status :         10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
    // 
    // Version History :        
    //                  v0.01 PME 2019/08/1 1338 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on https://doc.rust-lang.org/std/io/index.html 
    //
    // Key notes
    //                
    // TODO :    
    //

    pub fn get_user_input(user_query: &str) -> String {

        // Get a user input.  Display user query.
        let mut input  = String::new();
        println!(" {}", user_query);
        io::stdin().read_line(&mut input).unwrap();
        input
    }
   // ---------------------------------------------------------------------------------------------END






    // ################################################################################################
    // my_sql_logon
    // database::verifie_database_functions::my_sql_logon
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Log on to MySQL to allow software access.
    // 
    // Input :          Called without credentials.
    // Output / Action: Called prior to a MySQL command.
    // 
    // Status :         10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/29 2252 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                
    // TODO : Pull logon credentials from a separate file.   
    //

    pub fn my_sql_logon() {
            
        // If debug mode is on, announce.
        if DEBUG_MODE {
            println!("WARNING: Debug Mode : {}", DEBUG_MODE);
        }

        // MySQL Secret passwords and access credentials.
        let my_sql_username =   "root";
        let my_sql_password =   "d4tabasePW";
        let my_sql_port =       "3306";
        let my_sql_ip =         "localhost";


        // Connect to database.
        // See docs on the `OptsBuilder`'s methods for the list of options available via URL.

        // Construct login string from the credentials
        let my_sql_access = construct_login_string(my_sql_username, my_sql_password, my_sql_port, my_sql_ip);

        // Login to MySQL.
        if DEBUG_MODE {
            println!("\n DEBUG: Logging into database with these credentials: {}", my_sql_access);
        }

        let pool = mysql_database::Pool::new(my_sql_access).unwrap();
        
        //let pool = mysql_database::Pool::new("mysql://root:d4tabasePW@localhost:3306/mysql").unwrap();

        

        // Debug Mode:
        if DEBUG_MODE {
            println!("\n DEBUG: End of connnection attempt. \n");
        };


    }
    // ---------------------------------------------------------------------------------------------END





    // ################################################################################################
    // construct_login_string
    // Internal : construct_login_string
    //
    // Private function.  Supports my_sql_logon
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    construct a MySQL logon string from supplied credentials.
    //
    // Input :          Called with (MySQL Username, Password, MySQL Port, MySQL Server IP / localhost).
    // Output :         Returns single string containing access credentials formatted for and as used by public function my_sql_logon.
    // 
    // Status :         10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/29 2300 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                   
    //

    // MySQL Connect Functions : Construct and return Login String
    fn construct_login_string(my_sql_username: &str, my_sql_password: &str, my_sql_port: &str, my_sql_ip: &str) -> String {
        
        // Pull the access credentials together in one array with supporting formatting.
        let my_sql_access_content = ["mysql://", my_sql_username, ":", my_sql_password, "@", my_sql_ip, ":", my_sql_port, "/verifie"];

        // Convert the array to a correctly formatted single string.
        let mut my_sql_access = String::from("");             // Start a mutable string, append " to the start.  \u{22} is " character.
        my_sql_access.push_str(&my_sql_access_content.join(""));    // Now add the contents of the array, no separation between elements.
        my_sql_access.push_str("");                           // End the string with "]

        // Debug : Show MySQL access string as returned from this function.
        if DEBUG_MODE {
            println!("\n My_SQL access string {}", &my_sql_access);
        }

        // Now pass back the data in the following string to the function "-> String" which returns the data to the caller.
        my_sql_access

    }
    // ---------------------------------------------------------------------------------------------END




    // ################################################################################################
    // my_sql_whos_there
    // database::verifie_database_functions::my_sql_whos_there
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Check users logged into the database.
    // 
    // Input :          None.
    // Output / Action: Returns list of users logged in to MySQL.
    // 
    // Status :         10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/31 17:00 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                
    // TODO :   
    //

    // pub fn my_sql_show_contents_of_payments() -> String {

    //     let pool = mysql_database::Pool::new("mysql://root:d4tabasePW@localhost:3306/verifie").unwrap();

    //     let return_payments = pool.prep_exec(r"SELECT * FROM payment;", ()).unwrap();

    //     //for row in conn.prep_exec("SELECT * FROM payment;", ()).unwrap() {
    //     //    let (return_payments) = from_row(row.unwrap());
    //     //}
        
    //     println!("MySQL Data {:?}", return_payments);

    //     // Convert the array to a correctly formatted single string.
    //     let return_data = return_payments;

    //     return_data
    // }
    // ---------------------------------------------------------------------------------------------END




    // ################################################################################################
    // my_sql_insert_payee
    // database::verifie_database_functions::my_sql_insert_payee
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Check users logged into the database.
    // 
    // Input :          Value: i32, Name: string.
    // Output / Action: Sanitizes input data. If compliant, data is published to database. Returns true (compliant / published), or false.
    //
    // Dependancies:    local functions: fn sanitize_this(),
    // 
    // Status :         40  Dev Tested       Dev         Coder has run some exception testing. pre-release Quality. No peer testing.
    // 
    // Version History :        
    //                  v1.00 2019/08/01 11:54 - PME Code works well for example and can be templated for sanitize and database table insertion functions.
    //                  v1.00 2019/07/31 17:00 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes:
    //                
    // TODO :   
    //

    pub fn my_sql_insert_payee(amount: &str, name: &str) -> bool {

    // 1 - Undertake Sanitization checks on input data.

        // Look for any non-compliant data. Notably characters and MySQL commands.
        let sanitize_result_a = sanitize_this(amount);
        let sanitize_result_b = sanitize_this(name);

        // Test sanitization checks and act accordingly.  We are checking two results, so both (AND) must be true (passed checks).
        if sanitize_result_a && sanitize_result_b {

    // 2. If sanitization checks passed true, continue with submission of data to database.

            // Log on to database.
            let pool = mysql_database::Pool::new("mysql://root:d4tabasePW@localhost:3306/verifie").unwrap();

            // Pull the content together in one array with supporting formatting.
            let my_sql_payee_content = ["INSERT INTO payment (amount, account_name) VALUES (", amount, ", \u{22}", name, "\u{22})"];

            // Convert the array to a correctly formatted single string.
            let mut my_sql_payee = String::from("");                    // Start a mutable string, append " to the start.  \u{22} is " character.
            my_sql_payee.push_str(&my_sql_payee_content.join(""));      // Now add the contents of the array, no separation between elements.
            my_sql_payee.push_str("");                                  // End the string with "]


            // Debug : Show MySQL access string as returned from this function.
            if DEBUG_MODE {
                println!("\n DEBUG : my_sql_payee = {}", my_sql_payee)
            }

            // MySQL Command using content as previously formed.
            pool.prep_exec(my_sql_payee, ()).unwrap();

            // 
            return true;
        
        }

    // 3. If sanitization checks failed, do not submit data to database, instead rejecting and returning false.
    
        else {
        
            // Reject entire request if non-compliant data found. Return False.
            return false;

        }
    }
    // ---------------------------------------------------------------------------------------------END





    // ################################################################################################
    // my_sql_create_a_table
    // database::verifie_database_functions::my_sql_create_a_table
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Check users logged into the database.
    // 
    // Input :          None.
    // Output / Action: Returns list of users logged in to MySQL.
    // 
    // Status :         10  Non-Functional   Dev         First draft code. Incomplete and likely not to function.
    // 
    // Version History :        
    //                  v1.00 PME 2019/07/31 17:00 - PME Creation.
    // 
    // Useful Rust References:
    //                  Based on crate https://docs.rs/mysql/16.0.2/mysql/    
    //
    // Key notes
    //                
    // TODO :   
    //

    pub fn my_sql_create_a_table() {

        let pool = mysql_database::Pool::new("mysql://root:d4tabasePW@localhost:3306/verifie").unwrap();

        pool.prep_exec(r"CREATE TABLE payment2 (
                         customer_id int not null,
                         amount int not null,
                         account_name text
                     )", ()).unwrap();
        
    }

    // ---------------------------------------------------------------------------------------------END






    // ################################################################################################
    // sanitize_this
    // database::verifie_database_functions::sanitize_this
    //
    // Public function.
    // 
    // Copyright :      Copyright PME 2019.
    // License :        Not for any use by anyone other than verifie ... for now.
    // 
    // Description :    Check users logged into the database.
    // 
    // Input :          MySQL input data to sanitize check.
    // Output / Action: Returns a boolean; true for compliant, false for non-compliant (and likely dangerous).
    // 
    // Status :         40  Dev Tested       Dev         Coder has run some exception testing. pre-release Quality. No peer testing.
    // 
    // Version History :        
    //                  v1.00 PME 2019/08/01 09:48 - PME Commented and completed.
    //                  v0.01 PME 2019/08/01 00:13 - PME Creation.
    // 
    // Useful Rust References:
    //                  .   
    //
    // Key notes
    //                
    //

    pub fn sanitize_this(unsanitized: &str) -> bool {

        
        // DEBUG : Show unsanitized word, then show the banned word list.
        if DEBUG_MODE {
            println!("\n DEBUG : fn sanitize_this(unsanitized: &str)");
            println!("\n DEBUG : -----------------------------------");
            println!("\n DEBUG : Unsanitized word = {}", unsanitized);
            println!("\n DEBUG : MySQL Banned list = {:?}", MYSQL_SANITIZE_BANNED);
        }

        // Setup a variable to count illegal words found.
        let mut illegal_words_found = 0;

        // Check the unsanitized word against each word in the list for a match.
        for illegal_word in MYSQL_SANITIZE_BANNED.iter() {

            // DEBUG : Log the word we are currently checking against.
            if DEBUG_MODE {
                println!(" Checking word: {}", &illegal_word);
            }

            // Undertake the sanitize check.
            if unsanitized.contains(illegal_word) {

                // For each match to a banned word, increment the illegal words found variable by 1.
                illegal_words_found = illegal_words_found + 1;

                // LOG each banned word found.
                println!("\n WARNING! Non-compliant word found: {}", illegal_word);
            }
        }

        // If we found any illegal words, function return false (as failed the requirements of the sanitize checks).
        if illegal_words_found >= 1 {
            println!(" WARNING! Non-compliant word quantity found: {:?}", illegal_words_found);
            println!(" WARNING! Sanitization failed. \n");
            return false;

        // If we reach here, no illegal words were found. Function return true (as passed the requirements of the sanitize checks).
        } else {
            return true;
        }
    }


    // ---------------------------------------------------------------------------------------------END

    static MYSQL_SANITIZE_BANNED : &'static [&str] = &[
        "*",
        "SELECT",
        "DISTINCT",
        "ORDER",
        "BY",
        "WHERE",
        "AND",
        "OR",
        //"IN",
        "BETWEEN",
        "LIKE",
        "LIMIT",
        "IS NULL",
        "Table",
        "Column",
        "Aliases",
        "Joins",
        "INNER",
        "JOIN",
        "LEFT",
        "RIGHT",
        "Self",
        "CROSS",
        "GROUP",
        "HAVING",
        "ROLLUP",
        "Subquery",
        "Derived",
        "EXISTS",
        "SHOW",
        "UNION",
        "MINUS",
        "INTERSECT",
        "Update",
        "INSERT",
        "IGNORE",
        "DELETE",
        "ON",
        "DELETE",
        "CASCADE",
        "REPLACE",
        "MYSQL",
        "DATA",
        "DEFINITION",
        "Selecting",
        "CREATE",
        "DATABASE",
        "DROP",
        "Managing",
        "Storage",
        "Engines",
        "Types",
        "TABLE",
        "Primary",
        "Foreign",
        "UNIQUE",
        "Constraint",
        "CHECK",
        "Emulation",
        "NOT",
        "NULL",
        "ALTER",
        "ADD",
        "COLUMN",
        "DROP",
        "RENAME",
        "Temporary",
        "Tables",
        "TRUNCATE",
        "-p",
        "-u",
    ];


}


