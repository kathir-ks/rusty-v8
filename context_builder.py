import os
from typing import Dict, List, Optional
from pathlib import Path
from datetime import datetime
import google.generativeai as genai
from model_context import (
    SystemContext, CodebaseMetadata, GlobalContext,
    LocalContext, ChangeRequest, ModelContext
)
import json

class ContextBuilder:
    def __init__(self, api_key: str, root_dir: str):
        self.root_dir = Path(root_dir)
        self.chat_session = None
        self.model = None
        self._initialize_model(api_key)
    
    def _initialize_model(self, api_key: str) -> None:
        """Initialize Gemini model and create chat session"""
        try:
            genai.configure(api_key=api_key)
            self.model = genai.GenerativeModel('gemini-1.5-flash')
            self.chat_session = self.model.start_chat(
                history=[{
                    "role": "model",
                    "parts": ["Hello, I'm here to help with your questions."]
                }]
            )
        except Exception as e:
            raise RuntimeError(f"Failed to initialize Gemini model: {str(e)}")
    
    def send_message(self, message: str) -> str:
        """Send message to chat session and get response"""
        try:
            response = self.chat_session.send_message(message)
            return response.text
        except Exception as e:
            raise RuntimeError(f"Error sending message to LLM: {str(e)}")
    
    def analyze_directory_structure(self) -> Dict:
        """Analyze project structure and get initial metadata"""
        files = []
        for path in self.root_dir.rglob('*'):
            if path.is_file() and not any(part.startswith('.') for part in path.parts):
                files.append(str(path.relative_to(self.root_dir)))
        
        prompt = f"""
        Analyze this project structure and provide:
        1. Primary programming language
        2. Architecture pattern used
        3. Key components and their purposes
        4. Main dependencies
        
        Project files:
        {files}
        
        Return the response as a JSON object.
        """
        
        response = self.send_message(prompt)
        # Convert the string response to a dictionary
        try:
            return json.loads(response)
        except json.JSONDecodeError:
            # Fallback with empty dictionary if parsing fails
            return {
                'primary_language': '',
                'architecture_pattern': '',
                'key_components': [],
                'dependencies': [],
                'design_overview': {},
                'tech_stack': {},
                'architecture_details': {},
                'design_patterns': [],
                'shared_components': []
            }
    
    def analyze_file_content(self, file_path: str) -> Dict:
        """Analyze specific file content"""
        try:
            full_path = self.root_dir / file_path
            content = full_path.read_text()
            
            prompt = f"""
            Analyze this file and provide:
            1. Key functions and their purposes
            2. Important variables and their scopes
            3. Related code context needed for understanding
            
            File content:
            {content}
            
            Return the response as a JSON object.
            """
            
            return self.send_message(prompt)
        except Exception as e:
            raise RuntimeError(f"Error analyzing file {file_path}: {str(e)}")
    
    def build_complete_context(self) -> ModelContext:
        """Build complete context for the model"""
        try:
            # Analyze project structure
            project_analysis = self.analyze_directory_structure()
            
            # Build SystemContext
            system_context = SystemContext(
                system_prompt="You are a code-aware AI assistant...",
                model_capabilities=[
                    "code analysis",
                    "design pattern recognition",
                    "code generation"
                ],
                behavior_guidelines=[
                    "Provide detailed explanations",
                    "Follow coding best practices"
                ],
                response_format={"type": "markdown"}
            )
            
            # Build CodebaseMetadata with design information
            codebase_metadata = CodebaseMetadata(
                repository_name=self.root_dir.name,
                primary_language=project_analysis.get('primary_language', ''),
                file_path=str(self.root_dir),
                file_type="directory",
                last_modified=datetime.now().isoformat(),
                dependencies=project_analysis.get('dependencies', []),
                architecture_pattern=project_analysis.get('architecture_pattern', ''),
                design_overview=project_analysis.get('design_overview', {}),
                key_components=project_analysis.get('key_components', []),
                tech_stack=project_analysis.get('tech_stack', {})
            )
            
            # Build other contexts...
            global_context = GlobalContext(
                project_architecture=project_analysis.get('architecture_details', {}),
                design_patterns=project_analysis.get('design_patterns', []),
                shared_components=project_analysis.get('shared_components', []),
                global_variables={}
            )
            
            local_context = LocalContext(
                current_file_content="",
                surrounding_code={},
                variable_scope={},
                function_definitions=[]
            )
            
            change_request = ChangeRequest(
                request_type="analysis",
                description="Initial context building",
                affected_files=[],
                proposed_changes={},
                acceptance_criteria=[]
            )
            
            return ModelContext(
                system=system_context,
                codebase=codebase_metadata,
                global_context=global_context,
                local_context=local_context,
                change_request=change_request
            )
            
        except Exception as e:
            raise RuntimeError(f"Error building context: {str(e)}")

    def update_local_context(self, file_path: str) -> None:
        """Update local context for a specific file"""
        full_path = self.root_dir / file_path
        if not full_path.exists():
            raise FileNotFoundError(f"File {file_path} not found")
            
        file_content = full_path.read_text()
        
        # Ask Gemini to analyze the file
        prompt = f"""
        Analyze this file and provide:
        1. Key functions and their purposes
        2. Important variables and their scopes
        3. Related code context needed for understanding
        
        File content:
        {file_content}
        """
        
        analysis = self.model.generate_content(prompt)
        return analysis.text

def main():
    api_key = "AIzaSyAhjHmTzJb8wDy-ty6FyLj-qSU3Tj58JeQ"
    root_dir = "/home/kathirks_gc/v8/v8/include/cppgc/internal"
    
    builder = ContextBuilder(api_key, root_dir)
    context = builder.build_complete_context()
    
    # Example: Analyze a specific file
    file_analysis = builder.update_local_context("/home/kathirks_gc/v8/v8/include/cppgc/internal/caged-heap.h")
    
    # Use the context with Gemini
    prompt = f"Analyze the current codebase architecture\n\nContext: {context.to_prompt()}"
    response = builder.model.generate_content(prompt)
    print(response.text)

if __name__ == "__main__":
    main() 