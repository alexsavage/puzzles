Poly
	n template,rules,count
	d read(.template,.rules)
	r !,"Count: ",count
	f %=1:1:count s template=$$apply(template,.rules)
	u $p:nowrap
	w !,template
	d analyze(template)
	q
apply(template,rules)
	n pos,old,new
	s new=""
	f pos=1:1:$l(template) d
	. s old=$e(template,pos,pos+1)
	. s new=new_$e(old,1)_$g(rules(old))
	q new
analyze(template)
	n i,count,rank
	f i=1:1:$l(template) s %=$i(count($e(template,i)))
	s i=""
	f  s i=$o(count(i)) q:i=""  s rank(count(i),i)=""
	s i=$o(rank(""),-1)
	w !,"Most frequent: ",$o(rank(i,""))," (",i,")"
	s i=$o(rank(""))
	w !,"Least frequent: ",$o(rank(i,""))," (",i,")"
	w !,"Difference: ",$o(rank(""),-1)-$o(rank(""))
	q
read(template,rules)
	n ln,a,b
	w !,"Puzzle input:"
	r !,template
	r !,%
	f  r !,ln q:ln=""  d
	. s a=$p(ln," ",1)
	. s b=$p(ln," ",3)
	. s rules(a)=b
	q