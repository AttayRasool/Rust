                                                                  # Rust
#Introduction:
        Welcome to The Rust Programming Language. The Rust programming language helps you write faster, more reliable software. High-level ergonomics and low-level control are often at odds in programming language design; Rust challenges that conflict. Through balancing powerful technical capacity and a great developer experience, Rust gives you the option to control low-level details (such as memory usage) without all the hassle traditionally associated with such control.


Rust also brings contemporary developer tools to the systems programming world:

      Cargo, the included dependency manager and build tool, makes adding, compiling, and managing dependencies painless and consistent across the Rust           ecosystem.
      The Rustfmt formatting tool ensures a consistent coding style across developers.
      The Rust Language Server powers Integrated Development Environment (IDE) integration for code completion and inline error messages.
      By using these and other tools in the Rust ecosystem, developers can be productive while writing systems-level code.


#Installation:
  The first step is to install Rust. We’ll download Rust through rustup, a command line tool for managing Rust versions and associated tools. You’ll need an internet connection for the download.
  
#Installing rustup on Linux or macOS
If you’re using Linux or macOS, open a terminal and enter the following command:

                                $ curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

The command downloads a script and starts the installation of the rustup tool, which installs the latest stable version of Rust. You might be prompted for your password. If the install is successful, the following line will appear:

                                          Rust is installed now. Great!
                                          
You will also need a linker, which is a program that Rust uses to join its compiled outputs into one file. It is likely you already have one. If you get linker errors, you should install a C compiler, which will typically include a linker. A C compiler is also useful because some common Rust packages depend on C code and will need a C compiler.

*On macOS, you can get a C compiler by running:

                                              $ xcode-select --install
                                              
Linux users should generally install GCC or Clang, according to their distribution’s documentation. For example, if you use Ubuntu, you can install the build-essential package.


#Installing rustup on Windows
                On Windows, go to                     "https://www.rust-lang.org/tools/install"
                and follow the instructions for installing Rust. At some point in the installation, you’ll receive a message explaining that you’ll also need the MSVC build tools for Visual Studio 2013 or later.

To acquire the build tools, you’ll need to install Visual Studio 2022. When asked which workloads to install, include:

                  “Desktop Development with C++”
                   The Windows 10 or 11 SDK
                   The English language pack component, along with any other language pack of your choosing
The rest of this book uses commands that work in both cmd.exe and PowerShell. If there are specific differences, we’ll explain which to use.

#Troubleshooting
      To check whether you have Rust installed correctly, open a shell and enter this line:

                                                  $ rustc --version
                                                  
You should see the version number, commit hash, and commit date for the latest stable version that has been released, in the following format:

                                                    rustc x.y.z (abcabcabc yyyy-mm-dd)
If you see this information, you have installed Rust successfully! If you don’t see this information, check that Rust is in your %PATH% system variable as follows.

In Windows CMD, use:

                                                          > echo %PATH%
In PowerShell, use:

                                                          > echo $env:Path
In Linux and macOS, use:

                                                          $ echo $PATH
If that’s all correct and Rust still isn’t working, there are a number of places you can get help. Find out how to get in touch with other Rustaceans (a silly nickname we call ourselves) on the community page.


#Updating and Uninstalling
Once Rust is installed via rustup, updating to a newly released version is easy. From your shell, run the following update script:

                                                            $ rustup update
                                                            
To uninstall Rust and rustup, run the following uninstall script from your shell:

                                                             $ rustup self uninstall



