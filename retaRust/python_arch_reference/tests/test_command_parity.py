from __future__ import annotations

import os
import re
import shlex
import subprocess
import sys
import tempfile
import unittest
import zipfile
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
ORIGINAL_ZIP = Path('/mnt/data/reta.todel.zip')
GIT_BASELINE_COMMIT = os.environ.get('RETA_PARITY_BASELINE_COMMIT')


class CommandParityMatrixTest(unittest.TestCase):
    @staticmethod
    def _git_command(*args: str) -> subprocess.CompletedProcess:
        return subprocess.run(
            [
                'git',
                '-c',
                f'safe.directory={REPO_ROOT}',
                '-C',
                str(REPO_ROOT),
                *args,
            ],
            capture_output=True,
            text=True,
            timeout=30,
        )

    @classmethod
    def _oldest_git_commit(cls) -> str | None:
        try:
            proc = cls._git_command('rev-list', '--max-parents=0', 'HEAD')
        except (FileNotFoundError, subprocess.TimeoutExpired):
            return None
        if proc.returncode != 0:
            return None
        for line in proc.stdout.splitlines():
            commit = line.strip()
            if commit:
                return commit
        return None

    @classmethod
    def _resolve_original_archive(cls) -> Path:
        if ORIGINAL_ZIP.exists():
            return ORIGINAL_ZIP
        commit = GIT_BASELINE_COMMIT or cls._oldest_git_commit()
        if not commit:
            raise unittest.SkipTest(
                f'Original-Archiv fehlt: {ORIGINAL_ZIP}; kein Git-Baseline-Commit verfügbar.'
            )
        baseline_zip = cls.temp_root / f'reta.todel.{commit}.zip'
        try:
            proc = cls._git_command(
                'archive',
                '--format=zip',
                '--prefix=reta.todel/',
                commit,
                '-o',
                str(baseline_zip),
            )
        except (FileNotFoundError, subprocess.TimeoutExpired) as exc:
            raise unittest.SkipTest(
                f'Original-Archiv fehlt: {ORIGINAL_ZIP}; Git-Baseline-Export aus Commit {commit} schlug fehl: {exc}'
            )
        if proc.returncode != 0 or not baseline_zip.exists():
            stderr = proc.stderr.strip() or proc.stdout.strip() or 'unbekannter Fehler'
            raise unittest.SkipTest(
                f'Original-Archiv fehlt: {ORIGINAL_ZIP}; Git-Baseline-Export aus Commit {commit} schlug fehl: {stderr}'
            )
        return baseline_zip

    @classmethod
    def setUpClass(cls):
        cls._tempdir = tempfile.TemporaryDirectory(prefix='reta-parity-')
        cls.temp_root = Path(cls._tempdir.name)
        cls.stub_root = cls.temp_root / 'stubs'
        cls.original_root = cls.temp_root / 'original'
        cls.stub_root.mkdir(parents=True, exist_ok=True)
        cls.original_root.mkdir(parents=True, exist_ok=True)
        cls._write_dependency_stubs(cls.stub_root)
        original_archive = cls._resolve_original_archive()
        with zipfile.ZipFile(original_archive) as zf:
            zf.extractall(cls.original_root)
        cls.original_repo = cls.original_root / 'reta.todel'
        if not cls.original_repo.exists():
            raise unittest.SkipTest('Original-Repo konnte nicht extrahiert werden.')

    @classmethod
    def tearDownClass(cls):
        cls._tempdir.cleanup()

    @staticmethod
    def _write_dependency_stubs(root: Path) -> None:
        (root / 'rich').mkdir(parents=True, exist_ok=True)
        (root / 'rich' / '__init__.py').write_text('', encoding='utf-8')
        (root / 'rich' / 'console.py').write_text(
            'class Console:\n'
            '    def __init__(self,*a,**k): pass\n'
            '    def print(self,*args,**kwargs):\n'
            '        end=kwargs.get("end", "\\n")\n'
            '        text=" ".join(str(a) for a in args)\n'
            '        __import__("builtins").print(text, end=end)\n',
            encoding='utf-8',
        )
        (root / 'rich' / 'markdown.py').write_text(
            'class Markdown:\n'
            '    def __init__(self, text, *a, **k): self.text=text\n'
            '    def __str__(self): return self.text\n',
            encoding='utf-8',
        )
        (root / 'rich' / 'syntax.py').write_text(
            'class Syntax:\n'
            '    def __init__(self, text, *a, **k): self.text=text\n'
            '    def __str__(self): return self.text\n',
            encoding='utf-8',
        )
        (root / 'bbcode.py').write_text(
            'def render_html(text,*a,**k):\n    return text\n'
            'class Parser:\n'
            '    def __init__(self,*a,**k): pass\n'
            '    def add_simple_formatter(self,*a,**k): pass\n'
            '    def format(self,text): return text\n',
            encoding='utf-8',
        )
        (root / 'html2text.py').write_text(
            'class HTML2Text:\n'
            '    def __init__(self):\n'
            '        self.ignore_links=False\n'
            '        self.ignore_images=False\n'
            '    def handle(self, text):\n'
            '        return text\n'
            'def html2text(text):\n'
            '    return text\n',
            encoding='utf-8',
        )

        (root / 'textwrap2.py').write_text(
            'from textwrap import fill as _fill\n'
            'def fill(text, width=70, **kwargs):\n'
            '    kwargs.pop("use_hyphenator", None)\n'
            '    return _fill(text, width=width, **kwargs)\n',
            encoding='utf-8',
        )
        (root / 'pyphen.py').write_text(
            'class Pyphen:\n'
            '    def __init__(self,*a,**k): pass\n'
            '    def inserted(self, word): return word\n'
            '    def wrap(self, text, width):\n'
            '        if width <= 0: return [text]\n'
            '        return [text[i:i+width] for i in range(0, len(text), width)] or [""]\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit').mkdir(parents=True, exist_ok=True)
        (root / 'prompt_toolkit' / 'completion').mkdir(parents=True, exist_ok=True)
        (root / 'prompt_toolkit' / 'history').mkdir(parents=True, exist_ok=True)
        (root / 'prompt_toolkit' / 'styles').mkdir(parents=True, exist_ok=True)
        (root / 'prompt_toolkit' / 'formatted_text').mkdir(parents=True, exist_ok=True)
        (root / 'prompt_toolkit' / '__init__.py').write_text(
            'class PromptSession:\n'
            '    def __init__(self,*a,**k): pass\n'
            '    def prompt(self,*a,**k): return ""\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit' / 'completion' / '__init__.py').write_text(
            'class Completion:\n'
            '    def __init__(self,text,start_position=0,display=None,display_meta=None,*a,**k):\n'
            '        self.text=text; self.start_position=start_position; self.display=display; self.display_meta=display_meta\n'
            'class CompleteEvent:\n'
            '    def __init__(self,*a,**k): pass\n'
            'class Completer:\n'
            '    def get_completions(self, document, complete_event): return iter(())\n'
            'class WordCompleter(Completer):\n'
            '    def __init__(self, words=None, *a, **k): self.words=list(words or [])\n'
            '    def get_completions(self, document, complete_event):\n'
            '        for word in self.words: yield Completion(word, start_position=0)\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit' / 'document.py').write_text(
            'class Document:\n'
            '    def __init__(self,text="",cursor_position=None,*a,**k):\n'
            '        self.text=text; self.cursor_position=len(text) if cursor_position is None else cursor_position\n'
            '    @property\n'
            '    def text_before_cursor(self): return self.text[:self.cursor_position]\n'
            '    def get_word_before_cursor(self,*a,**k):\n'
            '        import re\n'
            '        m=re.search(r"\\S+$", self.text_before_cursor)\n'
            '        return m.group(0) if m else ""\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit' / 'history' / '__init__.py').write_text(
            'class FileHistory:\n'
            '    def __init__(self,*a,**k): pass\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit' / 'styles' / '__init__.py').write_text(
            'class Style:\n'
            '    @classmethod\n'
            '    def from_dict(cls,*a,**k): return cls()\n',
            encoding='utf-8',
        )
        (root / 'prompt_toolkit' / 'formatted_text' / '__init__.py').write_text(
            'AnyFormattedText = object\n',
            encoding='utf-8',
        )

    def _run_repo(self, repo_root: Path, command: str):
        env = dict(os.environ)
        existing_pythonpath = env.get('PYTHONPATH', '')
        env['PYTHONPATH'] = (
            str(self.stub_root)
            if not existing_pythonpath
            else str(self.stub_root) + os.pathsep + existing_pythonpath
        )
        env['PYTHONWARNINGS'] = 'ignore'
        proc = subprocess.run(
            [sys.executable, '-S', 'reta.py', *shlex.split(command)],
            cwd=repo_root,
            env=env,
            capture_output=True,
            text=True,
            timeout=300,
        )
        return proc.returncode, proc.stdout, proc.stderr

    @staticmethod
    def _strip_stty_noise(text: str) -> str:
        lines = []
        for line in text.splitlines():
            if line.startswith('stty: '):
                continue
            if line.startswith('Exception ignored in:'):
                continue
            if line.startswith('BrokenPipeError:'):
                continue
            if 'SyntaxWarning:' in line:
                continue
            if 'invalid escape sequence' in line:
                continue
            lines.append(line)
        return '\n'.join(lines).strip()

    @classmethod
    def _normalize_html(cls, text: str) -> str:
        def sort_p4(match):
            values = [item for item in match.group(1).split(',') if item != '']
            try:
                values = [str(v) for v in sorted(int(item) for item in values)]
            except ValueError:
                return match.group(0)
            return 'p4_' + ','.join(values)

        text = re.sub(r'p4_([0-9,]+)', sort_p4, text)
        text = re.sub(r'\s+', ' ', text).strip()
        return text

    def test_representative_command_matrix_matches_original(self):
        cases = [
            (
                'shell-religion-basic',
                '-zeilen --vorhervonausschnitt=1-3 -spalten --religionen=sternpolygon --breite=40',
                'shell',
            ),
            (
                'markdown-religion-basic',
                '-zeilen --vorhervonausschnitt=1-3 -spalten --religionen=sternpolygon --breite=40 -ausgabe --art=markdown',
                'markdown',
            ),
            (
                'html-religion-basic',
                '-zeilen --vorhervonausschnitt=1-3 -spalten --religionen=sternpolygon --breite=20 -ausgabe --art=html',
                'html',
            ),
            (
                'shell-fractional-csv-gluing',
                '-zeilen --vorhervonausschnitt=1-3 -spalten --gebrochenuniversum=5 --breite=40',
                'shell',
            ),
        ]
        for label, command, mode in cases:
            with self.subTest(label=label):
                orig_rc, orig_out, orig_err = self._run_repo(self.original_repo, command)
                new_rc, new_out, new_err = self._run_repo(REPO_ROOT, command)
                self.assertEqual(orig_rc, 0, msg=f'original rc != 0 for {label}: {orig_err}')
                self.assertEqual(new_rc, 0, msg=f'new rc != 0 for {label}: {new_err}')
                self.assertEqual(self._strip_stty_noise(new_err), '')
                self.assertEqual(self._strip_stty_noise(orig_err), '')
                if mode == 'html':
                    self.assertEqual(self._normalize_html(orig_out), self._normalize_html(new_out))
                else:
                    self.assertEqual(orig_out, new_out)
