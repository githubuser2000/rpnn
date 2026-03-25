echo '' > messages.po # xgettext needs that file, and we need it empty
find . -type f -iname "*.py" | xgettext -j -f - # this modifies messages.po
msgmerge -N existing.po messages.po > new.po
mv new.po existing.po
rm messages.po
