use assert_cmd::Command;

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("HW4_2_Fahrenheit").unwrap();
    cmd.assert().success().stdout("Input 3 arguments!\n");
}

#[test]
fn test_100_to_110() {    
    let mut cmd = Command::cargo_bin("HW4_2_Fahrenheit").unwrap();
    cmd.arg("100");
    cmd.arg("110");
    cmd.arg("5");
    cmd.assert().success().stdout("\tFahr\tCelcius\n\t100\t37.8\n\t105\t40.6\n\t110\t43.3\n");
}

#[test]
fn test_110_to_100() {    
    let mut cmd = Command::cargo_bin("HW4_2_Fahrenheit").unwrap();
    cmd.arg("110");
    cmd.arg("100");
    cmd.arg("5");
    cmd.assert().success().stdout("\tFahr\tCelcius\n\t110\t43.3\n\t105\t40.6\n\t100\t37.8\n");
}

#[test]
fn test_neg10_to_10() {    
    let mut cmd = Command::cargo_bin("HW4_2_Fahrenheit").unwrap();
    cmd.arg("-10");
    cmd.arg("10");
    cmd.arg("5");
    cmd.assert().success().stdout("\tFahr\tCelcius\n\t-10\t-23.3\n\t-5\t-20.6\n\t0\t-17.8\n\t5\t-15.0\n\t10\t-12.2\n");
}