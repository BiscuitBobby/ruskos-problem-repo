# RuskOS

A kernel is the core program around which an operating system is built. Utilising a custom inhouse kernel can mitigate potential telemetry and spyware built into an operating system such as Windows.

Tanya Von Degourachaff was delivering a package on behalf of Doktor Wilhelm Voigt that contained such a kernel. It was a little program called the Kreschnder cipher, A kernel written in Rust that contained a unique text pattern required to activate their new state-of-the-art aerial defence system.

Unfortunately, the kernel sustained some damage due to an enemy ambush on the way to the destination. Now it’s up to you, a field engineer Dingus to restore the kernel before shipping it back out.

## Objectives:
- Resolve the syntax errors
- Change the background color to black and text color to yellow
- Correct the keyboard port address to correct port address for the x86 architecture
- Correct the inverted text input

## Expected output:
![output](https://github.com/BiscuitBobby/ruskos-problem-repo/blob/main/output.gif)

## Requirements:
- Rust nightly
- Qemu

## Some pointers:
- The source code is located at the src/ directory
- The bulk of the incorrect code is primarily located at the following files:
  - main.rs
  - interrupts.rs
  - vga_buffer.rs
- The passcode is "amfoss"

## How to Start:
This guide will walk you through the setup process of the RuskOS kernel. Follow the steps below to install Rust (nightly), QEMU, and other necessary tools.

#### Step 1: Install Rust and Rustup

First, we need to install Rust and Rustup, the toolchain installer for Rust.

1. Open a terminal.
2. Install Rust and Rustup by running the following command:
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

#### Step 2: Install Rust Nightly

1. RuskOS requires the nightly version of Rust. Install it with the following commands:

    ```sh
    rustup toolchain install nightly
    ```

2. Set the default toolchain to nightly:

    ```sh
    rustup default nightly
    ```
#### Step 3: Install QEMU

1. QEMU is needed to emulate the hardware for running the RuskOS kernel.
- Update your package list:
  - On Ubuntu/Debian:
    ```sh
    sudo apt update
      ```
  - On Fedora:
    ```sh
    sudo dnf update
      ```
  - Install QEMU and related packages:

    - Debian/Ubuntu:
     ```sh
    sudo apt-get install qemu-system
      ```
  - Fedora:
    ```sh
    sudo dnf install @virtualization
    ```
    - MacOs:
      1. Install Homebrew if you haven't already:
         ```sh
         /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
         ```
      2. Install QEMU:
          ```sh
          brew install qemu
          ```
#### Step 4: Running the app

1. Clone the RuskOS Repository
   Clone the RuskOS repository to your local machine:
    ```sh
        git clone https://github.com/sabarixr/Rusk_OS.git
    ```
2. Navigate to the project directory:
    ```sh
        cd Rusk_OS
    ```
3. Install Bootimage:

   Bootimage is a tool that creates bootable disk images from your Rust kernel.
- Install Bootimage by running:
     ```sh
     cargo install bootimage
     ```

4. Add LLVM Tools Preview Component:

   LLVM tools are required for building the kernel.
- Add the LLVM tools component:
  ```sh
  rustup component add llvm-tools-preview
  ```

5. Build and Run the Kernel
- Build the kernel:
  ```sh
  cargo +nightly build -Z build-std --target x86_64-rusk.json
  ```
6. Run the kernel using QEMU:
    ```sh
    cargo run
    ```
## 📚 Resources: 
- <a href="https://www.geeksforgeeks.org/kernel-in-operating-system/">What is a Kernel?</a>
- <a href="https://doc.rust-lang.org/book/ch01-00-getting-started.html">Getting started with Rust</a>
- <a href="https://os.phil-opp.com/minimal-rust-kernel/">Making a kernel in Rust</a>
  

