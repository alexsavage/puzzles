OCTO
	n map,total,line,n,x,y
	s total=0
	w !,"Puzzle input:",!
	f y=1:1:10 r line f x=1:1:10 s map(x,y)=$e(line,x)
	f n=1:1:100 s total=total+$$step(.map)
	w !,"Total flashes after ",n," steps: ",total
	q
step(map)
	n x,y,flash,n
	s n=0
	f y=1:1:10 f x=1:1:10 s n=n+$$increment(.map,.flash,x,y)
	s x="",y=""
	f  s x=$o(flash(x)) q:x=""  d 
	. f  s y=$o(flash(x,y)) q:y=""  d 
	. . s map(x,y)=0
	q n
increment(map,flash,x,y)
	n n
	s n=0
	i $d(map(x,y)) d
	. s map(x,y)=map(x,y)+1
	. i $d(flash(x,y)) q
	. i map(x,y)>9 d  
	. . s n=n+1
	. . s flash(x,y)=1
	. . s n=n+$$increment(.map,.flash,x-1,y-1)
	. . s n=n+$$increment(.map,.flash,x,y-1)
	. . s n=n+$$increment(.map,.flash,x+1,y-1)
	. . s n=n+$$increment(.map,.flash,x-1,y)
	. . s n=n+$$increment(.map,.flash,x+1,y)
	. . s n=n+$$increment(.map,.flash,x-1,y+1)
	. . s n=n+$$increment(.map,.flash,x,y+1)
	. . s n=n+$$increment(.map,.flash,x+1,y+1)
	q n
	q
	;