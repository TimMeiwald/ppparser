string = """abstract
continue   
for          
new         
switch
assert     
default    
if           
package     
synchronized
boolean
do         
goto         
private     
this
break      
double     
implements   
protected   
throw
byte       
else       
import       
public      
throws
case       
enum       
instanceof   
return      
transient
catch      
extends    
int          
short       
try
char       
final      
interface    
static      
void
class     
finally
long
strictfp
volatile
const
float
native
super
while"""

for line in string.splitlines():
    line = line.strip()
    out = "<" + line+ "> = "
    for char in line:
        out += (f'"{char}",')
    out = out[:-1] + ";"
    print(out)


keyword = "<Keyword> = "
for line in string.splitlines():
    line = "<" + line.strip() + ">"
    keyword += line + "/"
keyword = keyword[:-1]
keyword += ";"
print(keyword)
