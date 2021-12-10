PUZZLE
 ; AUTHOR: Alex Savage
 ; PURPOSE: Solve puzzle
 NEW X,Y,Value,NeighborX,NeighborY,NeighborVal
 FOR X=1:1:100 FOR Y=1:1:100 DO
 . SET Value=^AOC9(X,Y)
 . QUIT:'$$Compare(Value,X-1,Y)
 . QUIT:'$$Compare(Value,X+1,Y)
 . QUIT:'$$Compare(Value,X,Y-1)
 . QUIT:'$$Compare(Value,X,Y+1)
 . SET ^RISK(X,Y)=Value+1
 . SET ^RISK=^RISK+Value+1 
 QUIT
Compare(Value,X,Y)
 IF '$DATA(^AOC9(X,Y)) QUIT 1 
 QUIT (^AOC9(X,Y)>Value)
 ;
 QUIT
