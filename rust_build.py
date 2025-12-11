#!/usr/bin/env python3
# Helper script to make it easy to run standalone rust scripts with basic dependencies without having to set up a crate.

import sys
import os
import tempfile
import shutil
import subprocess
import re
from pathlib import Path

def extract_dependencies(rust_file):
    """Extract dependencies from // DEPS: comments at the top of the file."""
    deps = []
    
    with open(rust_file, 'r') as f:
        for line in f:
            # Strip whitespace
            line = line.strip()
            
            # Check for DEPS comment
            match = re.match(r'^//\s*DEPS:\s*(.+)$', line)
            if match:
                deps.append(match.group(1))
            
            # Stop at first non-comment, non-empty line
            if line and not line.startswith('//') and not line.startswith('#'):
                break
    
    return deps

def create_cargo_toml(package_name, dependencies):
    """Generate Cargo.toml content."""
    toml = f"""[package]
name = "{package_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
"""
    
    for dep in dependencies:
        toml += f"{dep}\n"
    
    return toml

def build_rust_file(rust_file):
    """Build a Rust file with dependencies in a temporary Cargo project."""
    
    # Validate input file
    rust_path = Path(rust_file)
    if not rust_path.exists():
        print(f"Error: File '{rust_file}' not found", file=sys.stderr)
        return False
    
    # Extract package name
    package_name = rust_path.stem
    
    print(f"Building '{package_name}' from {rust_file}")
    
    # Extract dependencies
    deps = extract_dependencies(rust_file)
    
    if deps:
        print("Found dependencies:")
        for dep in deps:
            print(f"  {dep}")
    else:
        print("No dependencies found")
    
    # Create temporary directory
    with tempfile.TemporaryDirectory(prefix='cargo_build_') as temp_dir:
        temp_path = Path(temp_dir)
        print(f"Building in: {temp_dir}")
        
        # Create Cargo.toml
        cargo_toml = create_cargo_toml(package_name, deps)
        (temp_path / 'Cargo.toml').write_text(cargo_toml)
        
        # Create src directory and copy Rust file
        src_dir = temp_path / 'src'
        src_dir.mkdir()
        shutil.copy2(rust_file, src_dir / 'main.rs')
        
        # Build with cargo
        print("Running cargo build --release...")
        try:
            result = subprocess.run(
                ['cargo', 'build', '--release'],
                cwd=temp_dir,
                capture_output=True,
                text=True,
                check=True
            )
            
            if result.stdout:
                print(result.stdout)
            
        except subprocess.CalledProcessError as e:
            print(f"Build failed!", file=sys.stderr)
            print(e.stderr, file=sys.stderr)
            return False
        
        # Copy executable to current directory
        executable_src = temp_path / 'target' / 'release' / package_name
        executable_dst = Path.cwd() / package_name
        
        if not executable_src.exists():
            print(f"Error: Executable not found at {executable_src}", file=sys.stderr)
            return False
        
        shutil.copy2(executable_src, executable_dst)
        
        # Make executable on Unix systems
        os.chmod(executable_dst, 0o755)
        
        # Get file size
        size = executable_dst.stat().st_size
        size_mb = size / (1024 * 1024)
        
        print(f"\n✓ Success! Executable created: {executable_dst}")
        print(f"  Size: {size_mb:.2f} MB")
        
        return True

def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <rust_file.rs>")
        sys.exit(1)
    
    rust_file = sys.argv[1]
    
    success = build_rust_file(rust_file)
    sys.exit(0 if success else 1)

if __name__ == '__main__':
    main()
