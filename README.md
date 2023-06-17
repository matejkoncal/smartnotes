
# SmartNotes

smartnotes is a tool that utilizes the OpenAI API to enhance your notes. It acts as a daemon, continuously monitoring a specified folder. Whenever a file is added or modified within the folder, smartnotes retrieves it, sends it to the OpenAI API for processing, and saves the resulting modified file.



## Prerequisites

Before using smartnotes, make sure you have the following:

- Rust installed on your system. You can install Rust by following the instructions at https://www.rust-lang.org/tools/install.
- An OpenAI API key. Set the OPENAI_API_KEY environment variable to your API key.
- The desired folder path to monitor for note updates.
- (Optional) Set the SMARTNOTES_OUT environment variable to specify the destination path for the modified files. The default path is ~/notes/
## Installation

1. Clone the smartnotes repository:

```bash
git clone https://github.com/your-username/smartnotes.git
```
2. Change into the smartnotes directory:

```bash
cd smartnotes
```

3. Build the project:

```bash
cargo build --release
```
This command will compile the project and generate the executable binary file in the target/release directory.
## Usage
```bash
target/release/smartnotes /path/to/folder
```
The tool will start running as a daemon and continuously watch the specified folder. When a file is added or modified, smartnotes will send it to the OpenAI API for processing. The resulting modified file will be saved in the ~/notes/ directory (or the path specified by the SMARTNOTES_OUT environment variable).

If you want to add any additional instructions for AI, write them in a line starting with @ai.
## Stopping SmartNotes
To stop the smartnotes tool, you can use the following command in the Fish shell:

```bash
kill (pgrep smartnotes)
```
For Bash, use the following command:
```bash
kill $(pgrep smartnotes)
```
This will terminate the smartnotes daemon process.
