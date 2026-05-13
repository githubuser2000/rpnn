"""Lightweight local fallback for environments without the external bbcode package."""

class Parser:
    def format(self, text):
        return text


def render_html(text):
    return text
