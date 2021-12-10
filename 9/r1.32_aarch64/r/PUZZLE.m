PUZZLE
	GOTO PartOne
	; AUTHOR: Alex Savage
	; PURPOSE: Solve puzzle
PartOne
	NEW X,Y,Value,NeighborX,NeighborY,NeighborVal,Risk
	SET Risk=0
	FOR X=1:1:100 FOR Y=1:1:100 DO
	. SET Value=^MAP(X,Y)
	. QUIT:'$$Compare(Value,X-1,Y)
	. QUIT:'$$Compare(Value,X+1,Y)
	. QUIT:'$$Compare(Value,X,Y-1)
	. QUIT:'$$Compare(Value,X,Y+1)
	. SET Risk=Risk+Value+1 
	WRITE Risk
	QUIT
Compare(Value,X,Y)
	IF '$DATA(^MAP(X,Y)) QUIT 1 
	QUIT (^MAP(X,Y)>Value)
PartTwo
	;	
Load(File)
	NEW X,Y,Line
	OPEN File:READONLY
	USE File
	FOR X=1:1:100 READ Line FOR Y=1:1:100 SET ^MAP(X,Y)=$EXTRACT(Line,Y)
	USE 0
	CLOSE File
	QUIT
	QUIT
	;