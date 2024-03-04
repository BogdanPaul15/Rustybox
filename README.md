# Rustybox
For this homework I've used only standard rust libraries and regex library in order to implement grep.
In the main function, I get all of the arguments provided in the command line and match with the specific function. In case of error, I return the specific error code of the command using <span style="color: red;">**std::process::exit(code)**</span>. If the argument provided is not a function that was implemented for this homework, I simply print the message "**Invalid command**" and return exit code -1. If the command provided doesn't return any error, it returns 0.

I've tried to do some kind of error handling for all the inputs that don't meet the specified requirements.

### <span style="color: blue;">pwd</span>
For this command I get the current working directory and print it to the terminal. 
### <span style="color: blue;">echo [option] arguments</span>
If **-n** is not an option, I iterate over the args and print them on the same line with a space between them.
If **-n** is provided, I iterate over the args and print them on the same line with a space between them and a newline at the end.
### <span style="color: blue;">cat nume_fisiere</span>
Iterate over the args, read their contents and print them to the terminal.
### <span style="color: blue;">mkdir nume_directoare</span>
Iterate over the args and create each directory if it doesn't already exist.
### <span style="color: blue;">mv sursa destinatie</span>
I get the last argument (*which is the destination*), iterate over the remaining arguments and move them to the destination.
### <span style="color: blue;">ln [optiune] sursa nume_link</span>
If **-s** or **--symbolic** is provided, make a symbolic link using **symlink**.
If no option is provided, make a hard link.
### <span style="color: blue;">rmdir nume_directoare</span>
Remove all the empty directories provided as arguments.
### <span style="color: blue;">rm [options] fisiere / directoare</span>
If **-r** option is provided, it removes all the directories and their contents. If some args are also files, it removes only the files and returns specific error code.
If **-d** or **--dir** is provided, it removes all empty directories. 
Combinations of **-d** and **-r** are also available.
If no option is provided, it can't remove directories.
### <span style="color: blue;">ls [options] [director]</span>
Simple **ls** without options prints all the entries in the current working directory (*hidden entries are omitted*). If **-a** or **--all is provided, hidden entries are also printed.
For listing specific paths, if the provided path is a file, print the filename, else list all the entries in the specified path (*hidden entries are omitted*).
If **-a** or **--all is provided, hidden entries are also printed.
If **-r** or **--recursive** is provided (*it can be used with **-a** or **--all***), enter a recursive function called **recursive_ls**, which verifies if path is a subdirectory (*prints its name followed by ":" and displays all of its entries on the next line -> enter a function called **display** which prints to the terminal all entries of the specified path and also hidden entries if **-a** or **--all** is provided (I've used a bool as a parameter to check if hidden entries are allowed or not)*) and iterate over all entries in the current path. If the entry is a directory, recall the recursive function.
### <span style="color: blue;">cp [option] sursa destinatie</span>
If no options are provided, it performs a regular copy between source and destination, based on the destination type. (*if destination is a directory, copy the entire file, else rename the file and copy its contents*)
If **-r**, **-R** or **--recursive** is provided, perform a recursive copy with a function called **copy_r**, which handles the copy like so:
- if source is a directory, handle the rename case like above or create the destination directory. For every entry in source, add the entry name to the destination path and if the entry itself is a subdirectory, recall the function, else copy the entry to the destination path.
- if source is not a directory, perform a regular copy like above and handle the rename case when destination is not a directory.
### <span style="color: blue;">touch [options] fisier</span>
If no options are provided, **touch fisier** modifies *modify time* if the file doesn't exist, else truncate it (which modifies modify time) and read its contents to modify *access time*.
If **-a** is provided, read file contents in order to modify its *access time.*
If **-m** is provided:
- read the contents of the file;
- create a new file with the same name;
- write to the new file the contents of the original file (*which modifies 'modify time'*).
If **-c** or **--no--create** is provided, if file exists, change its modify time, else change nothing.
### <span style="color: blue;">chmod permisiuni fisier / director</span>
Firstly I check if the permissions argument is valid. Then check if it is specified in octal mode or symbolic mode.
For octal mode:
- transform the arg into octal base using **from_str_radix** and set new permissions to the specified file;
For symbolic mode: (**symbolic_to_octal** function)
- I've made a function that helps me to transform symbolic mode to octal base and solve the problem like above. In this function I've separated the user category, operation and permissions into different strings using a match. Then I iterate over the user categories and match the groups (u, g, o, a) to the permissions, in order to perform the sums (r(4), w(2), x(1)). After that, I concatenate the sums (***r + w + x***) into a string and transform it to octal base. I get the current permissions of the file and match the operation:
- '+' - adds the new permissions to the current permissions of the file (*using **OR** bitwise operation*);
- '-' - subtracts the new permissions from the current permissions of the file (*using **XOR** bitwise operation*). The function returns the new permissions and set them in the chmod function.
### <span style="color: blue;">grep [-i] regex nume_fisier</span>
If no option is provided, I read the contents of the file and match the regex with each line. If it is a match, I print the line to the terminal.
If **-i** is provided, I read the contents of the file and match the regex with each line. If it is not a match, print the line to the terminal.
For providing other options that are not implemented, the function returns error code.

## Verify

Run the following commands to test your homework:

You will have to install NodeJS (it is installed in the codespace)

```bash
# Clone tests repository
git submodule update --init 

# Update tests repository to the lastest version
cd tests
git pull 
cd ..

# Install loadash
npm install lodash
```

Install rustybox

```bash
cargo install --path .
```

If the `rustybox` command can't be found, be sure to add the default cargo installation folder into the PATH environment variable

```bash
export PATH=/home/<your username here>/.cargo/bin:$PATH
```

Run tests

```bash
cd tests
# Run all tests 
./run_all.sh

# Run single test
./run_all.sh pwd/pwd.sh
```
