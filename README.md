<h2 style="text-align:center"> Codesetup </h2>
<p style="text-align:center"> A simple tool for configuring a Cortex based project.</p>

## About ##
It's a CLI tool to ease the creation of an Arm Cortex based project for VS Code.

## How to use ##
To set up a project for VS Code just use:

> `codesetup init`


## Custom settings ##
In your home directory should be a folder named ".codesetup". In this directory, you will find the defaults.json file. If you have custom settings, you can add it here. Just copy the "generic" settings and add a comma in between. Now you can use 

> `codesetup init "your_settings_name`

If you have messed up the settings, just revert it with: </br>
WARNING: this command deletes all other settings. 

> `codesetup install`

## How to setup ##
### Linux ###
1. Download "codesetup" from the /app/linux directory
2. Copy it to your application directory. For example: "sudo cp ./codesetup /usr/bin/codesetup"
3. Restart your terminal
4. Run "codesetup install"

### Windows ###
1. Download "codesetup.exe" from the /app/windows directory
2. Create a new folder called codesetup in your program files directory. Something like: "C:\Program Files (x86)\codesetup"
3. Copy it to this application folder. 
4. Add Codesetup to your environment variables:
   1. Start a new Terminal
   2. Type "sysdm.cpl" and run it.
   3. Under "Advanced" click "Environment Variables..."
   4. Under "System variables", double-click on "Path"
   5. Add a new path containing your newly created folder: "C:\Program Files (x86)\codesetup"
5. Restart your terminal
6. Run "codesetup install"