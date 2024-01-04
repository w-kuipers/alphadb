class UnexpectedError(Exception):
    msg = "An unexpected error has occured."
    def __init__(self):
        super().__init__(self, self.msg)

class ConfigIncoplete(Exception):
    msg = "Config seems to be incomplete. The config file might be broken or edited."
    def __init__(self):
        super().__init__(self, self.msg)
