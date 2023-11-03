extern crate regex;

use regex::Regex;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, ErrorKind, Write};
use std::os::unix::fs as other_fs;
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};

fn pwd() -> Result<(), io::Error> {
    // Get and print the current directory
    let path = env::current_dir()?;
    println!("{}", path.display());
    Ok(())
}

fn echo(args: Vec<String>) -> Result<(), io::Error> {
    match args.len() {
        2 => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Not enough arguments.",
        )),
        3 => {
            if args[2] == "-n" {
                Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Can't call 'echo -n' on nothing.",
                ))
            } else {
                println!("{}", args[2]);
                Ok(())
            }
        }
        _ => {
            let last = args.len() - 1;
            if args[2] == "-n" {
                /*
                    If we have the '-n' option, iterate over the args and print them on
                    the same line with a space between them and without a newline at the final
                */
                for (index, arg) in args.iter().enumerate().skip(3) {
                    if index == last {
                        // Print the last element without the space
                        print!("{}", arg);
                    } else {
                        print!("{} ", arg);
                    }
                }
                Ok(())
            } else {
                /*
                    If we don't have the '-n' option, iterate over the args and print them on the
                    same line with a space between them
                */
                for (index, arg) in args.iter().enumerate().skip(2) {
                    if index == last {
                        // Print the last element without the space
                        print!("{}", arg);
                    } else {
                        print!("{} ", arg);
                    }
                }
                // Print a newline at the end
                println!();
                Ok(())
            }
        }
    }
}

fn cat(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'cat' on nothing.",
        ))
    } else {
        // Iterate over the arguments
        for arg in args.iter().skip(2) {
            // Read the content of each argument and print it
            let file = fs::read_to_string(arg)?;
            print!("{}", file);
        }
        Ok(())
    }
}

fn mkdir(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'mkdir' on nothing.",
        ))
    } else {
        // Iterate over the arguments
        for arg in args.iter().skip(2) {
            // Create each directory
            fs::DirBuilder::new().create(arg)?;
        }
        Ok(())
    }
}

fn mv(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'mv' on nothing.",
        ))
    } else {
        // Get the last argument (destination)
        let dest = args.len() - 1;
        // Iterate over all arguments and move them to the destination
        for arg in args.iter().skip(2) {
            fs::rename(arg, &args[dest])?;
        }
        Ok(())
    }
}

fn ln(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'ln' on nothing.",
        ))
    } else {
        if args[2] == "-s" || args[2] == "--symbolic" {
            // Make a symbolic link if option '-s' or '--symbolic' is provided
            other_fs::symlink(&args[3], &args[4])?;
        } else {
            if args[2].starts_with("-") {
                // If any other option is provided, return error
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Can't use this option on 'ln'.",
                ));
            } else {
                // If no option is provided, make a hard link
                fs::hard_link(&args[2], &args[3])?;
            }
        }
        Ok(())
    }
}

fn rmdir(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'rmdir' on nothing.",
        ))
    } else {
        // Iterate over the arguments and remove them
        for arg in args.iter().skip(2) {
            if Path::new(arg).is_dir() {
                fs::remove_dir(arg)?;
            } else {
                // Handle case when rmdir is used on files
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Not a directory.",
                ));
            }
        }
        Ok(())
    }
}

fn rm(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'rm' on nothing.",
        ));
    } else if args[2] == "-d" || args[2] == "--dir" {
        if args.len() == 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't use 'rm -d' like this.",
            ));
        } else {
            if args[3] == "-r" || args[3] == "-R" || args[3] == "--recursive" {
                // Iterate over the arguments
                for arg in args.iter().skip(4) {
                    let path = Path::new(arg);
                    if path.is_file() {
                        // Remove the file
                        fs::remove_file(path)?;
                    } else {
                        // Remove directory recursively
                        fs::remove_dir_all(path)?;
                    }
                }
            } else {
                // Iterate over the arguments
                for arg in args.iter().skip(3) {
                    let path = Path::new(arg);
                    if path.is_file() {
                        // Remove the file
                        fs::remove_file(path)?;
                    } else {
                        // Remove directory without content
                        fs::remove_dir(path)?;
                    }
                }
            }
        }
    } else if args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive" {
        if args.len() == 3 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't use 'rm -r' like this.",
            ));
        } else {
            if args[3] == "-d" || args[3] == "--dir" {
                // Iterate over the arguments
                for arg in args.iter().skip(4) {
                    let path = Path::new(arg);
                    if path.is_file() {
                        // Remove the file
                        fs::remove_file(path)?;
                    } else {
                        // Remove directory recursively
                        fs::remove_dir_all(path)?;
                    }
                }
            } else {
                // Iterate over the arguments
                for arg in args.iter().skip(3) {
                    let path = Path::new(arg);
                    if path.is_file() {
                        // Remove the file
                        fs::remove_file(path)?;
                    } else if path.is_dir() {
                        // Remove directory recursively
                        fs::remove_dir_all(path)?;
                    }
                }
            }
        }
    } else {
        // Iterate over the arguments
        let mut is_directory = false;
        for arg in args.iter().skip(2) {
            let path = Path::new(arg);
            // Check if each argument is a file
            if path.is_file() {
                // Remove only files (in this case, when we don't have options)
                fs::remove_file(arg)?;
            } else {
                // If not, you can't remove directories without an option
                is_directory = true;
            }
        }
        // Check if we encountered directories and return error
        if is_directory {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Can't remove directory without options.",
            ));
        }
    }
    Ok(())
}

