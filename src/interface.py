# interface.py

class Interface:
    def __init__(self, device, ip, mask):
        self.device = device
        self.ip = ip
        self.mask = mask

    def __str__(self):
        return f"[{self.device}, IP: {self.ip}, Mask: {self.mask}]"
