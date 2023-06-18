Code in src is also used as the base for creating the parsers since they're used internally as well as in the parsers.  

They get split by the first newline to remove imports and the first "#[cfg(test)]", so as long as all the needed code is after there's an empty newline and before "#[cfg(test)].  
Brittle but short of writing a Rust parser this is the easiest method to maitain coherence between the generated parsers and internal parsing code.   
  
TODO: Cache for Packrat  
TODO: Left Recursive Supporting Cache  
TODO: Generate the full parser for the generator  
TODO: Automatically detect Left recursion.  