## Predictive Parsing Table

Once the grammar was shown to be LL(1), the grammar was input into the a [parse table generator tool](http://jsmachines.sourceforge.net/machines/ll1.html). The fact that there was at most one production in each table cell also indicates that the grammar is indeed LL(1).

![Large image of generated parse table using jsmachines tool](assets/images/generated-parse-table)


```
S  -> A B program C ;
A  -> D A | EPSILON
B  -> N B | EPSILON
C  -> { CI
CI -> } | OR | FO | CH
CH -> id CX CH
CX -> CJ CY
CY -> CU | EPSILON
CJ -> AG . id CJ CU | AJX CJ CU | CU | id Q ;
CU -> AU = U
FO -> float id Q ; FO | int id Q ; FO
D  -> class id E { DR
DR -> J id DY | }
DY ->  FG ; | G ;
FG -> Q ; FG
E  -> : id H | EPSILON
G  -> ( K ) ; G
H -> , id H | EPSILON
J  -> float | id | int
K  -> J id Q AO | EPSILON
L  -> J M ( K )
M  -> id MM
MM -> :: | EPSILON
N  -> L C ;
OR -> R OR
O  -> R O | T O | }
Q  -> AN Q | EPSILON
R  -> for ( J id = U ; X ; T ) V ;
    | if ( U ) then V ;
    | get ( id RQC ;
    | put ( U ) ;
    | return ( U ) ;
T  -> id RQE U
U  -> Z AW
V  -> { O | R | T | EPSILON
X  -> Z AA Z
AA -> < | <= | <> | == | > | >=
AB -> + | - | or | '||'
AE -> && | * | / | and
AC -> ( Z ) RW | floatNum RW | intNum RW | + AC RW | - AC RW | ! AC RW | not AC RW | id RQ
RQ -> AZ RE
RQC -> AZ RE )
RQE -> AZ RE =
RE -> RW | EPSILON
RW -> AE AC
Z -> AC ACR Z
ACR -> AB | EPSILON
AG -> ( AK )
AJX -> AM AJX
AK -> U AP | EPSILON
AM -> [ Z ]
AN -> [ intNum ]
AO -> AQ AO | EPSILON
AP -> AR AP | EPSILON
AQ -> , J id Q
AR -> , U
AW -> AA Z | EPSILON
AZ -> AG . id RQ | AJX RQ | EPSILON
```


