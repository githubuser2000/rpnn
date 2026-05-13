#!/bin/bash
SCRIPT_PATH="${BASH_SOURCE:-$0}"
ABS_SCRIPT_PATH="$(realpath "${SCRIPT_PATH}")"
ABS_DIRECTORY="$(dirname "${ABS_SCRIPT_PATH}")"
GIT_DIRECTORY="${ABS_DIRECTORY}/.git"
${ABS_DIRECTORY}/libs/changeVersion.sh
if [ "$5" == '' ]; then
	commitstring="$(date)"
else
	commitstring="${5}"
fi
if [ "$1" == 'reta' ] || [ "$2" == 'reta' ]  || [ "$3" == 'reta' ] || [ "$4" == 'reta' ]; then
	bla=reta
fi
if [ "$1" == 'snapshot' ] || [ "$2" == 'snapshot' ]  || [ "$3" == 'snapshot' ] || [ "$4" == 'snapshot' ]; then
	reta-all2-snapshot.sh $bla > ~/religionen.html
else
	geschafft=false
	pypy-reta-all2.sh $bla > ~/religionen_.html
	[ `cat ~/religionen_.html | grep -e '^</table>$' | grep -v table2 | wc -l` -eq 1 ] && geschafft=true
	$geschafft && cp -av ~/religionen_.html ~/religionen.html
fi
#$geschafft && cp ~/religionen.html ${ABS_DIRECTORY}/religionen.html
if [ "$1" == 'htmld2' ] || [ "$2" == 'htmld2' ] || [ "$3" == 'htmld2' ] || [ "$4" == 'htmld2' ]; then
	if $geschafft; then
		cd ~;tar -c religionen.html | plzip -1 - | ssh root@d2 'plzip -d - | tar --overwrite -xf - -C /media/2TB/data/www/forum/' && echo html zu d2 gesendet || echo html konnte nicht zu d2 gesendet werden
		tar -c religionen.html | ssh root@ppp 'tar --overwrite -xf - -C /srv/http/forum' && echo html zu ppp gesendet || echo html konnte nicht zu ppp gesendet werden
		cd -
	fi
fi
if [ "$1" == 'tar' ] || [ "$2" == 'tar' ] || [ "$3" == 'tar' ] || [ "$4" == 'tar' ]; then
	echo sende auch tar
	cd ${ABS_DIRECTORY}
	cp -ax {*.{py,js,sh,ts,toml,md},head*.alx,foot*.alx,retaPrompt,reta,rp,rpl,rpb,rpe,math,modulo,prim,prim24,generate_html,i18n,*.english,doc,csv,run,libs} /home/alex/myRepos/religions-tabelle-releasses/31
	#cp -ax pypy-reta-all2-to-d2.sh /home/alex/myRepos/religions-tabelle-releasses/31
	#cp -ax pypy-reta-all2.sh /home/alex/myRepos/religions-tabelle-releasses/31
	cd -
	cd /home/alex/myRepos/religions-tabelle-releasses/31
	tar --exclude=__pycache__ --exclude=.mypy_cache --exclude=readme.org -c {*.{py,js,sh,ts,toml,md},head*.alx,foot*.alx,retaPrompt,reta,rp,rpl,rpb,rpe,math,modulo,prim,prim24,generate_html,i18n,*.english,doc,csv,run,libs} > /home/alex/myRepos/religions-tabelle-releasses/reta.tar
	tar --exclude=__pycache__ --exclude=.mypy_cache --exclude=readme.org -c {*.{py,js,sh,ts,toml,md},head*.alx,foot*.alx,retaPrompt,reta,rp,rpl,rpb,rpe,math,modulo,prim,prim24,generate_html,i18n,*.english,doc,csv,run,libs} | xz - --threads=16 -9 --lzma2=dict=256MiB -c - > ~/reta.tar.xz
    cat ~/religionen.html | xz - --threads=16 -9 --lzma2=dict=256MiB -c - > ~/religionen.html.xz
    tar -c ~/religionen.html > ~/religionen.html.tar
    cd -
	cd /home/alex/myRepos/religions-tabelle-releasses/; tar -c reta.tar | plzip -1 - | ssh root@d2 'plzip -d - | tar --overwrite -xf - -C /media/2TB/data/www/forum/'
	cd /home/alex/myRepos/religions-tabelle-releasses/; tar -c reta.tar | ssh root@ppp 'tar --overwrite -xf - -C /srv/http/forum'
    SCRIPT_PATH=/home/alex/myRepos/religions-tabelle-releasses/
    ABS_SCRIPT_PATH="$(realpath "${SCRIPT_PATH}")"
    ABS_DIRECTORY=/home/alex/myRepos/religions-tabelle-releasses
    GIT_DIRECTORY="${ABS_DIRECTORY}/.git"
	git --git-dir ${GIT_DIRECTORY} --work-tree=${ABS_DIRECTORY} add -A;git --git-dir ${GIT_DIRECTORY} --work-tree=${ABS_DIRECTORY} commit -m "${commitstring}";git --git-dir ${GIT_DIRECTORY} --work-tree=${ABS_DIRECTORY} push;git --git-dir ${GIT_DIRECTORY} --work-tree=${ABS_DIRECTORY} pushall
	cd -
fi
