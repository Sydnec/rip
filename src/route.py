# route.py

class Route:
    def __init__(self, network, gateway, interface, distance):
        self.network = network
        self.gateway = gateway
        self.interface = interface
        self.distance = distance

    def __str__(self):
        if self.gateway is None:
            gateway_str = "None\t"
        else:
            gateway_str = self.gateway.ip
        return f"{self.network}\t\t{gateway_str}\t\t{self.interface.ip}\t\t{self.distance}"
