import builtins

class Console:
    def print(self, *args, **kwargs):
        builtins.print(*args, **{k: v for k, v in kwargs.items() if k in {"sep", "end", "file", "flush"}})
