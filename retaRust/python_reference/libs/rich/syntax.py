class Syntax(str):
    def __new__(cls, code, *args, **kwargs):
        return str.__new__(cls, code)
