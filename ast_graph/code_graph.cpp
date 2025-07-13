#include <clang/AST/AST.h>
#include <clang/AST/ASTConsumer.h>
#include <clang/AST/RecursiveASTVisitor.h>
#include <clang/Frontend/CompilerInstance.h>
#include <clang/Frontend/FrontendActions.h>
#include <clang/Tooling/Tooling.h>
#include <clang/Tooling/CommonOptionsParser.h>
#include <boost/graph/adjacency_list.hpp>
#include <boost/graph/graphviz.hpp>
#include <unordered_map>
#include <string>
#include <vector>

// Node types in our code graph
enum class NodeType {
    FUNCTION,
    CLASS,
    VARIABLE,
    NAMESPACE,
    STRUCT,
    ENUM,
    TYPEDEF,
    TEMPLATE
};

// Edge types representing relationships
enum class EdgeType {
    CALLS,           // Function calls another function
    INHERITS,        // Class inherits from another
    CONTAINS,        // Class contains member
    USES_TYPE,       // Function uses a type
    DECLARES,        // Scope declares entity
    REFERENCES       // Variable references another
};

// Graph node properties
struct CodeNode {
    std::string name;
    NodeType type;
    std::string file_path;
    int line_number;
    std::string signature;  // For functions: return type + params
    std::string scope;      // Namespace/class scope
};

// Graph edge properties
struct CodeEdge {
    EdgeType type;
    std::string context;  // Additional info about the relationship
};

// Boost graph definition
using CodeGraph = boost::adjacency_list<
    boost::vecS,           // Edge container
    boost::vecS,           // Vertex container
    boost::directedS,      // Directed graph
    CodeNode,              // Vertex properties
    CodeEdge               // Edge properties
>;

using Vertex = boost::graph_traits<CodeGraph>::vertex_descriptor;
using Edge = boost::graph_traits<CodeGraph>::edge_descriptor;

class CodeGraphBuilder : public clang::RecursiveASTVisitor<CodeGraphBuilder> {
private:
    CodeGraph& graph;
    clang::ASTContext* context;
    std::unordered_map<std::string, Vertex> node_map;  // Name -> Vertex mapping
    std::string current_file;
    
    // Helper to get fully qualified name
    std::string getQualifiedName(const clang::NamedDecl* decl) {
        std::string name = decl->getQualifiedNameAsString();
        return name.empty() ? decl->getNameAsString() : name;
    }
    
    // Helper to get source location info
    std::pair<std::string, int> getLocationInfo(const clang::Decl* decl) {
        auto& sm = context->getSourceManager();
        auto loc = decl->getLocation();
        auto file_entry = sm.getFileEntryForID(sm.getFileID(loc));
        std::string file = file_entry ? file_entry->getName().str() : "unknown";
        int line = sm.getSpellingLineNumber(loc);
        return {file, line};
    }
    
    // Create or get existing node
    Vertex getOrCreateNode(const std::string& name, NodeType type, 
                          const std::string& file, int line,
                          const std::string& signature = "",
                          const std::string& scope = "") {
        auto it = node_map.find(name);
        if (it != node_map.end()) {
            return it->second;
        }
        
        Vertex v = boost::add_vertex(graph);
        graph[v] = {name, type, file, line, signature, scope};
        node_map[name] = v;
        return v;
    }
    
    // Add edge between nodes
    void addEdge(Vertex from, Vertex to, EdgeType edge_type, 
                const std::string& context = "") {
        auto [edge, inserted] = boost::add_edge(from, to, graph);
        if (inserted) {
            graph[edge] = {edge_type, context};
        }
    }

public:
    CodeGraphBuilder(CodeGraph& g, clang::ASTContext* ctx) 
        : graph(g), context(ctx) {}
    
    // Visit function declarations
    bool VisitFunctionDecl(clang::FunctionDecl* func) {
        if (!func->hasBody()) return true;  // Skip declarations without definitions
        
        auto [file, line] = getLocationInfo(func);
        std::string name = getQualifiedName(func);
        std::string signature = func->getType().getAsString();
        
        // Create function node
        Vertex func_vertex = getOrCreateNode(name, NodeType::FUNCTION, 
                                           file, line, signature);
        
        // Find function calls within this function
        findFunctionCalls(func, func_vertex);
        
        // Find variable references
        findVariableReferences(func, func_vertex);
        
        return true;
    }
    
    // Visit class declarations
    bool VisitCXXRecordDecl(clang::CXXRecordDecl* record) {
        if (!record->isCompleteDefinition()) return true;
        
        auto [file, line] = getLocationInfo(record);
        std::string name = getQualifiedName(record);
        
        NodeType type = record->isClass() ? NodeType::CLASS : NodeType::STRUCT;
        Vertex class_vertex = getOrCreateNode(name, type, file, line);
        
        // Handle inheritance
        for (const auto& base : record->bases()) {
            if (auto base_record = base.getType()->getAsCXXRecordDecl()) {
                std::string base_name = getQualifiedName(base_record);
                auto [base_file, base_line] = getLocationInfo(base_record);
                
                Vertex base_vertex = getOrCreateNode(base_name, NodeType::CLASS, 
                                                   base_file, base_line);
                addEdge(class_vertex, base_vertex, EdgeType::INHERITS);
            }
        }
        
        // Handle member variables and functions
        for (auto member : record->decls()) {
            if (auto field = clang::dyn_cast<clang::FieldDecl>(member)) {
                auto [member_file, member_line] = getLocationInfo(field);
                std::string member_name = name + "::" + field->getNameAsString();
                
                Vertex member_vertex = getOrCreateNode(member_name, NodeType::VARIABLE, 
                                                     member_file, member_line);
                addEdge(class_vertex, member_vertex, EdgeType::CONTAINS);
            }
        }
        
        return true;
    }
    