fn cp(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'cp' with no arguments.",
        ));
    } else if args[2] == "-r" || args[2] == "-R" || args[2] == "--recursive" {
        let src = Path::new(&args[3]);
        let dest = PathBuf::new().join(&args[4]);

        if src.is_dir() {
            // If source is a directory, perform a recursive copy
            copy_r(src, &dest)?;
        } else {
            // If source is a file, perform a regular file copy
            let file_name = match src.file_name() {
                Some(f) => f,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid source file path.",
                    ))
                }
            };
            let dest_file = dest.join(file_name);
            fs::copy(src, dest_file)?;
        }
    } else {
        // If '-r' is not an option, perform a regular copy
        let src = Path::new(&args[2]);
        let dest = Path::new(&args[3]);
        // If destination is a directory, copy the entire file
        if dest.is_dir() {
            let file_name = match src.file_name() {
                Some(f) => f,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid source file path.",
                    ))
                }
            };
            let dest_file = dest.join(file_name);
            fs::copy(src, dest_file)?;
        } else {
            // If destination is a file, rename the file and copy its contents
            fs::copy(src, dest)?;
        }
    }
    Ok(())
}

fn copy_r(source: &Path, destination: &PathBuf) -> io::Result<()> {
    let src = Path::new(source);
    let dest = PathBuf::new().join(destination);

    // If the source is a directory, create the destination directory
    if src.is_dir() {
        // Handle the rename case
        let mut dest_copy = dest.clone();
        // If destination exists, add source name to the destination
        if dest.exists() {
            dest_copy = dest_copy.join(match src.file_name() {
                Some(f) => f,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid source file path.",
                    ))
                }
            });
        }
        fs::create_dir_all(&dest_copy)?;
        // Iterate over the entries
        for entry in fs::read_dir(src)? {
            // Add the entry name to the destination path
            let entry = entry?;
            let entry_path = entry.path();
            let dest_path = dest_copy.join(match entry_path.file_name() {
                Some(f) => f,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid source file path.",
                    ))
                }
            });

            if entry_path.is_dir() {
                // If the entry is a subdirectory, recursively copy it
                copy_r(&entry_path, &dest_path)?;
            } else {
                // If the entry is a file, copy it to the destination path
                fs::copy(&entry_path, &dest_path)?;
            }
        }
    } else {
        // If the source is a file, copy it to the destination
        if dest.is_dir() {
            // If the destination is a directory, create a file inside it
            let file_name = match src.file_name() {
                Some(f) => f,
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid source file path.",
                    ))
                }
            };
            let dest_file = dest.join(file_name);
            fs::copy(src, dest_file)?;
        } else {
            // If the destination is a file, perform a regular file copy
            fs::copy(src, dest)?;
        }
    }
    Ok(())
}

