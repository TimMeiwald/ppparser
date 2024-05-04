string = """=
>   
<   
!   
~   
?   
:
==  
<=  
>=  
!=  
&&  
||  
++  
--
+   
-   
*   
/   
&   
|
^   
%   
<<   
>>   
>>>
+=  
-=  
*=  
/=  
&=  
|=  
^=  
%=  
<<=  
>>=  
>>>="""



total = ""
for line in string.splitlines():
    res = ""
    res += "("
    for char in line:
        if char in [" ", "\n", "\t", "\r"]:
            continue
        res += f'"{char}", '
    res = res[:-2]
    res += ")"
    total += res + "/"
    
keyword = "<Operator> = " + total[:-1] + ";"
print(keyword)
