# Shelly-Remote-Helper
***
Cette utilitaire permet de upload les fichiers javascripts que vous modifier directement au Shelly que vous voulez
Il permet aussi de voir les logs du Shelly en temp réel

## Setup (VS Code)
***
Setup votre environement de travail sur Visual Studio Code (doit être assez similaire dans d'autre éditeur de texte/IDE)  
Pour télécharger l'éxecutable, allez sur la [release page](https://gitlab.ptm.net/ptm/utilitaires/Shelly_Remote_Helper/-/releases)

### tasks.json
Créer votre **Workspace** ou vous allez créer le le fichier ``./.vscode/tasks.json`` et ajouter cette configuration à celui-ci :

```
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
puis pour activer les ``Tasks`` :

1. Appuyer sur ``CTRL `` + ``SHIFT `` + ``P`` et taper ``> Tasks: Manage Automatic Tasks in Folder``
2. Puis choisisser ``Allow Automatic Tasks in Folder``

Quand vous allez redémarrez la prochaine session dans ce **Workspace**, l'utilitaire démarrera automatiquement

## Paramètre
***
Voici toute les commandes disponibles  

``--help``  
Affiche toute les comandes disponibles avec leurs description  

``--path``  
Le dossier ou l'utilitaire va vérifier pour les fichiers éditier (à upload au  Shelly)  
peut être: ``chemin d'accèes``

``--host``  
L'IP du Shelly, l'utilitaire l'utilisera pour s'y connecter  
valeur par défaut: ``127.0.0.1``

``--ws-port``  
Le port que le websocket utilisera pour récupérer les Logs du Shelly  (généralement vous n'avez pas besoin de modifier ce paramètre)
valeur par défaut: ``80``  

``--username``  
Le nom d'utiliserateur que l'utilitaire utilisera pour se connecter au Shelly  
valeur par défaut: ``admin``  

``--password``  
Le mot de passe qui va être utiliser pour se connecter au Shelly  
valeur par défaut: ``123456``  

``--log``  
Le niveau de Log, souvent utiliser pendant le développement et le débuggage de l'utilitaire  
peut être: ``info``, ``error``, ``debug``, ``all``  
valeur par défaut: ``info``