fn chmod(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'chmod' like this.",
        ));
    } else {
        // Handle invalid options of 'chmod'
        if args[2].starts_with("-") {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't use 'chmod' with this option.",
            ));
        } else if args[2].parse::<u8>().is_ok() {
            // Check if the permissions are specified in numbers and transform them into octal base
            let octal_representation = u32::from_str_radix(&args[2], 8);
            match octal_representation {
                Ok(octal) => {
                    let path = Path::new(&args[3]);
                    let new_permissions = PermissionsExt::from_mode(octal);
                    // Set the new permissions to the specified path and handle possible errors
                    if let Err(_) = fs::set_permissions(path, new_permissions) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Can't set permissions.",
                        ));
                    }
                }
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        } else {
            // Perform a transformation from symbolic permissions to octal
            let octal_representation = symbolic_to_octal(&args[2], &args[3]);
            match octal_representation {
                Ok(octal) => {
                    let path = std::path::Path::new(&args[3]);
                    let new_permissions = PermissionsExt::from_mode(octal);
                    // Set the new permissions to the specified path and handle possible errors
                    if let Err(_) = fs::set_permissions(path, new_permissions) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "Can't set permissions.",
                        ));
                    }
                }
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidData, e)),
            }
        }
    }
    Ok(())
}

fn symbolic_to_octal(symbolic_permissions: &str, file: &str) -> Result<u32, io::Error> {
    let mut user_category = String::from("");
    let mut permissions = String::from("");
    let mut sum_permissions = (0, 0, 0); // tuple to handle rwx
    let mut operation = '+';

    // Separate user category, the operation performed and permissions
    for char in symbolic_permissions.chars() {
        match char {
            'u' | 'g' | 'o' | 'a' => user_category.push(char),
            '+' | '-' => operation = char,
            'r' | 'w' | 'x' => permissions.push(char),
            _ => (),
        }
    }

    // For every user category specified, match the permissions and perform a sum
    for char in user_category.chars() {
        match char {
            'u' => {
                // Iterate over each permissions and add it to the permissions tuple (user)
                for item in permissions.chars() {
                    match item {
                        'r' => sum_permissions.0 += 4,
                        'w' => sum_permissions.0 += 2,
                        'x' => sum_permissions.0 += 1,
                        _ => (),
                    }
                }
            }
            'g' => {
                // Iterate over each permissions and add it to the permissions tuple (groups)
                for item in permissions.chars() {
                    match item {
                        'r' => sum_permissions.1 += 4,
                        'w' => sum_permissions.1 += 2,
                        'x' => sum_permissions.1 += 1,
                        _ => (),
                    }
                }
            }
            'o' => {
                // Iterate over each permissions and add it to the permissions tuple (other)
                for item in permissions.chars() {
                    match item {
                        'r' => sum_permissions.2 += 4,
                        'w' => sum_permissions.2 += 2,
                        'x' => sum_permissions.2 += 1,
                        _ => (),
                    }
                }
            }
            'a' => {
                // Iterate over each permissions and add it to the permissions tuple (all)
                for item in permissions.chars() {
                    match item {
                        'r' => {
                            sum_permissions.0 += 4;
                            sum_permissions.1 += 4;
                            sum_permissions.2 += 4;
                        }
                        'w' => {
                            sum_permissions.0 += 2;
                            sum_permissions.1 += 2;
                            sum_permissions.2 += 2;
                        }
                        'x' => {
                            sum_permissions.0 += 1;
                            sum_permissions.1 += 1;
                            sum_permissions.2 += 1;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    // Concatenate the tuple into a string to transform it from str to octal
    let octal_string = sum_permissions.0.to_string()
        + &sum_permissions.1.to_string()
        + &sum_permissions.2.to_string();
    // Transform the string into octal base and handle possible errors
    let octal_representation = match u32::from_str_radix(&octal_string, 8) {
        Ok(v) => v,
        Err(e) => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
        }
    };
    // Get current permissions of the file
    let current_permissions = std::fs::metadata(file)?.permissions().mode();
    let result = match operation {
        // If we have to add the new permissions, perform OR bitwise operation between current permissions and new permissions
        '+' => current_permissions | octal_representation,
        // If we have to subtract the new permissions, perform XOR bitwise operation between current permissions and new permissions
        '-' => current_permissions ^ octal_representation,
        // Handle other operations
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid operation.",
            ))
        }
    };
    Ok(result)
}

fn touch(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use just 'touch'.",
        ));
    } else if args.len() == 3 {
        let file = Path::new(&args[2]);
        if !file.exists() {
            // If file doesn't exists, create it (which modifies 'modify time')
            File::create(&args[2])?;
        } else {
            // If file exists, truncate it (which modifies 'modify time')
            File::create(&args[2])?;
            // And read the contents of it (which modifies 'access time')
            fs::read_to_string(&args[2])?;
        }
    } else if args.len() == 4 {
        if args[2] == "-a" {
            // Read the contents of the file (which modifies 'access time')
            fs::read_to_string(&args[3])?;
        } else if args[2] == "-m" {
            // Read the contents of the file
            let file = fs::read_to_string(&args[3])?;
            // Create a new file with the same name
            let mut new_file = File::create(&args[3])?;
            // Write to the new file the contents of the original file (which modifies 'modify time')
            new_file.write_all(file.as_bytes())?;
        } else if args[2] == "-c" || args[2] == "--no-create" {
            // If file exists, change modify time
            if Path::new(&args[3]).exists() {
                File::create(&args[3])?;
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Can't use 'touch' like this.",
            ));
        }
    } else if args.len() == 5 {
        // If path exists, change modify time
        if Path::new(&args[4]).exists() {
            if ((args[2] == "-c" || args[2] == "--no-create")
                && (args[3] == "-a" || args[3] == "-m"))
                || ((args[2] == "-a" || args[2] == "-m")
                    && (args[3] == "-c" || args[3] == "--no-create"))
            {
                File::create(&args[3])?;
            }
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can't use 'touch' like this.",
        ));
    }
    Ok(())
}

