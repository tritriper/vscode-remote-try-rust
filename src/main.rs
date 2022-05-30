/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/
 
 extern crate ferris_says;

 use std::process::Command;

fn main() {

    let output = Command::new("test/imported_exe/hello_world_V2.0")
                    //  .arg("")
                     .output()
                     .expect("Failed to execute command");
    
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    
    assert!(output.status.success());
    
}