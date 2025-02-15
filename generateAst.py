import sys
from os import path, remove

"""
types:
[type of expr] : [list of fields]
"""

def defineAst(outputDir, baseName : str, types : list[str]):
    p = outputDir + "/" + baseName.lower() + ".rs"

    if path.exists(p):
        remove(p)

    print(p)

    with open(p, "x") as f:
        if baseName == "Stmt": f.write("use crate::expr::Expr;\n")
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
        if field == "": continue
        print(field)
        type_of_field, name_of_field = [x.strip() for x in field.strip().split(" ")]
        
        if type_of_field == "Expr" : type_of_field = "Box<Expr>"
        if type_of_field == "Stmt" : type_of_field = "Box<Stmt>"
        
        ### construct struct fields
        fileHandelr.write(f"    pub {name_of_field} : {type_of_field},\n")

    fileHandelr.write("}\n")

def defineVistorTrait (fileHandler, types):
    fileHandler.write(f"pub trait Visitor<T> {{\n")
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
        "Logical  : Expr left, Token operator, Expr right",
        "Call     : Expr callee, Token paren, Vec<Expr> arguments",
        "Grouping : Expr expression",
        "Literal  : LiteralType value",
        "Unary    : Token operator, Expr right",
        "Conditional : Expr condition, Expr then_branch, Expr else_branch",
        "Variable : Token name",
        "Assigment : Token name, Expr value",
    ]

    # the token in the function call is used to report optional runtaime errors

    smts = [
        "Expression : Expr expression",
        "Print      : Expr expression",
        "Var        : Token name, Option<Expr> initializer",
        "Block      : Vec<Stmt> statements",
        "Iff         : Expr condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch",
        "Whilee     : Expr condition, Box<Stmt> body, bool is_for",
        "Breakk      : ",
        "Continuee   : ",

    ]
    if len(sys.argv) != 2:
        print("Usage: generateAst <output directory>")
        sys.exit(64)

    outputDir = sys.argv[1]
    defineAst(outputDir, "Stmt", smts)