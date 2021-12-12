BASIN
	n x,y,map,depth,risk,basinMap,basinID,basinSize,answer
	s risk=0
	w !,"Input (100x100):",!
	f y=1:1:100 r Line f x=1:1:100 s map(x,y)=$e(Line,x)
	; Part 1
	f y=1:1:100 f x=1:1:100 d
	. s depth=map(x,y)
	. q:'$$below(.map,depth,x-1,y)
	. q:'$$below(.map,depth,x+1,y)
	. q:'$$below(.map,depth,x,y-1)
	. q:'$$below(.map,depth,x,y+1)
	. s risk=risk+depth+1 
	w !,"Risk: ",risk
	; Part 2
	s nextBasinID=0
	f y=1:1:100 f x=1:1:100 d fill(x,y,.map,.basinMap,.basinSize,$i(nextBasinID))
	s basinID=""
	f  s basinID=$o(basinSize(basinID)) q:basinID=""  d  
	. s %=$i(answer(basinSize(basinID)))
	s basinSize="",answer=1
	f x=1:1:3 s basinSize=$o(answer(basinSize),-1) s answer=answer*basinSize w !,"Basin "_x,": ",basinSize
	w !,"Solution 2: ",answer,!
	q
	;
below(map,depth,x,y) q ('$d(map(x,y)))!(map(x,y)>depth)
		;
fill(x,y,map,basinMap,basinSize,basinID)
	n %,level
	s level=1,x(1)=x,y(1)=y
fillLoop ; M stack runs out fast, do recursion with GOTO
	s x=x(level),y=y(level)
	s level=level-1
	i '$d(map(x,y)) g fillDone
	i $d(basinMap(x,y)) g fillDone
	i map(x,y)=9 s basinMap(x,y)=0 g fillDone
	s basinMap(x,y)=basinID
	s %=$i(basinSize(basinID))
	d pushFill(.level,.x,.y,x-1,y)
	d pushFill(.level,.x,.y,x+1,y)
	d pushFill(.level,.x,.y,x,y-1)
	d pushFill(.level,.x,.y,x,y+1)
fillDone
	i level>0 g fillLoop	
	q
pushFill(level,xStack,yStack,x,y)
	s level=level+1
	s xStack(level)=x,yStack(level)=y
	q
	q 
	;