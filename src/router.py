# router.py

import time
import glob
from interface import Interface
from neighbor import Neighbor
from routing_table import RoutingTable
from config_reader import read_config
from utils import convert_to_network_ip, is_same_network
from route import Route
import threading
import time

class Router:
    def __init__(self, config_file_path):
        config = read_config(config_file_path)
        self.name = config_file_path.split("/")[-1].split(".")[0]
        self.interfaces = []
        # Ajoute les interfaces du routeur présentes dans config.yaml
        for interface_data in config:
            interface = Interface(interface_data['interface']['device'],
                                  interface_data['interface']['ip'],
                                  interface_data['interface']['mask'])
            self.interfaces.append(interface)

        self.neighbors = []
        self.routing_table = RoutingTable(self.interfaces)

        # Paramètre du protocole RIP
        self.route_lifetime = 15  # Durée de vie par défaut d'une route en distance
        self.update_interval = 2  # Intervalle de mise à jour des annonces de route en secondes

    def add_interface(self, interface):
        self.interfaces.append(interface)

    def add_neighbor(self, neighbor_router):
        for interface in self.interfaces:
            for neighbor_interface in neighbor_router.interfaces:
                if is_same_network(interface, neighbor_interface):
                    neighbor = Neighbor(neighbor_router, neighbor_interface, neighbor_router.routing_table)
                    self.neighbors.append(neighbor)
                    return

    def periodic_update(self):
        print(self.__str__())
        while True:
            start_time = time.time()  
            for neighbor in self.neighbors:
                neighbor_routes = neighbor.router.routing_table.routes
                for route in neighbor_routes:
                    existing_route = self.routing_table.get_route(route.network)
                    if existing_route is None:
                        new_route = Route(route.network, neighbor.interface, route.interface, route.distance + 1)
                        self.routing_table.add_route(new_route)
                    else:
                        if route.distance + 1 < existing_route.distance:
                            existing_route.distance = route.distance + 1
                            existing_route.gateway = route.gateway
            print(self.__str__())

            end_time = time.time()  # Temps de fin de l'itération
            elapsed_time = end_time - start_time  # Temps écoulé pendant l'itération
            time.sleep(max(0, self.update_interval - elapsed_time)) # Pause jusqu'à la prochaine itération


    def __str__(self):
        neighbor_str = "Neighbors : " + ", ".join([neighbor.router.name + "@" + neighbor.interface.ip for neighbor in self.neighbors])
        title_str = f"================================ {self.name} ================================"
        return f"{title_str}\nNetwork\t\t\tGateway\t\t\tInterface\t\tDistance\n\n{self.routing_table}\n\n{neighbor_str}\n{'=' * len(title_str)}\n"

# ! TEST !

if __name__ == "__main__":
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

    for router in routers:
        print(router.__str__())