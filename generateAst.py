import sys
from os import path, remove

"""
types:
[type of expr] : [list of fields]
"""

def defineAst(outputDir, baseName : str, types : list[str]):
    p = outputDir + "/" + baseName + ".rs"

    if path.exists(p):
        remove(p)

    print(p)

    with open(p, "x") as f:
        f.write("use crate::scanner::{Token, LiteralType};\n")
    
        defineEnum(f, baseName, [t.split(":")[0] for t in types])

        for t in types:
            structName, fields = [x.strip() for x in t.split(":")]

            defineStruct(f, structName, fields.split(","))

        defineVistorTrait(f, [t.split(":")[0] for t in types])
        defineAccept(f, baseName, [t.split(":")[0] for t in types])

        f.write("\n")

def defineEnum (fileHandler, enumName : str, cases : list[str]) :
    fileHandler.write(f"pub enum {enumName.strip()} {{\n")
    for case in cases:
        case = case.strip()
        fileHandler.write(f"    {case} ({case}),\n")
    fileHandler.write("}\n")

def defineStruct (fileHandelr, structName, fields) :
    fileHandelr.write(f"pub struct {structName} {{\n")
    for field in fields:
        print(field)
        type_of_field, name_of_field = [x.strip() for x in field.strip().split(" ")]
        
        if type_of_field == "Expr" : type_of_field = "Box<Expr>"
        
        ### construct struct fields
        fileHandelr.write(f"    pub {name_of_field} : {type_of_field},\n")

    fileHandelr.write("}\n")

def defineVistorTrait (fileHandler, types):
    fileHandler.write("pub trait Visitor<T> {\n")
    for t in types:
        typeName = t.split(":")[0].strip()
        fileHandler.write(f"    fn visit_{typeName.lower()}(&mut self, {typeName.lower()} : &{typeName}) -> T;\n")
    fileHandler.write("}\n")

def defineAccept (fileHander, forType : str, types : list[str]) :
    fileHander.write(f"impl {forType} {{\n")
    
    fileHander.write(f"    pub fn accept<T>(&self, visitor : &mut dyn Visitor<T>) -> T {{\n")
    fileHander.write(f"        match self {{\n")
    for t in types:
        typeName = t.split(":")[0].strip()
        fileHander.write(f"            {forType}::{typeName} ({typeName.lower()}) => visitor.visit_{typeName.lower()}({typeName.lower()}),\n")
    fileHander.write("          }\n")
    fileHander.write("      }\n")
    fileHander.write("}\n")

if __name__ == "__main__" :
    exprs = [
        "Binary   : Expr left, Token operator, Expr right",
        "Grouping : Expr expression",
        "Literal  : Object value",
        "Unary    : Token operator, Expr right"
    ]
    if len(sys.argv) != 2:
        print("Usage: generateAst <output directory>")
        sys.exit(64)

    outputDir = sys.argv[1]
    defineAst(outputDir, "Expr", exprs)