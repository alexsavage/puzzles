OCTO
Puzzle1
	n map,total
	s total=0
	d read(.map)
	f %=1:1:100 s total=total+$$step(.map)
	w !,"Total flashes after 100 steps: ",total
	q
Puzzle2
	n map,step
	s step=0
	d read(.map)
	w !
	f  s step=step+1,%=$$step(.map) q:$$full(.map)  
	w !!,"Full at step ",step
	q
read(map)
	n x,y,line
	w !,"Puzzle input:",!
	f y=1:1:10 r line f x=1:1:10 s map(x,y)=$e(line,x)
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
full(map)
	n x,y,full
	s full=1
	f y=1:1:10 d  q:'full
	. f x=1:1:10 d  q:'full
	. . i map(x,y)>0 s full=0
	q full
	q
	;