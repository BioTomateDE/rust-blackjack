## Usage Guide (Windows)
1. On the right side, click on **Releases**.
2. Download the latest release `.exe` file.
3. Press `Win + R`.
4. Type `powershell.exe` and hit enter.
5. Type `cd ~/Downloads` or wherever you downloaded the file to.
6. Type `./blackjack` and hit enter.

## Usage Guide (Linux)
1. On the right side, click on **Releases**.
2. Download the latest release binary.
3. Open a terminal emulator and navigate to the downloaded file's directory.
4. Run the binary: `./blackjack`.

## How to download (other platforms)
1. Open a command prompt and navigate to some temporary folder.
2. Clone this repository: `git clone https://github.com/BioTomateDE/rust-blackjack`.
3. Navigate into the cloned repository: `cd ./rust-blackjack`.
4. Install [Cargo](https://www.rust-lang.org/tools/install) if you haven't already.
5. Build the program: `cargo b -r`.
6. The binary will be located in `./target/release/blackjack`.
7. Run the built program.

## Options
You can set the `BJ_SLEEP` environment variable to `disabled`
if you want to disable the sleeps between actions.

This is also achievable by running the binary with 
the `nosleep` flag (literally just `./blackjack nosleep`).

For betting amount inputs, you can input one of the following:
- a number of dollars.
- `all` to bet all of your money.
- `half` to bet half of your money (rounded down if not even).
- `idk` to choose a random amount of money (more gambling per gambling 🤑).
