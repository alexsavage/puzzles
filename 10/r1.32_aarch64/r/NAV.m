NAV
	n input,ln,errTotal,errScore,fixSort,fixScore
	s errTotal=0,fixSort=0
	w !,"Input puzzle data. End with blank line:",!
	f  r input q:input=""  d  
	. s errScore=0,fixScore=0
	. d score(input,.errScore,.fixScore)
	. s errTotal=errTotal+errScore
	. i fixScore>0 s fixSort(fixScore)="",fixSort=fixSort+1
	w !,"Error total: ",errTotal
	s fixScore=""
	f ln=0:1:(fixSort/2) s fixScore=$o(fixSort(fixScore))
	w !,"Median autocorrect score: ",fixScore
	q
score(input,errScore,fixScore)
	n pos,stack,level,char
	s level=0
	f pos=1:1:$l(input) d  q:errScore>0
	. s char=$e(input,pos)
	. i $$isOpener(char) s level=level+1,stack(level)=char
	. e  d  
	. . i char'=$$closeFor(stack(level)) s errScore=$$errScore(char) q
	. . s level=level-1
	q:errScore>0
	f  q:level<1  d 
	. s fixScore=(fixScore*5)+$$fixScore(stack(level))
	. s level=level-1
	q
isOpener(char) q "([{<"[char
closeFor(char) q $s(char="(":")",char="[":"]",char="{":"}",char="<":">")
errScore(char) q $s(char=")":3,char="]":57,char="}":1197,char=">":25137)
fixScore(char) q $s(char="(":1,char="[":2,char="{":3,char="<":4)