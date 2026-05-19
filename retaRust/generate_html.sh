#!/bin/sh
cat head1.alx
cat religionen.js
cat head2.alx
rreta -spalten --alles --breite=0 -ausgabe --art=html --onetable --nocolor
rgrundStrukHtml blank
cat footer.alx
