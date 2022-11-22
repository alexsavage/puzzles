Cave ;
	n caves,location,next,stack,depth,seen,solution
	d load(.caves)
	s location="start",next=""
	s depth=0,stack(0)="start"
	s solution=0
	w !,"Solving"
	f  d  q:depth<0
	. s location=stack(depth)
	. i next'="" k stack(depth,"seen",next)
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
	. q:$d(stack(depth,"seen",next))
	. i next?1.L s stack(depth,"seen",next)=""
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