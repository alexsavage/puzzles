Cave2 ;
	n caves,location,next,stack,depth,seen,solution
	d load(.caves)
	s location="start",next=""
	s depth=0,stack(0)="start"
	s solution=0
	w !,"Solving"
	f  d  q:depth<0
	. s location=stack(depth)
	. i next'="" k stack(depth,"seen",next),stack(depth,"seenTwice")
	. s next=$o(caves(location,next))
	. i next="" d  QUIT
	. . s next=location
	. . k stack(depth)
	. . s depth=depth-1 q:depth<0
	. . s location=stack(depth)
	. i next="end" d  QUIT
	. . s solution=solution+1 
	. . i solution#1000=0 w "." 
	. . i solution#10000=0 w solution/10000
	. i $d(stack(depth-1,"seen")) m stack(depth,"seen")=stack(depth-1,"seen") 
	. i $d(stack(depth-1,"seenTwice")) s stack(depth,"seenTwice")=stack(depth-1,"seenTwice") 
	. i $d(stack(depth,"seen",next)),$d(stack(depth,"seenTwice")) QUIT
	. i next?1.L d  
	. . i $d(stack(depth,"seen",next)) s stack(depth,"seenTwice")=next 
	. . e  s stack(depth,"seen",next)="" 
	. s stack($i(depth))=next,next=""
	w !!,"Solutions: ",solution
	q
load(caves) ;
	n line,a,b
	w #,"Puzzle input:"
	f  r !,line q:line=""  d
	. s a=$p(line,"-",1),b=$p(line,"-",2)
	. s:(b'="start")&(a'="end") caves(a,b)=""
	. s:(a'="start")&(b'="end") caves(b,a)=""
	q