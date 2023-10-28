# KProtonEntry

`KProtonEntry` is a utility to run commands within the Proton environment on Steam. It uses an environment variable `kprotonCommand` to determine which command to execute using the Windows Command Prompt (`cmd.exe`).

## Usage

1. Rename KProtonEntry.exe to a descriptive name that describes which proton compatdata folder it will help you enter into.

2. Add `KProtonEntry.exe` as a non-Steam game to your Steam library.

   - Steam Library > "Add a Game" > Add a non-steam game > select the `KProtonEntry.exe`.

3. Right-click on `KProtonEntry.exe` in your Steam library, choose 'Properties', and in the 'Launch Options', set the desired command to be executed in the format:

   ```
   kprotonCommand="your_command_here" %command%
   ```

4. Select a proton version in the Compatibility settings under properties.

5. Launch `KProtonEntry.exe` from Steam. The utility will fetch the command from the `kprotonCommand` environment variable and execute it using `cmd.exe`.
