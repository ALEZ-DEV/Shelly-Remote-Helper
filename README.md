# Shelly-Remote-Helper
***
This utilitary can upload JS files that you are editing on your local machine to your shelly and get logs via websocket, all in realtime

## Compilation

### Require
To use it, you need to install Rust, if is not already installed, you can install it with [Rustup](https://www.rust-lang.org/tools/install)

Then choose the complete installation, this will install [Cargo](https://github.com/rust-lang/cargo), the packet manager of Rust

One time this is installed, you can go to the next section

### Compilation

Clone the project in any directory
```bash
git clone https://github.com/ALEZ-DEV/Shelly-Remote-Helper.git
```

Change workspace
```bash
cd Shelly-Remote-helper
```

Compile and Run the project
```bash
cargo run
```

## Setup (VS Code)
Setup your workspace on Visual Studio Code (the setup process have to be similare in other text editor/IDE)  
To download it, you can simply go to the [release page](https://github.com/ALEZ-DEV/Shelly-Remote-Helper/releases) of this repo, then add it to your `./.vscode` directory on your VS code project

### Setup automatique

The utilitary will setup automatically the task.json for you, start the project where your shelly script is, then in the cmd of VS code start this two command
```shell
cd .\.vscode
.\Shelly_Remote_Helper.exe --host <IP du Shelly> --password <mot de passe> setup --vs-code
```

then for enable the ``Tasks`` :

1. Press on ``CTRL`` + ``SHIFT`` + ``P`` and type ``> Tasks: Manage Automatic Tasks in Folder``
2. Then choose ``Allow Automcatic Tasks in Folder``

When you restart your next session of this **Workspace**, the utilitary will auto start

then restart your VS code and will run automatically

if you want anyway to configure it manually, follow these steps

### Setup manually (tasks.json)
Create your **Workspace** where you will create the file ``./.vscode/tasks.json`` and add this configuration to it :

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Shelly Remote Helper",
            "type": "shell",
            "command": "${workspaceFolder}/.vscode/shelly_remote_helper.exe --path ${workspaceFolder} --host <ip du shelly a debug> --password <mot de passe du shelly>",
            "group": "none",
            "presentation": {
                "reveal": "always",
                "panel": "new"
            },
            "runOptions": {
                "runOn": "folderOpen"
            }
        }
    ]
}
```
then for enable the ``Tasks`` :

1. Press on ``CTRL`` + ``SHIFT`` + ``P`` and type ``> Tasks: Manage Automatic Tasks in Folder``
2. Then choose ``Allow Automcatic Tasks in Folder``

When you restart your next session of this **Workspace**, the utilitary will auto start

## Utile Function (Only work if you use the utilitary)
`stopCurrentScript()`
if you call this function in your code, the executed script will stop itself

```javascript
Shelly.call(
    "Shelly.SomeFunction",
    {},
    function (result, userdata) {
        print(result);
        // execute other code
        stopCurrentScript();
    },
);
```

## Commands
There is all the available commands

#### Global parameter

``--host``  
IP of the Shelly, the utilitary will use it to connect to it
Mandatory*

``--username``  
The usename of the user in the Shelly, the utilitary will use it to connect to it
default value: ``admin``

``--password``  
The password that will be used to connecect to the Shelly
Mandatory*

``--log``  
The level log, often usedwhile in development and to debug the utilitary
can be: ``info``, ``error``, ``debug``, ``all``  
Default value: ``info``

``help``  
Display all the available commands with their description

#### Debug

``debug``  
Start the remote debugger for the shelly

``--path``  
The directory where the utilitary will check edited files (to upload to the Shelly)
can be: ``path/to/the/directory/to/check``

``--ws-port``  
The port which the webscoket will use to get the logs from the Shelly (generally, you don't have to edit this parameter
Default value: ``80``

#### Start

``start``  
Start one script in the Shelly by it's name
can be: ``nom du script``

#### Setup

``setup``  
Setup the configuration to use the utilitary in VS code (other configuration can be added on demand or merge request)
Mandatory to specify the one of the next argument 

``--vs-code``  
Will create the configuration for VS code
