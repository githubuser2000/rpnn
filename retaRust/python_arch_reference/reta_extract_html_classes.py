#!/usr/bin/env python3
# -*- coding: utf-8 -*-
from __future__ import annotations

import json, os, re, subprocess, sys, tempfile
from pathlib import Path
from typing import Dict, List, Tuple


def _repo_root(argv: List[str]) -> Path:
    explicit = Path(argv[2]).resolve() if len(argv) > 2 else None
    candidates = []
    if explicit is not None:
        candidates.append(explicit)
    here = Path(__file__).resolve().parent
    candidates.extend([here, here.parent, Path.cwd(), Path.cwd().parent])
    for base in candidates:
        if (base / 'reta.py').exists() and (base / 'libs' / 'lib4tables.py').exists():
            return base
    raise SystemExit('Konnte reta.py und libs/lib4tables.py nicht finden. Übergib optional den Repo-Pfad als zweites Argument.')


def _stub_dir() -> Path:
    tmp = Path(tempfile.mkdtemp(prefix='reta_html_extract_'))
    (tmp / 'bbcode.py').write_text('# stub\n', encoding='utf-8')
    (tmp / 'html2text.py').write_text('# stub\n', encoding='utf-8')
    (tmp / 'textwrap2.py').write_text('def fill(*a, **k):\n    return a[0] if a else ""\n', encoding='utf-8')
    return tmp


def _run_reta_html(repo_root: Path) -> str:
    stub = _stub_dir()
    env = os.environ.copy()
    env['PYTHONPATH'] = os.pathsep.join([str(stub), str(repo_root / 'libs'), env.get('PYTHONPATH', '')])
    cmd = [sys.executable, 'reta.py', '-zeilen', '--vorhervonausschnitt=1', '-spalten', '--alles', '-ausgabe', '--art=html']
    result = subprocess.run(cmd, cwd=repo_root, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, timeout=300)
    if result.returncode != 0:
        raise SystemExit('reta.py HTML-Extraktion fehlgeschlagen:\n' + result.stderr)
    return result.stdout


def _parse_attrs(open_tag: str) -> List[Tuple[str, str]]:
    return [(m.group(1), m.group(2)) for m in re.finditer(r'([A-Za-z_:][-A-Za-z0-9_:.]*)="([^"]*)"', open_tag)]


def _first_attr_map(attrs: List[Tuple[str, str]]) -> Dict[str, str]:
    out: Dict[str, str] = {}
    for key, value in attrs:
        if key not in out:
            out[key] = value
    return out


def _extract_header_cells(html: str) -> List[Dict[str, object]]:
    match = re.search(r'<tr[^>]*>(.*)</tr>', html, re.S)
    if not match:
        raise SystemExit('Konnte die HTML-Kopfzeile im reta-Output nicht finden.')
    row_html = match.group(1)
    cells: List[Dict[str, object]] = []
    for td_match in re.finditer(r'(<td\b[^>]*>)(.*?)(</td>)', row_html, re.S):
        open_tag, inner_html, close_tag = td_match.groups()
        attrs = _parse_attrs(open_tag)
        first_attrs = _first_attr_map(attrs)
        class_attrs = [value for key, value in attrs if key == 'class']
        primary_class_string = class_attrs[0] if class_attrs else ''
        primary_classes = primary_class_string.split()
        extra_class_strings = class_attrs[1:]
        extra_classes: List[str] = []
        for item in extra_class_strings:
            extra_classes.extend(item.split())
        all_classes = primary_classes + [c for c in extra_classes if c not in primary_classes]
        row_number = None
        column_number = None
        for token in primary_classes:
            if token.startswith('z_') and token[2:].isdigit():
                row_number = int(token[2:])
            if token.startswith('r_') and token[2:].isdigit():
                column_number = int(token[2:])
        if column_number is None:
            column_number = len(cells)
        text = re.sub(r'\s+', ' ', re.sub(r'<[^>]+>', ' ', inner_html)).strip()
        data_attributes = {k: v for k, v in first_attrs.items() if k.startswith('data-')}
        element_attr_map = dict(first_attrs)
        if primary_class_string:
            element_attr_map['class'] = primary_class_string
        cells.append({
            'column_number': column_number,
            'row_number': row_number,
            'tag': 'td',
            'classes': primary_classes,
            'class_string': primary_class_string,
            'class_attributes': class_attrs,
            'extra_class_strings': extra_class_strings,
            'all_classes': all_classes,
            'data_attributes': data_attributes,
            'attributes': attrs,
            'attributes_first': first_attrs,
            'text': text,
            'raw_open_tag': open_tag,
            'raw_html': ''.join((open_tag, inner_html, close_tag)),
            'html_elements': [{
                'tag': 'td',
                'classes': primary_classes,
                'class_string': primary_class_string,
                'attributes': element_attr_map,
                'html': ''.join((open_tag, inner_html, close_tag)),
            }],
        })
    return cells


def main(argv: List[str]) -> int:
    out_path = Path(argv[1]).resolve() if len(argv) > 1 else (Path.cwd() / 'htmlclassesPy.jsonl')
    repo_root = _repo_root(argv)
    html = _run_reta_html(repo_root)
    cells = _extract_header_cells(html)
    with out_path.open('w', encoding='utf-8') as f:
        for cell in cells:
            f.write(json.dumps(cell, ensure_ascii=False, separators=(',', ':')) + '\n')
    print(f'geschrieben: {out_path}')
    print(f'spalten: {len(cells)}')
    return 0

if __name__ == '__main__':
    raise SystemExit(main(sys.argv))
