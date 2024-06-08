# routing_table.py

from route import Route
from ipaddress import IPv4Network
from utils import convert_to_network_ip

class RoutingTable:
    def __init__(self, interfaces):
        self.routes = []
        self.init_routing_table(interfaces)

    def add_route(self, route):
        self.routes.append(route)

    def init_routing_table(self, interfaces):
        for interface in interfaces:
            interface_network = convert_to_network_ip(interface.ip, interface.mask)
            network = interface_network + '/' + str(interface.mask)
            route = Route(network, None, interface, 1)
            self.add_route(route)

    def get_route(self, network):
        for route in self.routes:
            if route.network == network:
                return route
        return None

    def __str__(self):
        route_str = "\n".join([str(route) for route in self.routes])
        return f"{route_str}"
