# config_reader.py

import glob
import yaml

def read_config(file_path):
    with open(file_path, 'r') as file:
        config = yaml.safe_load(file)
    return config

# ! TEST !
if __name__ == "__main__":
    # Chemin du répertoire contenant les fichiers YAML
    config_directory = "config/*.yaml"

    # Récupérer la liste des fichiers YAML dans le répertoire et les trier par ordre alphabétique
    yaml_files = sorted(glob.glob(config_directory))

    # Lire chaque fichier de configuration dans l'ordre alphabétique
    for config_file_path in yaml_files:
        config = read_config(config_file_path)

        # Affichage du contenu de la configuration
        print(config_file_path + " : ")
        print(config)