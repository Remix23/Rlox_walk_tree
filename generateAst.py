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
        if baseName == "Expr": f.write("use std::hash::Hash;\n")
    
        defineEnum(f, baseName, [t.split(":")[0] for t in types])

        for t in types:
            structName, fields = [x.strip() for x in t.split(":")]

            defineStruct(f, baseName, structName, fields.split(","))

        defineVistorTrait(f, [t.split(":")[0] for t in types])
        defineAccept(f, baseName, [t.split(":")[0] for t in types])

        if baseName == "Expr":
            define_hash(f, baseName)
            define_partial_eq(f, baseName)
            define_eq(f, baseName)

        f.write("\n")

def defineEnum (fileHandler, enumName : str, cases : list[str]) :
    fileHandler.write(f"#[derive(Debug, Clone)]\n")
    fileHandler.write(f"pub enum {enumName.strip()} {{\n")
    for case in cases:
        case = case.strip()
        fileHandler.write(f"    {case} ({case}),\n")
    fileHandler.write("}\n")

def defineStruct (fileHandelr, base_class, structName, fields) :
    fileHandelr.write(f"#[derive(Debug, Clone)]\n")
    fileHandelr.write(f"pub struct {structName} {{\n")
    for field in fields:
        if field == "": continue
        print(field)
        type_of_field, name_of_field = [x.strip() for x in field.strip().split(" ")]
        
        if type_of_field == "Expr" : type_of_field = "Box<Expr>"
        if type_of_field == "Stmt" : type_of_field = "Box<Stmt>"
        
        ### construct struct fields
        fileHandelr.write(f"    pub {name_of_field} : {type_of_field},\n")
    
    # add uuid field
    if base_class == "Expr": fileHandelr.write(f"    pub uuid : usize\n")

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
    
    if forType == "Expr": 

        fileHander.write(f"    pub fn get_uuid(&self) -> usize {{\n")
        fileHander.write(f"        match self {{\n")
        for t in types:
            typeName = t.split(":")[0].strip()
            fileHander.write(f"            {forType}::{typeName} (e) => e.uuid,\n")
        fileHander.write("          }\n")
        fileHander.write("      }\n")
    fileHander.write("}\n")



def define_hash (fileHandler, forType : str) :
    fileHandler.write(f"impl Hash for {forType} {{\n")
    fileHandler.write(f"    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {{\n")
    fileHandler.write(f"        self.get_uuid().hash(state);\n")
    fileHandler.write(f"    }}\n")
    fileHandler.write(f"}}\n")

def define_partial_eq (fileHandler, forType : str) :
    fileHandler.write(f"impl PartialEq for {forType} {{\n")
    fileHandler.write(f"    fn eq(&self, other: &Self) -> bool {{\n")
    fileHandler.write(f"        self.get_uuid() == other.get_uuid()\n")
    fileHandler.write(f"    }}\n")
    fileHandler.write(f"}}\n")

def define_eq (fileHandler, forType : str) :
    fileHandler.write(f"impl Eq for {forType} {{\n")
    fileHandler.write(f"}}\n")

if __name__ == "__main__" :
    exprs = [
        "Binary   : Expr left, Token operator, Expr right",
        "Logical  : Expr left, Token operator, Expr right",
        "Call     : Expr callee, Token paren, Vec<Expr> arguments",
        "Get      : Expr object, Token name",
        "Set      : Expr object, Token name, Expr value",
        "This     : Token keyword", 
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
        "Function   : Token name, Vec<Token> params, Vec<Stmt> body",
        "Print      : Expr expression",
        "Var        : Token name, Option<Expr> initializer",
        "Block      : Vec<Stmt> statements",
        "Iff         : Expr condition, Box<Stmt> then_branch, Option<Box<Stmt>> else_branch",
        "Whilee     : Expr condition, Box<Stmt> body, bool is_for",
        "Breakk      : Token keyword",
        "Continuee   : Token keyword",
        "Returnn     : Token keyword, Option<Expr> value",
        "Class       : Token name, Option<Expr> SuperClass, Vec<Function> methods"

    ]
    if len(sys.argv) != 2:
        print("Usage: generateAst <output directory>")
        sys.exit(64)

    outputDir = sys.argv[1]
    defineAst(outputDir, "Stmt", smts)