    // Visit variable declarations
    bool VisitVarDecl(clang::VarDecl* var) {
        auto [file, line] = getLocationInfo(var);
        std::string name = getQualifiedName(var);
        
        Vertex var_vertex = getOrCreateNode(name, NodeType::VARIABLE, file, line);
        
        // Connect to type if it's a user-defined type
        if (auto record_type = var->getType()->getAsCXXRecordDecl()) {
            std::string type_name = getQualifiedName(record_type);
            auto [type_file, type_line] = getLocationInfo(record_type);
            
            Vertex type_vertex = getOrCreateNode(type_name, NodeType::CLASS, 
                                               type_file, type_line);
            addEdge(var_vertex, type_vertex, EdgeType::USES_TYPE);
        }
        
        return true;
    }
    
    // Visit namespace declarations
    bool VisitNamespaceDecl(clang::NamespaceDecl* ns) {
        auto [file, line] = getLocationInfo(ns);
        std::string name = getQualifiedName(ns);
        
        getOrCreateNode(name, NodeType::NAMESPACE, file, line);
        return true;
    }
    
private:
    // Find function calls within a function body
    void findFunctionCalls(clang::FunctionDecl* func, Vertex func_vertex) {
        // This would require a more complex visitor to traverse the function body
        // and find CallExpr nodes, then link them to their corresponding function declarations
        // Implementation omitted for brevity
    }
    
    // Find variable references within a function
    void findVariableReferences(clang::FunctionDecl* func, Vertex func_vertex) {
        // Similar to function calls, this would traverse the function body
        // looking for DeclRefExpr nodes
        // Implementation omitted for brevity
    }
};

// AST Consumer that creates our graph builder
class CodeGraphConsumer : public clang::ASTConsumer {
private:
    CodeGraph& graph;
    std::unique_ptr<CodeGraphBuilder> builder;
    
public:
    CodeGraphConsumer(CodeGraph& g) : graph(g) {}
    
    void HandleTranslationUnit(clang::ASTContext& context) override {
        builder = std::make_unique<CodeGraphBuilder>(graph, &context);
        builder->TraverseDecl(context.getTranslationUnitDecl());
    }
};

// Frontend action to process files
class CodeGraphAction : public clang::ASTFrontendAction {
private:
    CodeGraph& graph;
    
public:
    CodeGraphAction(CodeGraph& g) : graph(g) {}
    
    std::unique_ptr<clang::ASTConsumer> CreateASTConsumer(
        clang::CompilerInstance&, clang::StringRef) override {
        return std::make_unique<CodeGraphConsumer>(graph);
    }
};

// Utility function to export graph to GraphViz format
void exportToGraphViz(const CodeGraph& graph, const std::string& filename) {
    std::ofstream file(filename);
    
    // Custom vertex writer
    auto vertex_writer = [&](std::ostream& out, Vertex v) {
        const auto& node = graph[v];
        std::string type_str;
        switch (node.type) {
            case NodeType::FUNCTION: type_str = "FUNC"; break;
            case NodeType::CLASS: type_str = "CLASS"; break;
            case NodeType::VARIABLE: type_str = "VAR"; break;
            case NodeType::NAMESPACE: type_str = "NS"; break;
            case NodeType::STRUCT: type_str = "STRUCT"; break;
            default: type_str = "OTHER"; break;
        }
        
        out << "[label=\"" << node.name << "\\n" << type_str << "\\n"
            << node.file_path << ":" << node.line_number << "\"]";
    };
    
    // Custom edge writer
    auto edge_writer = [&](std::ostream& out, Edge e) {
        const auto& edge = graph[e];
        std::string edge_str;
        switch (edge.type) {
            case EdgeType::CALLS: edge_str = "calls"; break;
            case EdgeType::INHERITS: edge_str = "inherits"; break;
            case EdgeType::CONTAINS: edge_str = "contains"; break;
            case EdgeType::USES_TYPE: edge_str = "uses"; break;
            default: edge_str = "relates"; break;
        }
        
        out << "[label=\"" << edge_str << "\"]";
    };
    
    boost::write_graphviz(file, graph, vertex_writer, edge_writer);
}

// Example usage class for the factory
class CodeGraphActionFactory : public clang::tooling::FrontendActionFactory {
private:
    CodeGraph& graph;
    
public:
    CodeGraphActionFactory(CodeGraph& g) : graph(g) {}
    
    std::unique_ptr<clang::FrontendAction> create() override {
        return std::make_unique<CodeGraphAction>(graph);
    }
};

// Main function to demonstrate usage
int main(int argc, const char** argv) {
    // Parse command line arguments
    auto ExpectedParser = clang::tooling::CommonOptionsParser::create(
        argc, argv, llvm::cl::GeneralCategory);
    
    if (!ExpectedParser) {
        llvm::errs() << ExpectedParser.takeError();
        return 1;
    }
    
    clang::tooling::CommonOptionsParser& OptionsParser = ExpectedParser.get();
    clang::tooling::ClangTool Tool(OptionsParser.getCompilations(),
                                   OptionsParser.getSourcePathList());
    
    // Create the code graph
    CodeGraph graph;
    
    // Create factory and run the tool
    auto factory = std::make_unique<CodeGraphActionFactory>(graph);
    int result = Tool.run(factory.get());
    
    // Export results
    exportToGraphViz(graph, "code_graph.dot");
    
    // Print some statistics
    std::cout << "Graph built successfully!\n";
    std::cout << "Nodes: " << boost::num_vertices(graph) << "\n";
    std::cout << "Edges: " << boost::num_edges(graph) << "\n";
    std::cout << "Exported to code_graph.dot\n";
    
    return result;
}
