import glob
import threading
from router import Router
import time

def main():
    config_directory = "config/*.yaml"
    yaml_files = sorted(glob.glob(config_directory))
    routers = []

    # Créer une instance de chaque routeur
    for config_file_path in yaml_files:
        router = Router(config_file_path)
        routers.append(router)
    
    # Définir les voisins pour chaque routeur en comparant les interfaces
    for i, router1 in enumerate(routers):
        for j, router2 in enumerate(routers):
            if i != j:  # Éviter de comparer le même routeur
                router1.add_neighbor(router2)

    # Démarrer les threads
    router_threads = []
    for router in routers:
        time.sleep(0.01)
        router_thread = threading.Thread(target=router.periodic_update)
        router_thread.daemon = True
        router_thread.start()
        router_threads.append(router_thread)
                
    # Attend que tous les threads de routeur se terminent
    for router_thread in router_threads:
        router_thread.join()

if __name__ == "__main__":
    main()
