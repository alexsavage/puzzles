PUZZLE
	n X,Y,Map,Value,Risk
	s Risk=0
	w !,"Input (100x100):",!
	f Y=1:1:100 r Line f X=1:1:100 s Map(X,Y)=$e(Line,X)
	f X=1:1:100 f Y=1:1:100 DO
	. s Value=Map(X,Y)
	. q:'$$Compare(.Map,Value,X-1,Y)
	. q:'$$Compare(.Map,Value,X+1,Y)
	. q:'$$Compare(.Map,Value,X,Y-1)
	. q:'$$Compare(.Map,Value,X,Y+1)
	. s Risk=Risk+Value+1 
	w !,"Risk: ",Risk
	q
Compare(Map,Value,X,Y)
	i '$d(Map(X,Y)) q 1 
	q (Map(X,Y)>Value)
	q
	;