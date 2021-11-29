/*


    Check resources.rs for some great resources to learn rust


*/
/***************************************/
/* What IS Rust? (Start)*/



/* What IS Rust? (End)*/
/***************************************/
/*  Installing Rust (Start)*/
    // For Mac And Linux 
        /* 
        1)Type the following command in your terminal
            $ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

        2)Youll also need to download a C compiler which will inculde a linker
            Gnu/Linux)
                Debian Based Distros)
                    $ sudo apt install build-essentials

                Arch Based Distros) 
                    $ sudo pacman -S base-devel

                Fedora and other dnf based package managers disros)
                    $ sudo dnf install make automake gcc gcc-c++ kernel-devel

            MacOS)
                IDK ? I dont have one and you shouldnt too, just kidding
                type the following command itll install a c compiler for you
                    $ xcode-select --install
        */

    // For windows
        /*
            go here , i guess
                https://forge.rust-lang.org/infra/other-installation-methods.html
        */

/* Installing Rust (End)*/
/***************************************/
/* Using Cargo To build and run (Start)*/
    /*
        //make a new project
            1)make a new project in a new folder type the following command
                $ cargo new nameOfYourProject

            2)make a new project in the current directory(folder) type the following command
                $ cargo init

        //build
            //TODO

        //build and run 
            to build and run your project type the following command
                $ cargo run

        //Check
            //TODO


    */
/* Using Cargo To build and run (End)*/
/***************************************/
/* Common Programming Concepts (Start)*/

    /* Comments(Start)*/

        //  Command Line (anything after // is commented)

        /*
            Command Block (anything between is commented)
        */

    /* Comments(End)*/

    /* Variables, Constants , Mutation , Shadowing (Start)*/
        /* Variables (Start)*/
            /*
            //what is a variable
                Variable is a storage, a place or a box which can store values inside of it

            //how to create a variable and how dows it look like
                    let variable_name = value;
                let statement is used to create a variable
                //TODO : Add more details here

            //all about variables in rust
                by default variables are immutable
                    When a variable is immutable, once a value is bound to a name, you can’t change that value. (from Rust Book)
                
                you can't change a variable type once you declared it
                    but you can change value of a mutable variable (we'll learn mutable variables in the future)


            */
        /* Variables (End)*/

        /* Mutation (Start)*/

        /* Mutation (End)*/


        /* Constants (Start)*/

        /* Constants (End)*/

        /* Shadowing (Start)*/

        /* Shadowing (End)*/
    /* Variables, Constants , Mutation(End)*/

/* Common Programming Concepts (End)*/
/***************************************/
