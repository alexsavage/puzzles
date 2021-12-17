Fold
	n grid,program,step,axis,val
	d load(.grid)
	f  r !,ln q:ln=""  d  
	. s ln=$p(ln," ",$l(ln," ")) 
	. s axis=$p(ln,"=",1),val=$p(ln,"=",2)
	. d fold(.grid,axis,val)
	d print(.grid)
	d count(.grid)
	q
load(grid)
	n ln,x,y
	w !,"Grid input:"
	f  r !,ln q:ln=""  s x=$p(ln,",",1),y=$p(ln,",",2),grid(y,x)=""
	q
fold(grid,axis,val)
	i axis="y" d fold1(.grid,val)
	i axis="x" d fold2(.grid,val)
	q
fold2(grid,two)
	n one,offset
	s one=""
	f  s one=$o(grid(one)) q:one=""  d 
	. s offset=two
	. f  s offset=$o(grid(one,offset)) q:offset=""  s grid(one,two-(offset-two))="" k grid(one,offset)
	q
fold1(grid,one)
	n two,offset
	s offset=one
	f  s offset=$o(grid(offset)) q:offset=""  d
	. s two=""
	. f  s two=$o(grid(offset,two)) q:two=""  s grid(one-(offset-one),two)="" k grid(offset,two)
	q
count(grid)
	n x,y,count
	s (x,y)="",count=0
	f  s y=$o(grid(y)) q:y=""  s x="" f  s x=$o(grid(y,x)) q:x=""  s count=count+1
	w !,"Pixels: ",count
	q
print(grid)
	n y,x
	s y=0
	u $p:nowrap
	f y=0:1:$o(grid(""),-1) d
	. w !
	. f x=0:1:$o(grid(y,""),-1) d
	. . w $s($d(grid(y,x)):$c(9608,9608),1:"  ") ; U+2588: Full Block
	w !
	q