#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""Legacy compatibility facade for the table-tag schema.

The actual tag/enum ownership lives in ``reta_architecture.tag_schema``.
This module intentionally re-exports the historical public names so existing
imports keep working.
"""

from reta_architecture.tag_schema import (
    ST,
    TagSchemaBundle,
    bootstrap_tag_schema,
    dictViceversa,
    tableTags,
    tableTags2,
    tableTags_kombiTable,
    tableTags2_kombiTable,
    tableTags_kombiTable2,
    tableTags2_kombiTable2,
)

TAG_SCHEMA = bootstrap_tag_schema()

__all__ = [
    "ST",
    "TagSchemaBundle",
    "TAG_SCHEMA",
    "bootstrap_tag_schema",
    "dictViceversa",
    "tableTags",
    "tableTags2",
    "tableTags_kombiTable",
    "tableTags2_kombiTable",
    "tableTags_kombiTable2",
    "tableTags2_kombiTable2",
]