fn ls(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() == 2 {
        // List the current directory
        let paths = fs::read_dir(".");
        match paths {
            Ok(paths) => {
                // For each path in the current directory, print it to the screen and skip the hidden files and directories
                for path in paths {
                    let entry = path.unwrap();
                    let file_name = entry.file_name();
                    if !file_name.to_string_lossy().starts_with('.') {
                        // Skip hidden files and directories
                        let trimmed = file_name.to_string_lossy();
                        println!("{}", trimmed);
                    }
                }
            }
            // Handle possible errors
            Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
        }
    } else if args.len() == 3 {
        // List the current directory including hidden files and directories
        if args[2] == "-a" || args[2] == "--all" {
            println!(".");
            println!("..");
            let paths = fs::read_dir(".");
            match paths {
                Ok(paths) => {
                    // For each path in the current directory, print it to the screen and skip the hidden files and directories
                    for path in paths {
                        let entry = path.unwrap();
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
                // Handle possible errors
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
            }
        } else {
            // Print path if it is a file
            let file = Path::new(&args[2]);
            if file.is_file() {
                println!("{}", file.to_string_lossy());
            } else {
                // If path is a directory, iterate over it and print all files and directories, and skip the hidden ones
                let paths = fs::read_dir(file);

                match paths {
                    Ok(val) => {
                        for path in val {
                            let entry = path.unwrap();
                            let file_name = entry.file_name();
                            if file_name.to_string_lossy().starts_with('.') {
                                // Skip hidden files and directories
                                continue;
                            }
                            println!("{}", file_name.to_string_lossy());
                        }
                    }
                    // Handle possible errors
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
                }
            }
        }
    } else if args.len() == 4 {
        if args[2] == "-a" || args[2] == "--all" {
            println!(".");
            println!("..");
            // For each path in the current path, print it to the screen including the hidden files and directories
            let paths = fs::read_dir(&args[3]);
            match paths {
                Ok(val) => {
                    for path in val {
                        let entry = path.unwrap();
                        println!("{}", entry.file_name().to_string_lossy());
                    }
                }
                // Handle possible errors
                Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
            }
        } else if args[2] == "-r" || args[2] == "-R" {
            // Recursive print all entries of the given path
            recursive_ls(&args[3], false);
        }
    } else if args.len() == 5 {
        if (args[2] == "-a" || args[2] == "--all") && (args[3] == "-R" || args[3] == "-r")
            || (args[3] == "-a" || args[3] == "--all") && (args[2] == "-R" || args[2] == "-r")
        {
            // Recursive print all entries of the given path (including hidden ones)
            recursive_ls(&args[4], true);
        }
    }
    Ok(())
}

fn recursive_ls(dir_path: &str, is_visible: bool) {
    if let Ok(paths) = fs::read_dir(dir_path) {
        // If the given path is a directory, print it with ":" and display all of its entries
        if Path::new(dir_path).is_dir() {
            println!("{}:", Path::new(dir_path).display());
            display(&PathBuf::from(dir_path), is_visible);
        }
        // For each entry in the current path, check if it is a directory and perform a recursive call
        for path in paths {
            if let Ok(entry) = path {
                let entry_path = entry.path();
                if entry.path().is_dir() {
                    // Call the function recursive with the entry path if it is a directory
                    recursive_ls(&entry_path.to_string_lossy(), is_visible);
                }
            }
        }
    }
}

fn display(files: &PathBuf, is_visible: bool) -> () {
    // Print all the entries in the given path
    if let Ok(paths) = fs::read_dir(files) {
        for path in paths {
            if is_visible {
                println!(".");
                println!("..");
            }
            if let Ok(entry) = path {
                // If entry starts with '.' means that it is hidden
                let file_name = entry.file_name();
                if file_name.to_string_lossy().starts_with('.') {
                    // Check if '-a' is an option (is_visible bool)
                    if is_visible {
                        // Print the hidden entry and continue
                        println!("{}", file_name.to_string_lossy());
                        continue;
                    } else {
                        // Continue if entry is hidden but option '-a' is not provided
                        continue;
                    }
                }
                // Print the entry if it isn't hidden
                println!("{}", file_name.to_string_lossy());
            }
        }
    }
}

fn grep(args: Vec<String>) -> Result<(), io::Error> {
    if args.len() < 4 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Invalid operation",
        ));
    } else {
        if args.len() == 4 {
            if let Ok(file) = File::open(&args[3]) {
                // Check if regex is valid
                let regex = match Regex::new(&(&args[2])) {
                    Ok(r) => r,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
                };
                // Get contents of file to apply regex
                let reader = BufReader::new(file);

                // Iterate over the lines of file contents and verify if it is a match with the regex
                for line in reader.lines() {
                    let line = line?;
                    if regex.is_match(&line) {
                        // Print the line
                        println!("{}", line);
                    }
                }
            }
        } else if args.len() == 5 && &args[2] == "-i" {
            if let Ok(file) = File::open(&args[4]) {
                // Check if regex is valid
                let regex = match Regex::new(&args[3]) {
                    Ok(r) => r,
                    Err(e) => return Err(io::Error::new(io::ErrorKind::InvalidInput, e)),
                };
                // Get contents of file to apply regex
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line?;
                    // Iterate over the lines of file contents and verify if it is not a match with the regex
                    if !regex.is_match(&line) {
                        // Print the line
                        println!("{}", line);
                    }
                }
            }
        } else {
            // Return error for other options
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid operation",
            ));
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Get the args

    // Match the command, match the call of the function and return the specific error code
    if args[1] == "pwd" && args.len() == 2 {
        let _ = pwd();
    } else if args[1] == "echo" {
        match echo(args) {
            Err(_e) => std::process::exit(-10),
            _ => (),
        }
    } else if args[1] == "cat" {
        match cat(args) {
            Err(_e) => std::process::exit(-20),
            _ => (),
        }
    } else if args[1] == "mkdir" {
        match mkdir(args) {
            Err(_e) => std::process::exit(-30),
            _ => (),
        }
    } else if args[1] == "mv" {
        match mv(args) {
            Err(_e) => std::process::exit(-40),
            _ => (),
        }
    } else if args[1] == "rmdir" {
        match rmdir(args) {
            Err(_e) => std::process::exit(-60),
            _ => (),
        }
    } else if args[1] == "ln" {
        match ln(args) {
            Err(e) => match e.kind() {
                ErrorKind::InvalidInput => {
                    eprintln!("Invalid command");
                    std::process::exit(-1);
                }
                _other_error => std::process::exit(-50),
            },
            _ => (),
        }
    } else if args[1] == "rm" {
        match rm(args) {
            Err(e) => match e.kind() {
                ErrorKind::InvalidInput => {
                    eprintln!("Invalid command");
                    std::process::exit(-1);
                }
                _other_error => std::process::exit(-70),
            },
            _ => (),
        }
    } else if args[1] == "cp" {
        match cp(args) {
            Err(_e) => std::process::exit(-90),
            _ => (),
        }
    } else if args[1] == "chmod" {
        match chmod(args) {
            Err(e) => match e.kind() {
                ErrorKind::InvalidInput => {
                    eprintln!("Invalid command");
                    std::process::exit(-1);
                }
                _other_error => std::process::exit(-25),
            },
            _ => (),
        }
    } else if args[1] == "touch" {
        match touch(args) {
            Err(_e) => std::process::exit(-100),
            _ => (),
        }
    } else if args[1] == "ls" {
        match ls(args) {
            Err(_e) => std::process::exit(-80),
            _ => (),
        }
    } else if args[1] == "grep" {
        match grep(args) {
            Err(_e) => (),
            _ => (),
        }
    } else {
        // Handle the case when command doesn't have an implementation
        println!("Invalid command");
        std::process::exit(-1);
    }
    std::process::exit(0);
}
