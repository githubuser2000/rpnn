"""Lightweight local fallback for environments without the external html2text package."""

class HTML2Text:
    def handle(self, text):
        return text


def html2text(text):
    return text
