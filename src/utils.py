# utils.py

from ipaddress import IPv4Network

def convert_to_network_ip(ip, mask):
    ip_network = IPv4Network(ip + '/' + str(mask), strict=False)
    return str(ip_network.network_address)
    
def is_same_network(interface1, interface2):
    return convert_to_network_ip(interface1.ip, interface1.mask) == convert_to_network_ip(interface2.ip, interface2.mask)